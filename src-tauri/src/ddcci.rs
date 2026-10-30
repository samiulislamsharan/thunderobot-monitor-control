use std::ptr;
use std::sync::Mutex;
use std::thread;
use std::time::{Duration, Instant};

type BOOL = i32;
type HANDLE = *mut std::ffi::c_void;
type HMONITOR = *mut std::ffi::c_void;

#[repr(C)]
#[derive(Debug)]
struct PHYSICAL_MONITOR {
    h_physical_monitor: HANDLE,
    sz_physical_monitor_description: [u16; 128],
}

#[link(name = "user32")]
extern "system" {
    fn MonitorFromWindow(hwnd: HANDLE, dw_flags: u32) -> HMONITOR;
}

#[link(name = "dxva2")]
extern "system" {
    fn GetNumberOfPhysicalMonitorsFromHMONITOR(
        h_monitor: HMONITOR,
        pdw_number_of_physical_monitors: *mut u32,
    ) -> BOOL;

    fn GetPhysicalMonitorsFromHMONITOR(
        h_monitor: HMONITOR,
        dw_physical_monitor_array_size: u32,
        p_physical_monitor_array: *mut PHYSICAL_MONITOR,
    ) -> BOOL;

    fn DestroyPhysicalMonitors(
        dw_physical_monitor_array_size: u32,
        p_physical_monitor_array: *mut PHYSICAL_MONITOR,
    ) -> BOOL;

    fn GetVCPFeatureAndVCPFeatureReply(
        h_monitor: HANDLE,
        b_vcp_code: u8,
        pvct: *mut u32,
        pdw_current_value: *mut u32,
        pdw_maximum_value: *mut u32,
    ) -> BOOL;

    fn SetVCPFeature(h_monitor: HANDLE, b_vcp_code: u8, dw_new_value: u32) -> BOOL;
}

pub struct DdcciController {
    last_write: Mutex<Instant>,
    delay_ms: Mutex<u64>,
}

impl DdcciController {
    pub fn new(delay_ms: u64) -> Self {
        Self {
            last_write: Mutex::new(Instant::now()),
            delay_ms: Mutex::new(delay_ms),
        }
    }

    pub fn set_delay(&self, delay_ms: u64) {
        if let Ok(mut d) = self.delay_ms.lock() {
            *d = delay_ms;
        }
    }

    fn rate_limit(&self) {
        if let Ok(mut last) = self.last_write.lock() {
            let delay = self.delay_ms.lock().map(|d| *d).unwrap_or(35);
            let elapsed = last.elapsed();
            let req_duration = Duration::from_millis(delay);
            if elapsed < req_duration {
                thread::sleep(req_duration - elapsed);
            }
            *last = Instant::now();
        }
    }

    pub fn get_vcp(&self, code: u8) -> Result<(u32, u32), String> {
        self.rate_limit();
        unsafe {
            let h_mon = MonitorFromWindow(ptr::null_mut(), 1); // MONITOR_DEFAULTTOPRIMARY
            if h_mon.is_null() {
                return Err("Failed to obtain primary HMONITOR".into());
            }

            let mut count = 0u32;
            if GetNumberOfPhysicalMonitorsFromHMONITOR(h_mon, &mut count) == 0 || count == 0 {
                return Err("No physical monitors detected on primary display".into());
            }

            let mut phys_monitors: Vec<PHYSICAL_MONITOR> = (0..count)
                .map(|_| PHYSICAL_MONITOR {
                    h_physical_monitor: ptr::null_mut(),
                    sz_physical_monitor_description: [0; 128],
                })
                .collect();

            if GetPhysicalMonitorsFromHMONITOR(h_mon, count, phys_monitors.as_mut_ptr()) == 0 {
                return Err("Failed to query physical monitor handles".into());
            }

            let h_phys = phys_monitors[0].h_physical_monitor;
            let mut vct = 0u32;
            let mut cur_val = 0u32;
            let mut max_val = 0u32;

            let ok = GetVCPFeatureAndVCPFeatureReply(h_phys, code, &mut vct, &mut cur_val, &mut max_val);
            DestroyPhysicalMonitors(count, phys_monitors.as_mut_ptr());

            if ok != 0 {
                Ok((cur_val, max_val))
            } else {
                Err(format!("DDC/CI VCP read failed for 0x{:02X}", code))
            }
        }
    }

    pub fn set_vcp(&self, code: u8, value: u32) -> Result<(), String> {
        self.rate_limit();
        unsafe {
            let h_mon = MonitorFromWindow(ptr::null_mut(), 1);
            if h_mon.is_null() {
                return Err("Failed to obtain primary HMONITOR".into());
            }

            let mut count = 0u32;
            if GetNumberOfPhysicalMonitorsFromHMONITOR(h_mon, &mut count) == 0 || count == 0 {
                return Err("No physical monitors detected".into());
            }

            let mut phys_monitors: Vec<PHYSICAL_MONITOR> = (0..count)
                .map(|_| PHYSICAL_MONITOR {
                    h_physical_monitor: ptr::null_mut(),
                    sz_physical_monitor_description: [0; 128],
                })
                .collect();

            if GetPhysicalMonitorsFromHMONITOR(h_mon, count, phys_monitors.as_mut_ptr()) == 0 {
                return Err("Failed to get physical monitor handles".into());
            }

            let h_phys = phys_monitors[0].h_physical_monitor;
            let ok = SetVCPFeature(h_phys, code, value);
            DestroyPhysicalMonitors(count, phys_monitors.as_mut_ptr());

            if ok != 0 {
                Ok(())
            } else {
                Err(format!("DDC/CI VCP write failed for 0x{:02X}={}", code, value))
            }
        }
    }
}
