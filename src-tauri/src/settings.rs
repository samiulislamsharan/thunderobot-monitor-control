use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

type BOOL = i32;
type HANDLE = *mut std::ffi::c_void;

#[link(name = "user32")]
extern "system" {
    fn RegisterHotKey(hwnd: HANDLE, id: i32, fs_modifiers: u32, vk: u32) -> BOOL;
    fn UnregisterHotKey(hwnd: HANDLE, id: i32) -> BOOL;
}

#[link(name = "kernel32")]
extern "system" {
    fn GetLastError() -> u32;
}

const MOD_ALT: u32 = 0x0001;
const MOD_CONTROL: u32 = 0x0002;
const MOD_SHIFT: u32 = 0x0004;
const MOD_WIN: u32 = 0x0008;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HotkeyConfig {
    pub id: String,
    pub action: String,
    pub key_combination: String,
    pub enabled: bool,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub autostart: bool,
    pub close_to_tray: bool,
    pub start_minimized: bool,
    pub default_profile: String,
    pub i2c_delay_ms: u64,
    pub telemetry_interval_sec: u64,
    pub phone_remote_enabled: bool,
    pub phone_remote_port: u16,
    pub hotkeys: Vec<HotkeyConfig>,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            autostart: false,
            close_to_tray: true,
            start_minimized: false,
            default_profile: "preset-fps-250".into(),
            i2c_delay_ms: 35,
            telemetry_interval_sec: 3,
            phone_remote_enabled: true,
            phone_remote_port: 8000,
            hotkeys: vec![
                HotkeyConfig {
                    id: "brightness_up".into(),
                    action: "Brightness Up (+5%)".into(),
                    key_combination: "Ctrl+Alt+Up".into(),
                    enabled: true,
                    description: "Increase display backlight luminance by 5%".into(),
                },
                HotkeyConfig {
                    id: "brightness_down".into(),
                    action: "Brightness Down (-5%)".into(),
                    key_combination: "Ctrl+Alt+Down".into(),
                    enabled: true,
                    description: "Decrease display backlight luminance by 5%".into(),
                },
                HotkeyConfig {
                    id: "contrast_up".into(),
                    action: "Contrast Up (+5%)".into(),
                    key_combination: "Ctrl+Alt+Right".into(),
                    enabled: true,
                    description: "Increase display contrast ratio by 5%".into(),
                },
                HotkeyConfig {
                    id: "contrast_down".into(),
                    action: "Contrast Down (-5%)".into(),
                    key_combination: "Ctrl+Alt+Left".into(),
                    enabled: true,
                    description: "Decrease display contrast ratio by 5%".into(),
                },
                HotkeyConfig {
                    id: "preset_fps".into(),
                    action: "Preset: 250Hz Esports".into(),
                    key_combination: "Ctrl+Alt+1".into(),
                    enabled: true,
                    description: "Switch to Competitive 250Hz FPS gaming preset".into(),
                },
                HotkeyConfig {
                    id: "preset_night".into(),
                    action: "Preset: Night Mode".into(),
                    key_combination: "Ctrl+Alt+2".into(),
                    enabled: true,
                    description: "Switch to Eye-Care Low Blue Light preset".into(),
                },
                HotkeyConfig {
                    id: "preset_cinema".into(),
                    action: "Preset: Cinema 21:9".into(),
                    key_combination: "Ctrl+Alt+3".into(),
                    enabled: true,
                    description: "Switch to 21:9 Ultrawide Cinema Movie preset".into(),
                },
                HotkeyConfig {
                    id: "preset_work".into(),
                    action: "Preset: Standard Work".into(),
                    key_combination: "Ctrl+Alt+4".into(),
                    enabled: true,
                    description: "Switch to 6500K calibrated office / design preset".into(),
                },
                HotkeyConfig {
                    id: "toggle_standby".into(),
                    action: "Toggle Standby / Sleep".into(),
                    key_combination: "Ctrl+Alt+End".into(),
                    enabled: true,
                    description: "Put monitor into DDC/CI power-saving standby".into(),
                },
                HotkeyConfig {
                    id: "toggle_window".into(),
                    action: "Show / Hide Window".into(),
                    key_combination: "Ctrl+Alt+M".into(),
                    enabled: true,
                    description: "Toggle main controller window visibility".into(),
                },
            ],
        }
    }
}

