import ctypes
from ctypes import wintypes
import threading
import time
from typing import Dict, List, Optional, Tuple, Any

user32 = ctypes.windll.user32
dxva2 = ctypes.windll.dxva2


class PHYSICAL_MONITOR(ctypes.Structure):
    _fields_ = [
        ("hPhysicalMonitor", wintypes.HANDLE),
        ("szPhysicalMonitorDescription", wintypes.WCHAR * 128),
    ]


# Standard and ThundeRobot DQ34C240L (MStar) VCP Codes
VCP_DEFINITIONS = {
    0x10: {
        "name": "brightness",
        "label": "Brightness",
        "description": "Backlight luminance level",
        "type": "continuous",
        "min": 0,
        "max": 100,
        "category": "picture",
    },
    0x12: {
        "name": "contrast",
        "label": "Contrast",
        "description": "Digital contrast ratio",
        "type": "continuous",
        "min": 0,
        "max": 100,
        "category": "picture",
    },
    0x14: {
        "name": "color_preset",
        "label": "Color Temperature",
        "description": "Preset color temperature or user RGB mode",
        "type": "discrete",
        "values": {
            5: "6500K (Warm)",
            6: "7500K (Normal)",
            8: "9300K (Cool)",
            11: "User / Custom",
        },
        "category": "color",
    },
    0x16: {
        "name": "red_gain",
        "label": "Red Gain",
        "description": "Red channel color gain",
        "type": "continuous",
        "min": 0,
        "max": 100,
        "category": "color",
    },
    0x18: {
        "name": "green_gain",
        "label": "Green Gain",
        "description": "Green channel color gain",
        "type": "continuous",
        "min": 0,
        "max": 100,
        "category": "color",
    },
    0x1A: {
        "name": "blue_gain",
        "label": "Blue Gain",
        "description": "Blue channel color gain",
        "type": "continuous",
        "min": 0,
        "max": 100,
        "category": "color",
    },
    0x6C: {
        "name": "red_black_level",
        "label": "Red Black Level",
        "description": "Red video black level offset",
        "type": "continuous",
        "min": 0,
        "max": 255,
        "category": "color",
    },
    0x6E: {
        "name": "green_black_level",
        "label": "Green Black Level",
        "description": "Green video black level offset",
        "type": "continuous",
        "min": 0,
        "max": 255,
        "category": "color",
    },
    0x70: {
        "name": "blue_black_level",
        "label": "Blue Black Level",
        "description": "Blue video black level offset",
        "type": "continuous",
        "min": 0,
        "max": 255,
        "category": "color",
    },
    0xDC: {
        "name": "picture_mode",
        "label": "Picture Mode",
        "description": "Factory calibrated display mode",
        "type": "discrete",
        "values": {0: "Standard", 2: "Movie", 3: "Game", 5: "FPS", 7: "RTS"},
        "category": "picture",
    },
    0x60: {
        "name": "input_source",
        "label": "Input Source",
        "description": "Active video input connector",
        "type": "discrete",
        "values": {
            15: "DisplayPort 1 (Primary)",
            16: "DisplayPort 2",
            17: "HDMI 1",
            18: "HDMI 2",
        },
        "category": "input",
    },
    0xAA: {
        "name": "orientation",
        "label": "Screen Orientation",
        "description": "Display orientation",
        "type": "discrete",
        "values": {1: "Landscape (0°)", 2: "Portrait (90°)"},
        "category": "system",
    },
    0x3E: {
        "name": "gaming_sharpness",
        "label": "Sharpness / Black EQ",
        "description": "Gaming picture enhancement / sharpness",
        "type": "continuous",
        "min": 0,
        "max": 100,
        "category": "gaming",
    },
    0x68: {
        "name": "overdrive",
        "label": "Overdrive Level",
        "description": "Pixel response time acceleration",
        "type": "discrete",
        "values": {
            0: "Off",
            1: "Level 1",
            2: "Level 2",
            3: "Level 3",
            4: "Level 4",
            5: "Level 5",
        },
        "category": "gaming",
    },
    0xE3: {
        "name": "mprt_aim",
        "label": "Motion Blur Reduction / MPRT",
        "description": "1ms MPRT strobe / Aim Assist toggle",
        "type": "discrete",
        "values": {0: "Disabled", 1: "Enabled"},
        "category": "gaming",
    },
    0xD6: {
        "name": "power_mode",
        "label": "Power State",
        "description": "Display power management",
        "type": "discrete",
        "values": {1: "Power On", 4: "Standby / Suspend", 5: "Power Off"},
        "category": "system",
    },
    0xAC: {
        "name": "h_freq",
        "label": "Horizontal Frequency",
        "description": "Line scan frequency in kHz",
        "type": "telemetry",
        "category": "telemetry",
    },
    0xAE: {
        "name": "v_freq",
        "label": "Vertical Refresh Rate",
        "description": "Active refresh rate in Hz",
        "type": "telemetry",
        "category": "telemetry",
    },
    0xC0: {
        "name": "usage_hours",
        "label": "Power-On Hours",
        "description": "Cumulative operating hours",
        "type": "telemetry",
        "category": "telemetry",
    },
    0x05: {
        "name": "reset_luminance",
        "label": "Reset Luminance/Contrast",
        "description": "Reset brightness and contrast to defaults",
        "type": "action",
        "category": "reset",
    },
    0x08: {
        "name": "reset_color",
        "label": "Reset Color Defaults",
        "description": "Reset RGB gains and temperature to defaults",
        "type": "action",
        "category": "reset",
    },
    0x04: {
        "name": "reset_factory",
        "label": "Factory Reset",
        "description": "Restore all monitor settings to factory default",
        "type": "action",
        "category": "reset",
    },
}


