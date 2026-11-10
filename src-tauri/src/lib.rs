pub mod ddcci;
pub mod profiles;
pub mod server;
pub mod settings;

use ddcci::DdcciController;
use profiles::{apply_profile_to_monitor, load_profiles, save_profiles, Profile};
use settings::{
    apply_startup_settings, load_settings, save_settings, update_saved_monitor_setting,
    validate_hotkey_against_settings, AppSettings, SavedMonitorSettings,
};
use std::sync::{Arc, Mutex};
use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Manager, State,
};

pub struct AppState {
    pub ddcci: Arc<DdcciController>,
    pub settings: Arc<Mutex<AppSettings>>,
}

#[tauri::command]
fn get_monitor_info(state: State<AppState>) -> serde_json::Value {
    let port = {
        let set = state.settings.lock().unwrap();
        set.phone_remote_port
    };
    let lan_ip = server::get_lan_ip();
    let phone_url = format!("http://{}:{}", lan_ip, port);

    serde_json::json!({
        "model": "DQ34C240L",
        "vendor": "ThundeRobot",
        "full_name": "ThundeRobot DQ34C240L 34″ Fast VA 250Hz Curved Gaming Monitor",
        "resolution": "3440x1440",
        "refresh_rate": 250,
        "curved": true,
        "panel_type": "Fast VA",
        "aspect_ratio": "21:9 Ultrawide",
        "lan_ip": lan_ip,
        "port": port,
        "local_url": format!("http://localhost:{}", port),
        "phone_url": phone_url,
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
    let red_black_level = state.ddcci.get_vcp(0x6C).map(|(c, _)| c).unwrap_or(50);
    let green_black_level = state.ddcci.get_vcp(0x6E).map(|(c, _)| c).unwrap_or(50);
    let blue_black_level = state.ddcci.get_vcp(0x70).map(|(c, _)| c).unwrap_or(50);
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
            "red_black_level": { "value": red_black_level, "hex": "0x6C" },
            "green_black_level": { "value": green_black_level, "hex": "0x6E" },
            "blue_black_level": { "value": blue_black_level, "hex": "0x70" },
            "input_source": { "value": input_source, "hex": "0x60" },
            "overdrive": { "value": overdrive, "hex": "0x68" },
            "gaming_sharpness": { "value": gaming_sharpness, "hex": "0x3E" },
            "mprt_aim": { "value": mprt_aim, "hex": "0xE3" }
        }
    }))
}

pub fn resolve_feature_code(
    name: Option<&str>,
    code: Option<&serde_json::Value>,
) -> Result<u8, String> {
    if let Some(n) = name {
        match n {
            "brightness" => return Ok(0x10),
            "contrast" => return Ok(0x12),
            "picture_mode" => return Ok(0xDC),
            "color_preset" => return Ok(0x14),
            "red_gain" => return Ok(0x16),
            "green_gain" => return Ok(0x18),
            "blue_gain" => return Ok(0x1A),
            "red_black_level" => return Ok(0x6C),
            "green_black_level" => return Ok(0x6E),
            "blue_black_level" => return Ok(0x70),
            "input_source" => return Ok(0x60),
            "overdrive" => return Ok(0x68),
            "gaming_sharpness" => return Ok(0x3E),
            "mprt_aim" => return Ok(0xE3),
            "orientation" => return Ok(0xAA),
            "power_mode" => return Ok(0xD6),
            "reset_luminance" => return Ok(0x05),
            "reset_color" => return Ok(0x08),
            "reset_factory" => return Ok(0x04),
            s => {
                if let Ok(c) = s.parse::<u8>() {
                    return Ok(c);
                }
                if s.starts_with("0x") || s.starts_with("0X") {
                    if let Ok(c) = u8::from_str_radix(&s[2..], 16) {
                        return Ok(c);
                    }
                }
            }
        }
    }
    if let Some(c) = code {
        if let Some(n) = c.as_u64() {
            return Ok(n as u8);
        }
        if let Some(s) = c.as_str() {
            return resolve_feature_code(Some(s), None);
        }
    }
    Err("Unknown or missing feature identifier".into())
}

