import json
import os
import uuid
from typing import List, Dict, Any, Optional

PROFILES_FILE = os.path.join(os.path.dirname(__file__), "profiles.json")

DEFAULT_PROFILES = [
    {
        "id": "preset-fps-250",
        "name": "Competitive FPS 250Hz",
        "description": "Max brightness, FPS mode, fast overdrive, crisp clarity for esports",
        "icon": "Crosshair",
        "built_in": True,
        "badge": "250Hz Esports",
        "color": "#00f0ff",
        "settings": {
            "brightness": 100,
            "contrast": 50,
            "picture_mode": 5,      # FPS
            "color_preset": 11,     # User
            "red_gain": 50,
            "green_gain": 50,
            "blue_gain": 50,
            "gaming_sharpness": 60,
            "overdrive": 3,
            "mprt_aim": 1
        }
    },
    {
        "id": "preset-night-reading",
        "name": "Night Mode / Low Blue",
        "description": "Gentle backlight, warm 6500K color temperature to eliminate eye strain",
        "icon": "Moon",
        "built_in": True,
        "badge": "Eye Care",
        "color": "#f59e0b",
        "settings": {
            "brightness": 25,
            "contrast": 42,
            "picture_mode": 0,      # Standard
            "color_preset": 5,      # 6500K Warm
            "red_gain": 55,
            "green_gain": 48,
            "blue_gain": 38,
            "gaming_sharpness": 50,
            "overdrive": 0,
            "mprt_aim": 0
        }
    },
    {
        "id": "preset-cinema",
        "name": "Cinema 21:9 Movie",
        "description": "Deep blacks, high dynamic range feel, vibrant color grading for ultrawide films",
        "icon": "Film",
        "built_in": True,
        "badge": "HDR / Cinema",
        "color": "#a855f7",
        "settings": {
            "brightness": 80,
            "contrast": 55,
            "picture_mode": 2,      # Movie
            "color_preset": 6,      # 7500K Normal
            "red_gain": 52,
            "green_gain": 50,
            "blue_gain": 50,
            "gaming_sharpness": 55,
            "overdrive": 1,
            "mprt_aim": 0
        }
    },
    {
        "id": "preset-standard-work",
        "name": "Standard Office / Work",
        "description": "Balanced luminance and neutral 50/50 color balance for all-day coding and work",
        "icon": "Monitor",
        "built_in": True,
        "badge": "Productivity",
        "color": "#10b981",
        "settings": {
            "brightness": 65,
            "contrast": 50,
            "picture_mode": 0,      # Standard
            "color_preset": 6,      # 7500K Normal
            "red_gain": 50,
            "green_gain": 50,
            "blue_gain": 50,
            "gaming_sharpness": 50,
            "overdrive": 1,
            "mprt_aim": 0
        }
    }
]

class ProfileManager:
    def __init__(self, filepath: str = PROFILES_FILE):
        self.filepath = filepath
        self._ensure_file_exists()

    def _ensure_file_exists(self):
        if not os.path.exists(self.filepath):
            self._save_raw(DEFAULT_PROFILES)

    def _load_raw(self) -> List[Dict[str, Any]]:
        try:
            with open(self.filepath, "r", encoding="utf-8") as f:
                return json.load(f)
        except Exception:
            return DEFAULT_PROFILES

    def _save_raw(self, profiles: List[Dict[str, Any]]):
        with open(self.filepath, "w", encoding="utf-8") as f:
            json.dump(profiles, f, indent=2)

    def get_all(self) -> List[Dict[str, Any]]:
        return self._load_raw()

    def get_by_id(self, profile_id: str) -> Optional[Dict[str, Any]]:
        for p in self._load_raw():
            if p.get("id") == profile_id:
                return p
        return None

    def create(self, name: str, description: str, icon: str, settings: Dict[str, int]) -> Dict[str, Any]:
        profiles = self._load_raw()
        new_profile = {
            "id": f"custom-{uuid.uuid4().hex[:8]}",
            "name": name,
            "description": description or "User-defined custom monitor profile",
            "icon": icon or "Sparkles",
            "built_in": False,
            "badge": "Custom",
            "color": "#ec4899",
            "settings": settings
        }
        profiles.append(new_profile)
        self._save_raw(profiles)
        return new_profile

    def delete(self, profile_id: str) -> bool:
        profiles = self._load_raw()
        target = next((p for p in profiles if p.get("id") == profile_id), None)
        if not target or target.get("built_in", False):
            return False  # Cannot delete built-in defaults
        
        filtered = [p for p in profiles if p.get("id") != profile_id]
        self._save_raw(filtered)
        return True