class MonitorHandleContext:
    """Thread-safe context manager that opens a physical monitor handle and automatically closes it."""

    def __init__(self, monitor_index: int = 0, lock: Optional[threading.Lock] = None):
        self.monitor_index = monitor_index
        self.lock = lock
        self.handle = None

    def __enter__(self):
        if self.lock:
            self.lock.acquire()

        # Enumerate HMONITORs
        hmonitors = []

        def enum_proc(hMonitor, hdcMonitor, lprcMonitor, dwData):
            hmonitors.append(hMonitor)
            return True

        MONITORENUMPROC = ctypes.WINFUNCTYPE(
            wintypes.BOOL,
            wintypes.HMONITOR,
            wintypes.HDC,
            ctypes.POINTER(wintypes.RECT),
            wintypes.LPARAM,
        )
        user32.EnumDisplayMonitors(None, None, MONITORENUMPROC(enum_proc), 0)

        if not hmonitors:
            raise RuntimeError("No display monitors found")

        # Pick target HMONITOR
        target_idx = min(self.monitor_index, len(hmonitors) - 1)
        hmon = hmonitors[target_idx]

        count = wintypes.DWORD()
        if (
            not dxva2.GetNumberOfPhysicalMonitorsFromHMONITOR(hmon, ctypes.byref(count))
            or count.value == 0
        ):
            raise RuntimeError(
                f"Failed to get physical monitor count for HMONITOR {target_idx}"
            )

        phy_mons = (PHYSICAL_MONITOR * count.value)()
        if not dxva2.GetPhysicalMonitorsFromHMONITOR(hmon, count.value, phy_mons):
            raise RuntimeError(
                f"Failed to get physical monitor handles for HMONITOR {target_idx}"
            )

        self.handle = phy_mons[0].hPhysicalMonitor
        self.description = phy_mons[0].szPhysicalMonitorDescription
        return self.handle

    def __exit__(self, exc_type, exc_val, exc_tb):
        if self.handle:
            try:
                dxva2.DestroyPhysicalMonitor(self.handle)
            except Exception:
                pass
            self.handle = None
        # Enforce rate-limiting delay between DDC/CI I2C commands (35ms)
        time.sleep(0.035)
        if self.lock:
            self.lock.release()