#[tauri::command]
fn set_feature(
    state: State<AppState>,
    name: Option<String>,
    code: Option<serde_json::Value>,
    value: u32,
    _vcp_type: Option<u32>,
) -> Result<serde_json::Value, String> {
    let resolved_code = resolve_feature_code(name.as_deref(), code.as_ref())?;
    state.ddcci.set_vcp(resolved_code, value)?;

    if let Ok(mut set) = state.settings.lock() {
        update_saved_monitor_setting(&mut *set, resolved_code, value);
        let _ = save_settings(&*set);
    }

    Ok(serde_json::json!({
        "success": true,
        "feature": name.unwrap_or_else(|| format!("0x{:02X}", resolved_code)),
        "code": resolved_code,
        "value": value
    }))
}

#[tauri::command]
fn set_raw_vcp(
    state: State<AppState>,
    code: Option<serde_json::Value>,
    opcode: Option<serde_json::Value>,
    value: u32,
) -> Result<serde_json::Value, String> {
    let target = code.or(opcode).ok_or("Missing VCP code")?;
    let resolved = resolve_feature_code(None, Some(&target))?;
    state.ddcci.set_vcp(resolved, value)?;
    Ok(serde_json::json!({
        "success": true,
        "code": format!("0x{:02X}", resolved),
        "value": value
    }))
}

#[tauri::command]
#[allow(non_snake_case)]
fn reset_monitor(
    state: State<AppState>,
    reset_type: Option<String>,
    resetType: Option<String>,
) -> Result<serde_json::Value, String> {
    let r_type = reset_type.or(resetType).unwrap_or_else(|| "factory".into());
    let code = match r_type.as_str() {
        "luminance" => 0x05,
        "color" => 0x08,
        "factory" | "all" => 0x04,
        _ => return Err(format!("Unknown reset type: {}", r_type)),
    };
    state.ddcci.set_vcp(code, 1)?;
    Ok(serde_json::json!({ "success": true, "reset_type": r_type }))
}

#[tauri::command]
fn get_profiles() -> serde_json::Value {
    let list = load_profiles();
    serde_json::json!({ "profiles": list })
}

#[tauri::command]
#[allow(non_snake_case)]
fn apply_profile(
    state: State<AppState>,
    profile_id: Option<String>,
    profileId: Option<String>,
) -> Result<serde_json::Value, String> {
    let pid = profile_id.or(profileId).ok_or("Missing profile_id")?;
    let list = load_profiles();
    if let Some(target) = list.iter().find(|p| p.id == pid) {
        apply_profile_to_monitor(&state.ddcci, target)?;
        if let Ok(mut set) = state.settings.lock() {
            for (feat, &val) in &target.settings {
                if let Some(code) = profiles::feature_name_to_vcp_code(feat) {
                    update_saved_monitor_setting(&mut *set, code, val);
                }
            }
            let _ = save_settings(&*set);
        }
        Ok(serde_json::json!({ "success": true, "profile_id": pid }))
    } else {
        Err(format!("Profile {} not found", pid))
    }
}

#[tauri::command]
fn create_profile(profile: Profile) -> Result<serde_json::Value, String> {
    let mut list = load_profiles();
    let mut new_p = profile;
    if new_p.id.is_empty() {
        new_p.id = format!(
            "custom-{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis()
        );
    }
    list.push(new_p.clone());
    save_profiles(&list)?;
    Ok(serde_json::json!({ "success": true, "profile": new_p }))
}

