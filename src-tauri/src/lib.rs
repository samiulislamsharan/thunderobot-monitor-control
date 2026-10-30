pub mod ddcci;
pub mod settings;

use ddcci::DdcciController;
use settings::{load_settings, save_settings, validate_hotkey_against_settings, AppSettings};
use std::sync::{Arc, Mutex};
use tauri::{
    menu::{Menu, MenuItem},
    tray::{TrayIconBuilder, TrayIconEvent},
    Emitter, Manager, State,
};

pub struct AppState {
    pub ddcci: Arc<DdcciController>,
    pub settings: Arc<Mutex<AppSettings>>,
}

#[tauri::command]
fn get_monitor_info(state: State<AppState>) -> serde_json::Value {
    serde_json::json!({
        "model": "DQ34C240L",
        "vendor": "ThundeRobot",
        "full_name": "ThundeRobot DQ34C240L 34″ Fast VA 250Hz Curved Gaming Monitor",
        "resolution": "3440x1440",
        "refresh_rate": 250,
        "curved": true,
        "panel_type": "Fast VA",
        "aspect_ratio": "21:9 Ultrawide",
        "lan_ip": "127.0.0.1",
        "port": 8000,
        "local_url": "http://localhost:8000",
        "phone_url": "http://localhost:8000",
        "telemetry": {
            "v_freq": 250,
            "h_freq": 473.2,
            "usage_hours": 127
        }
    })
}

#[tauri::command]
fn get_features(state: State<AppState>) -> Result<serde_json::Value, String> {
    let brightness = state.ddcci.get_vcp(0x10).map(|(c, _)| c).unwrap_or(95);
    let contrast = state.ddcci.get_vcp(0x12).map(|(c, _)| c).unwrap_or(45);
    let picture_mode = state.ddcci.get_vcp(0xDC).map(|(c, _)| c).unwrap_or(0);
    let color_preset = state.ddcci.get_vcp(0x14).map(|(c, _)| c).unwrap_or(11);
    let red_gain = state.ddcci.get_vcp(0x16).map(|(c, _)| c).unwrap_or(50);
    let green_gain = state.ddcci.get_vcp(0x18).map(|(c, _)| c).unwrap_or(50);
    let blue_gain = state.ddcci.get_vcp(0x1A).map(|(c, _)| c).unwrap_or(50);
    let input_source = state.ddcci.get_vcp(0x60).map(|(c, _)| c).unwrap_or(15);
    let overdrive = state.ddcci.get_vcp(0x68).map(|(c, _)| c).unwrap_or(0);
    let gaming_sharpness = state.ddcci.get_vcp(0x3E).map(|(c, _)| c).unwrap_or(50);
    let mprt_aim = state.ddcci.get_vcp(0xE3).map(|(c, _)| c).unwrap_or(0);

    Ok(serde_json::json!({
        "features": {
            "brightness": { "value": brightness, "hex": "0x10" },
            "contrast": { "value": contrast, "hex": "0x12" },
            "picture_mode": { "value": picture_mode, "hex": "0xDC" },
            "color_preset": { "value": color_preset, "hex": "0x14" },
            "red_gain": { "value": red_gain, "hex": "0x16" },
            "green_gain": { "value": green_gain, "hex": "0x18" },
            "blue_gain": { "value": blue_gain, "hex": "0x1A" },
            "input_source": { "value": input_source, "hex": "0x60" },
            "overdrive": { "value": overdrive, "hex": "0x68" },
            "gaming_sharpness": { "value": gaming_sharpness, "hex": "0x3E" },
            "mprt_aim": { "value": mprt_aim, "hex": "0xE3" }
        }
    }))
}

#[tauri::command]
fn set_feature(
    state: State<AppState>,
    name: String,
    value: u32,
    _vcp_type: Option<u32>,
) -> Result<serde_json::Value, String> {
    let code: u8 = match name.as_str() {
        "brightness" => 0x10,
        "contrast" => 0x12,
        "picture_mode" => 0xDC,
        "color_preset" => 0x14,
        "red_gain" => 0x16,
        "green_gain" => 0x18,
        "blue_gain" => 0x1A,
        "input_source" => 0x60,
        "overdrive" => 0x68,
        "gaming_sharpness" => 0x3E,
        "mprt_aim" => 0xE3,
        "orientation" => 0xAA,
        "power_mode" => 0xD6,
        _ => return Err(format!("Unknown feature: {}", name)),
    };

    state.ddcci.set_vcp(code, value)?;
    Ok(serde_json::json!({ "success": true, "feature": name, "value": value }))
}

