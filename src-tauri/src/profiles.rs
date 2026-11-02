use crate::ddcci::DdcciController;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default = "default_icon")]
    pub icon: String,
    #[serde(default)]
    pub built_in: bool,
    #[serde(default)]
    pub badge: Option<String>,
    #[serde(default)]
    pub color: Option<String>,
    pub settings: HashMap<String, u32>,
}

fn default_icon() -> String {
    "Sparkles".to_string()
}

pub fn default_profiles() -> Vec<Profile> {
    vec![
        Profile {
            id: "preset-fps-250".into(),
            name: "Competitive FPS 250Hz".into(),
            description: "Max brightness, FPS mode, fast overdrive, crisp clarity for esports".into(),
            icon: "Crosshair".into(),
            built_in: true,
            badge: Some("250Hz Esports".into()),
            color: Some("#00f0ff".into()),
            settings: [
                ("brightness".into(), 100),
                ("contrast".into(), 50),
                ("picture_mode".into(), 5),
                ("color_preset".into(), 11),
                ("red_gain".into(), 50),
                ("green_gain".into(), 50),
                ("blue_gain".into(), 50),
                ("gaming_sharpness".into(), 60),
                ("overdrive".into(), 3),
                ("mprt_aim".into(), 1),
            ].into_iter().collect(),
        },
        Profile {
            id: "preset-night-reading".into(),
            name: "Night Mode / Low Blue".into(),
            description: "Gentle backlight, warm 6500K color temperature to eliminate eye strain".into(),
            icon: "Moon".into(),
            built_in: true,
            badge: Some("Eye Care".into()),
            color: Some("#f59e0b".into()),
            settings: [
                ("brightness".into(), 25),
                ("contrast".into(), 42),
                ("picture_mode".into(), 0),
                ("color_preset".into(), 5),
                ("red_gain".into(), 55),
                ("green_gain".into(), 48),
                ("blue_gain".into(), 38),
                ("gaming_sharpness".into(), 50),
                ("overdrive".into(), 0),
                ("mprt_aim".into(), 0),
            ].into_iter().collect(),
        },
        Profile {
            id: "preset-cinema".into(),
            name: "Cinema 21:9 Movie".into(),
            description: "Deep blacks, high dynamic range feel, vibrant color grading for ultrawide films".into(),
            icon: "Film".into(),
            built_in: true,
            badge: Some("HDR / Cinema".into()),
            color: Some("#a855f7".into()),
            settings: [
                ("brightness".into(), 80),
                ("contrast".into(), 55),
                ("picture_mode".into(), 2),
                ("color_preset".into(), 6),
                ("red_gain".into(), 52),
                ("green_gain".into(), 50),
                ("blue_gain".into(), 50),
                ("gaming_sharpness".into(), 55),
                ("overdrive".into(), 1),
                ("mprt_aim".into(), 0),
            ].into_iter().collect(),
        },
        Profile {
            id: "preset-standard-work".into(),
            name: "Standard Office / Work".into(),
            description: "Balanced luminance and neutral 50/50 color balance for all-day coding and work".into(),
            icon: "Monitor".into(),
            built_in: true,
            badge: Some("Productivity".into()),
            color: Some("#10b981".into()),
            settings: [
                ("brightness".into(), 65),
                ("contrast".into(), 50),
                ("picture_mode".into(), 0),
                ("color_preset".into(), 6),
                ("red_gain".into(), 50),
                ("green_gain".into(), 50),
                ("blue_gain".into(), 50),
                ("gaming_sharpness".into(), 50),
                ("overdrive".into(), 1),
                ("mprt_aim".into(), 0),
            ].into_iter().collect(),
        },
    ]
}

pub fn get_profiles_file_path() -> PathBuf {
    if let Ok(app_data) = std::env::var("APPDATA") {
        let dir = PathBuf::from(app_data).join("thunderobot-monitor-control");
        let _ = fs::create_dir_all(&dir);
        dir.join("profiles.json")
    } else {
        PathBuf::from("profiles.json")
    }
}

pub fn load_profiles() -> Vec<Profile> {
    let path = get_profiles_file_path();
    if path.exists() {
        if let Ok(content) = fs::read_to_string(&path) {
            if let Ok(profiles) = serde_json::from_str::<Vec<Profile>>(&content) {
                if !profiles.is_empty() {
                    return profiles;
                }
            }
        }
    }
    let defaults = default_profiles();
    let _ = save_profiles(&defaults);
    defaults
}

pub fn save_profiles(profiles: &[Profile]) -> Result<(), String> {
    let path = get_profiles_file_path();
    let content = serde_json::to_string_pretty(profiles).map_err(|e| e.to_string())?;
    fs::write(path, content).map_err(|e| e.to_string())
}

pub fn feature_name_to_vcp_code(name: &str) -> Option<u8> {
    match name {
        "brightness" => Some(0x10),
        "contrast" => Some(0x12),
        "picture_mode" => Some(0xDC),
        "color_preset" => Some(0x14),
        "red_gain" => Some(0x16),
        "green_gain" => Some(0x18),
        "blue_gain" => Some(0x1A),
        "red_black_level" => Some(0x6C),
        "green_black_level" => Some(0x6E),
        "blue_black_level" => Some(0x70),
        "input_source" => Some(0x60),
        "overdrive" => Some(0x68),
        "gaming_sharpness" => Some(0x3E),
        "mprt_aim" => Some(0xE3),
        "orientation" => Some(0xAA),
        "power_mode" => Some(0xD6),
        _ => None,
    }
}

pub fn apply_profile_to_monitor(ddcci: &DdcciController, profile: &Profile) -> Result<(), String> {
    for (feat_name, &val) in &profile.settings {
        if let Some(code) = feature_name_to_vcp_code(feat_name) {
            let _ = ddcci.set_vcp(code, val);
        }
    }
    Ok(())
}