#[tauri::command]
#[allow(non_snake_case)]
fn delete_profile(
    profile_id: Option<String>,
    profileId: Option<String>,
) -> Result<serde_json::Value, String> {
    let pid = profile_id.or(profileId).ok_or("Missing profile_id")?;
    let mut list = load_profiles();
    if let Some(idx) = list.iter().position(|p| p.id == pid) {
        if list[idx].built_in {
            return Err("Cannot delete built-in preset".into());
        }
        list.remove(idx);
        save_profiles(&list)?;
        Ok(serde_json::json!({ "success": true }))
    } else {
        Err(format!("Profile {} not found", pid))
    }
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
fn snapshot_monitor_state_cmd(state: State<AppState>) -> Result<serde_json::Value, String> {
    let brightness = state.ddcci.get_vcp(0x10).ok().map(|(c, _)| c);
    let contrast = state.ddcci.get_vcp(0x12).ok().map(|(c, _)| c);
    let picture_mode = state.ddcci.get_vcp(0xDC).ok().map(|(c, _)| c);
    let color_preset = state.ddcci.get_vcp(0x14).ok().map(|(c, _)| c);
    let red_gain = state.ddcci.get_vcp(0x16).ok().map(|(c, _)| c);
    let green_gain = state.ddcci.get_vcp(0x18).ok().map(|(c, _)| c);
    let blue_gain = state.ddcci.get_vcp(0x1A).ok().map(|(c, _)| c);
    let red_black_level = state.ddcci.get_vcp(0x6C).ok().map(|(c, _)| c);
    let green_black_level = state.ddcci.get_vcp(0x6E).ok().map(|(c, _)| c);
    let blue_black_level = state.ddcci.get_vcp(0x70).ok().map(|(c, _)| c);
    let overdrive = state.ddcci.get_vcp(0x68).ok().map(|(c, _)| c);
    let gaming_sharpness = state.ddcci.get_vcp(0x3E).ok().map(|(c, _)| c);
    let mprt_aim = state.ddcci.get_vcp(0xE3).ok().map(|(c, _)| c);

    let saved = SavedMonitorSettings {
        brightness,
        contrast,
        picture_mode,
        color_preset,
        red_gain,
        green_gain,
        blue_gain,
        red_black_level,
        green_black_level,
        blue_black_level,
        overdrive,
        gaming_sharpness,
        mprt_aim,
    };

    if let Ok(mut set) = state.settings.lock() {
        set.saved_monitor_settings = Some(saved.clone());
        save_settings(&*set)?;
    }

    Ok(serde_json::json!({
        "success": true,
        "saved_settings": saved
    }))
}

#[tauri::command]
#[allow(non_snake_case)]
fn validate_hotkey_cmd(
    state: State<AppState>,
    key_combination: Option<String>,
    keyCombination: Option<String>,
    action_id: Option<String>,
    actionId: Option<String>,
) -> serde_json::Value {
    let combo = key_combination.or(keyCombination).unwrap_or_default();
    let aid = action_id.or(actionId);
    let settings = state.settings.lock().unwrap();
    validate_hotkey_against_settings(&*settings, &combo, aid.as_deref())
}

pub fn toggle_quickpanel_window(app: &tauri::AppHandle) {
    if let Some(panel) = app.get_webview_window("quickpanel") {
        if panel.is_visible().unwrap_or(false) {
            let _ = panel.hide();
        } else {
            if let Ok(Some(monitor)) = panel.primary_monitor() {
                let size = monitor.size();
                let scale = monitor.scale_factor();
                let width = (410.0 * scale) as i32;
                let height = (590.0 * scale) as i32;
                // Position in bottom-right corner right above taskbar
                let x = (size.width as i32) - width - (16.0 * scale) as i32;
                let y = (size.height as i32) - height - (60.0 * scale) as i32;
                let _ =
                    panel.set_position(tauri::Position::Physical(tauri::PhysicalPosition { x, y }));
            }
            let _ = panel.show();
            let _ = panel.set_focus();
        }
    }
}

pub fn show_main_window(app: &tauri::AppHandle) {
    if let Some(win) = app.get_webview_window("main") {
        let _ = win.show();
        let _ = win.unminimize();
        let _ = win.set_focus();
    }
}

#[tauri::command]
fn open_main_window_cmd(app: tauri::AppHandle) {
    show_main_window(&app);
}

#[tauri::command]
fn hide_quickpanel_cmd(app: tauri::AppHandle) {
    if let Some(panel) = app.get_webview_window("quickpanel") {
        let _ = panel.hide();
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let settings = load_settings();
    let phone_enabled = settings.phone_remote_enabled;
    let phone_port = settings.phone_remote_port;

    let ddcci = Arc::new(DdcciController::new(settings.i2c_delay_ms));
    let settings_arc = Arc::new(Mutex::new(settings.clone()));

    // Automatically apply saved settings / profile on application startup if enabled
    if settings.restore_settings_on_startup {
        let startup_ddcci = ddcci.clone();
        let startup_settings = settings.clone();
        std::thread::spawn(move || {
            // Give display driver & monitor scalar time to initialize
            std::thread::sleep(std::time::Duration::from_millis(800));
            apply_startup_settings(&startup_ddcci, &startup_settings);
        });
    }

    if phone_enabled {
        let s_ddcci = ddcci.clone();
        let s_settings = settings_arc.clone();
        std::thread::spawn(move || {
            server::start_server(s_ddcci, s_settings, phone_port);
        });
    }

    let app_state = AppState {
        ddcci: ddcci.clone(),
        settings: settings_arc.clone(),
    };

    let start_minimized = settings.start_minimized;

    let mut builder = tauri::Builder::default()
        .manage(app_state)
        .plugin(tauri_plugin_log::Builder::default().build())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            Some(vec!["--minimized"]),
        ));

    #[cfg(debug_assertions)]
    {
        builder = builder.plugin(tauri_plugin_mcp_bridge::init());
    }

    builder
        .setup(move |app| {
            // Build Native System Tray Menu
            let quick_item = MenuItem::with_id(
                app,
                "quick_panel",
                "Quick Panel (Sliders & Presets)",
                true,
                None::<&str>,
            )?;
            let main_item = MenuItem::with_id(
                app,
                "main_window",
                "Full Controller Dashboard",
                true,
                None::<&str>,
            )?;
            let preset_fps =
                MenuItem::with_id(app, "fps_250", "Preset: 250Hz Esports", true, None::<&str>)?;
            let preset_night =
                MenuItem::with_id(app, "night", "Preset: Night Mode", true, None::<&str>)?;
            let quit_item = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;

            let menu = Menu::with_items(
                app,
                &[
                    &quick_item,
                    &main_item,
                    &preset_fps,
                    &preset_night,
                    &quit_item,
                ],
            )?;

            let mut tray_builder = TrayIconBuilder::new()
                .menu(&menu)
                .show_menu_on_left_click(false);

            if let Some(icon) = app.default_window_icon() {
                tray_builder = tray_builder.icon(icon.clone());
            }

            let _tray = tray_builder
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "quick_panel" => {
                        toggle_quickpanel_window(app);
                    }
                    "main_window" => {
                        show_main_window(app);
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
                .on_tray_icon_event(|tray, event| match event {
                    TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } => {
                        let app = tray.app_handle();
                        toggle_quickpanel_window(app);
                    }
                    TrayIconEvent::DoubleClick { .. } => {
                        let app = tray.app_handle();
                        show_main_window(app);
                    }
                    _ => {}
                })
                .build(app)?;

            // Auto-hide quickpanel when it loses focus (clicked outside)
            if let Some(panel) = app.get_webview_window("quickpanel") {
                let panel_clone = panel.clone();
                panel.on_window_event(move |event| {
                    if let tauri::WindowEvent::Focused(false) = event {
                        let _ = panel_clone.hide();
                    }
                });
            }

            // Ensure Main window is created and displayed according to start_minimized
            let win = if let Some(w) = app.get_webview_window("main") {
                w
            } else {
                tauri::WebviewWindowBuilder::new(app, "main", tauri::WebviewUrl::default())
                    .title("ThundeRobot Monitor Control")
                    .inner_size(1050.0, 780.0)
                    .resizable(true)
                    .build()?
            };

            if !start_minimized {
                let _ = win.show();
                let _ = win.set_focus();
            }

            // Handle close-to-tray if configured
            let win_clone = win.clone();
            let settings_close = settings_arc.clone();
            win.on_window_event(move |event| {
                if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                    let close_to_tray = {
                        let s = settings_close.lock().unwrap();
                        s.close_to_tray
                    };
                    if close_to_tray {
                        api.prevent_close();
                        let _ = win_clone.hide();
                    }
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_monitor_info,
            get_features,
            set_feature,
            set_raw_vcp,
            reset_monitor,
            get_profiles,
            apply_profile,
            create_profile,
            delete_profile,
            get_settings,
            save_settings_cmd,
            snapshot_monitor_state_cmd,
            validate_hotkey_cmd,
            open_main_window_cmd,
            hide_quickpanel_cmd
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