#[tauri::command]
fn set_raw_vcp(state: State<AppState>, code: u8, value: u32) -> Result<serde_json::Value, String> {
    state.ddcci.set_vcp(code, value)?;
    Ok(serde_json::json!({ "success": true, "code": format!("0x{:02X}", code), "value": value }))
}

#[tauri::command]
fn reset_monitor(state: State<AppState>, reset_type: String) -> Result<serde_json::Value, String> {
    let code = match reset_type.as_str() {
        "luminance" => 0x05,
        "color" => 0x08,
        "factory" => 0x04,
        _ => return Err(format!("Unknown reset type: {}", reset_type)),
    };
    state.ddcci.set_vcp(code, 1)?;
    Ok(serde_json::json!({ "success": true, "reset_type": reset_type }))
}

#[tauri::command]
fn get_settings(state: State<AppState>) -> serde_json::Value {
    let settings = state.settings.lock().unwrap();
    serde_json::to_value(&*settings).unwrap_or(serde_json::json!({}))
}

#[tauri::command]
fn save_settings_cmd(
    state: State<AppState>,
    settings: AppSettings,
) -> Result<serde_json::Value, String> {
    save_settings(&settings)?;
    state.ddcci.set_delay(settings.i2c_delay_ms);
    if let Ok(mut current) = state.settings.lock() {
        *current = settings;
    }
    Ok(serde_json::json!({ "success": true }))
}

#[tauri::command]
fn validate_hotkey_cmd(
    state: State<AppState>,
    key_combination: String,
    action_id: Option<String>,
) -> serde_json::Value {
    let settings = state.settings.lock().unwrap();
    validate_hotkey_against_settings(&*settings, &key_combination, action_id.as_deref())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let settings = load_settings();
    let ddcci = Arc::new(DdcciController::new(settings.i2c_delay_ms));
    let settings_arc = Arc::new(Mutex::new(settings));

    let app_state = AppState {
        ddcci: ddcci.clone(),
        settings: settings_arc.clone(),
    };

    tauri::Builder::default()
        .manage(app_state)
        .plugin(tauri_plugin_log::Builder::default().build())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            Some(vec!["--minimized"]),
        ))
        .setup(|app| {
            // Build Native System Tray
            let toggle_item = MenuItem::with_id(app, "toggle", "Show / Hide Controller", true, None::<&str>)?;
            let preset_fps = MenuItem::with_id(app, "fps_250", "Preset: 250Hz Esports", true, None::<&str>)?;
            let preset_night = MenuItem::with_id(app, "night", "Preset: Night Mode", true, None::<&str>)?;
            let quit_item = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;

            let menu = Menu::with_items(
                app,
                &[
                    &toggle_item,
                    &preset_fps,
                    &preset_night,
                    &quit_item,
                ],
            )?;

            let _tray = TrayIconBuilder::new()
                .menu(&menu)
                .icon(app.default_window_icon().unwrap().clone())
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "toggle" => {
                        if let Some(win) = app.get_webview_window("main") {
                            if win.is_visible().unwrap_or(false) {
                                let _ = win.hide();
                            } else {
                                let _ = win.show();
                                let _ = win.set_focus();
                            }
                        }
                    }
                    "fps_250" => {
                        let state = app.state::<AppState>();
                        let _ = state.ddcci.set_vcp(0x10, 95);
                        let _ = state.ddcci.set_vcp(0x12, 45);
                        let _ = state.ddcci.set_vcp(0x68, 5); // Overdrive Level 5
                    }
                    "night" => {
                        let state = app.state::<AppState>();
                        let _ = state.ddcci.set_vcp(0x10, 25);
                        let _ = state.ddcci.set_vcp(0x12, 40);
                        let _ = state.ddcci.set_vcp(0x14, 5); // Warm
                    }
                    "quit" => {
                        app.exit(0);
                    }
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click { .. } = event {
                        let app = tray.app_handle();
                        if let Some(win) = app.get_webview_window("main") {
                            let _ = win.show();
                            let _ = win.set_focus();
                        }
                    }
                })
                .build(app)?;

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_monitor_info,
            get_features,
            set_feature,
            set_raw_vcp,
            reset_monitor,
            get_settings,
            save_settings_cmd,
            validate_hotkey_cmd
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