class DDCCIController:
    """High-level thread-safe DDC/CI monitor controller."""

    def __init__(self, monitor_index: int = 0):
        self.monitor_index = monitor_index
        self._lock = threading.Lock()

    def get_vcp(self, code: int) -> Dict[str, Any]:
        """Read a single VCP feature by code."""
        with MonitorHandleContext(self.monitor_index, self._lock) as handle:
            pvct = wintypes.DWORD()
            cur_val = wintypes.DWORD()
            max_val = wintypes.DWORD()
            res = dxva2.GetVCPFeatureAndVCPFeatureReply(
                handle,
                code,
                ctypes.byref(pvct),
                ctypes.byref(cur_val),
                ctypes.byref(max_val),
            )
            if not res:
                err = ctypes.GetLastError()
                return {
                    "code": code,
                    "hex_code": f"0x{code:02X}",
                    "supported": False,
                    "error": err,
                }

            definition = VCP_DEFINITIONS.get(code, {})
            return {
                "code": code,
                "hex_code": f"0x{code:02X}",
                "name": definition.get("name", f"vcp_0x{code:02X}"),
                "label": definition.get("label", f"VCP 0x{code:02X}"),
                "current": cur_val.value,
                "max": max_val.value,
                "type": pvct.value,
                "supported": True,
            }

    def set_vcp(self, code: int, value: int) -> bool:
        """Write a value to a VCP feature code."""
        with MonitorHandleContext(self.monitor_index, self._lock) as handle:
            res = dxva2.SetVCPFeature(handle, code, value)
            if not res:
                err = ctypes.GetLastError()
                raise RuntimeError(
                    f"SetVCPFeature(0x{code:02X}, {value}) failed with error {err}"
                )
            return True

    def get_capabilities_string(self) -> str:
        """Retrieve the monitor DDC/CI capabilities string."""
        with MonitorHandleContext(self.monitor_index, self._lock) as handle:
            cap_len = wintypes.DWORD()
            if not dxva2.GetCapabilitiesStringLength(handle, ctypes.byref(cap_len)):
                return ""
            cap_buf = ctypes.create_string_buffer(cap_len.value)
            if not dxva2.CapabilitiesRequestAndCapabilitiesReply(
                handle, cap_buf, cap_len.value
            ):
                return ""
            return cap_buf.value.decode("latin1", errors="ignore")

    def get_all_vcp(self) -> Dict[str, Any]:
        """Read all known supported VCP features in a single session."""
        results = {}
        with MonitorHandleContext(self.monitor_index, self._lock) as handle:
            for code, meta in VCP_DEFINITIONS.items():
                pvct = wintypes.DWORD()
                cur_val = wintypes.DWORD()
                max_val = wintypes.DWORD()
                res = dxva2.GetVCPFeatureAndVCPFeatureReply(
                    handle,
                    code,
                    ctypes.byref(pvct),
                    ctypes.byref(cur_val),
                    ctypes.byref(max_val),
                )
                if res:
                    val = cur_val.value

                    # Frequency decoding
                    formatted = val
                    if code == 0xAE:  # Vertical frequency: e.g. 25000 -> 250 Hz
                        # Some scalers report Hz * 100 or raw counter
                        if val > 1000:
                            formatted = round(val / 100.0, 1)
                        else:
                            formatted = val
                    elif code == 0xAC:  # Horizontal frequency in kHz
                        if val > 1000:
                            formatted = round(val / 100.0, 1)

                    results[meta["name"]] = {
                        "code": code,
                        "hex": f"0x{code:02X}",
                        "label": meta["label"],
                        "value": val,
                        "formatted": formatted,
                        "max": max_val.value,
                        "type": meta.get("type", "continuous"),
                        "values": meta.get("values", None),
                        "category": meta.get("category", "other"),
                    }
                time.sleep(0.015)
        return results

    def reset_feature(self, reset_code: int) -> bool:
        """Trigger a reset VCP code (0x04 for Factory, 0x05 for Luminance, 0x08 for Color)."""
        if reset_code not in (0x04, 0x05, 0x08):
            raise ValueError(f"Invalid reset code 0x{reset_code:02X}")
        return self.set_vcp(reset_code, 1)