pub fn parse_hotkey_str(combo: &str) -> Result<(u32, u32), String> {
    let mut mods = 0u32;
    let mut vk = 0u32;

    for part in combo.split('+').map(|s| s.trim().to_uppercase()) {
        match part.as_str() {
            "CTRL" | "CONTROL" => mods |= MOD_CONTROL,
            "ALT" | "MENU" => mods |= MOD_ALT,
            "SHIFT" => mods |= MOD_SHIFT,
            "WIN" | "SUPER" | "META" => mods |= MOD_WIN,
            "UP" => vk = 0x26,
            "DOWN" => vk = 0x28,
            "LEFT" => vk = 0x25,
            "RIGHT" => vk = 0x27,
            "END" => vk = 0x23,
            "HOME" => vk = 0x24,
            "PAGEUP" => vk = 0x21,
            "PAGEDOWN" => vk = 0x22,
            "SPACE" => vk = 0x20,
            "ENTER" => vk = 0x0D,
            "ESCAPE" => vk = 0x1B,
            f if f.starts_with('F') && f.len() > 1 => {
                if let Ok(num) = f[1..].parse::<u32>() {
                    if (1..=24).contains(&num) {
                        vk = 0x70 + (num - 1);
                    }
                }
            }
            c if c.len() == 1 => {
                vk = c.chars().next().unwrap() as u32;
            }
            _ => return Err(format!("Unrecognized key: {}", part)),
        }
    }

    if vk == 0 {
        return Err("No valid non-modifier key specified".into());
    }
    Ok((mods, vk))
}

pub fn test_os_conflict(combo: &str) -> (bool, String) {
    match parse_hotkey_str(combo) {
        Ok((mods, vk)) => unsafe {
            let test_id = 9988;
            let res = RegisterHotKey(std::ptr::null_mut(), test_id, mods, vk);
            let err = GetLastError();

            if res != 0 {
                UnregisterHotKey(std::ptr::null_mut(), test_id);
                (true, "Available and supported by OS".into())
            } else if err == 1409 {
                (
                    false,
                    "Conflict: Already registered by Windows or another running app".into(),
                )
            } else {
                (false, format!("Windows error code: {}", err))
            }
        },
        Err(e) => (false, e),
    }
}

pub fn validate_hotkey_against_settings(
    settings: &AppSettings,
    combo: &str,
    action_id: Option<&str>,
) -> serde_json::Value {
    let norm = combo
        .split('+')
        .map(|s| s.trim())
        .collect::<Vec<_>>()
        .join("+");

    // 1. Format check
    if let Err(e) = parse_hotkey_str(&norm) {
        return serde_json::json!({
            "valid": false,
            "status": "invalid_format",
            "message": format!("Invalid format: {}", e),
            "normalized": norm
        });
    }

    // 2. Internal duplicate check
    for hk in &settings.hotkeys {
        if hk.enabled && action_id != Some(&hk.id) {
            if hk.key_combination.eq_ignore_ascii_case(&norm) {
                return serde_json::json!({
                    "valid": false,
                    "status": "internal_conflict",
                    "message": format!("Conflict: Already assigned to '{}'", hk.action),
                    "conflicting_action": hk.action,
                    "normalized": norm
                });
            }
        }
    }

    // 3. System conflict probe
    let (os_ok, os_msg) = test_os_conflict(&norm);
    if !os_ok {
        return serde_json::json!({
            "valid": false,
            "status": "system_conflict",
            "message": os_msg,
            "normalized": norm
        });
    }

    serde_json::json!({
        "valid": true,
        "status": "ok",
        "message": "Hotkey is valid and ready to use",
        "normalized": norm
    })
}

pub fn get_settings_file_path() -> PathBuf {
    if let Ok(app_data) = std::env::var("APPDATA") {
        let dir = PathBuf::from(app_data).join("thunderobot-monitor-control");
        let _ = fs::create_dir_all(&dir);
        dir.join("settings.json")
    } else {
        PathBuf::from("settings.json")
    }
}

pub fn load_settings() -> AppSettings {
    let path = get_settings_file_path();
    if path.exists() {
        if let Ok(content) = fs::read_to_string(&path) {
            if let Ok(s) = serde_json::from_str::<AppSettings>(&content) {
                return s;
            }
        }
    }
    let s = AppSettings::default();
    let _ = save_settings(&s);
    s
}

pub fn save_settings(settings: &AppSettings) -> Result<(), String> {
    let path = get_settings_file_path();
    let content = serde_json::to_string_pretty(settings).map_err(|e| e.to_string())?;
    fs::write(path, content).map_err(|e| e.to_string())
}
