import json
import os
import sys
import ctypes
from typing import Dict, Any, List, Optional, Tuple

SETTINGS_FILE = os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "settings.json")

DEFAULT_HOTKEYS = [
    {
        "id": "brightness_up",
        "action": "Brightness Up (+5%)",
        "key_combination": "Ctrl+Alt+Up",
        "enabled": True,
        "description": "Increase display backlight luminance by 5%"
    },
    {
        "id": "brightness_down",
        "action": "Brightness Down (-5%)",
        "key_combination": "Ctrl+Alt+Down",
        "enabled": True,
        "description": "Decrease display backlight luminance by 5%"
    },
    {
        "id": "contrast_up",
        "action": "Contrast Up (+5%)",
        "key_combination": "Ctrl+Alt+Right",
        "enabled": True,
        "description": "Increase display contrast ratio by 5%"
    },
    {
        "id": "contrast_down",
        "action": "Contrast Down (-5%)",
        "key_combination": "Ctrl+Alt+Left",
        "enabled": True,
        "description": "Decrease display contrast ratio by 5%"
    },
    {
        "id": "preset_fps",
        "action": "Preset: 250Hz Esports",
        "key_combination": "Ctrl+Alt+1",
        "enabled": True,
        "description": "Switch to Competitive 250Hz FPS gaming preset"
    },
    {
        "id": "preset_night",
        "action": "Preset: Night Mode",
        "key_combination": "Ctrl+Alt+2",
        "enabled": True,
        "description": "Switch to Eye-Care Low Blue Light preset"
    },
    {
        "id": "preset_cinema",
        "action": "Preset: Cinema 21:9",
        "key_combination": "Ctrl+Alt+3",
        "enabled": True,
        "description": "Switch to 21:9 Ultrawide Cinema Movie preset"
    },
    {
        "id": "preset_work",
        "action": "Preset: Standard Work",
        "key_combination": "Ctrl+Alt+4",
        "enabled": True,
        "description": "Switch to 6500K calibrated office / design preset"
    },
    {
        "id": "toggle_standby",
        "action": "Toggle Standby / Sleep",
        "key_combination": "Ctrl+Alt+End",
        "enabled": True,
        "description": "Put monitor into DDC/CI power-saving standby"
    },
    {
        "id": "toggle_window",
        "action": "Show / Hide Window",
        "key_combination": "Ctrl+Alt+M",
        "enabled": True,
        "description": "Toggle main controller window visibility"
    }
]

DEFAULT_SETTINGS = {
    "autostart": False,
    "close_to_tray": True,
    "start_minimized": False,
    "default_profile": "preset-fps-250",
    "i2c_delay_ms": 35,
    "telemetry_interval_sec": 3,
    "phone_remote_enabled": True,
    "phone_remote_port": 8000,
    "hotkeys": DEFAULT_HOTKEYS
}

# Win32 Virtual Key mapping for conflict testing
MOD_ALT = 0x0001
MOD_CONTROL = 0x0002
MOD_SHIFT = 0x0004
MOD_WIN = 0x0008

VK_MAP = {
    "UP": 0x26,
    "DOWN": 0x28,
    "LEFT": 0x25,
    "RIGHT": 0x27,
    "PAGEUP": 0x21,
    "PAGEDOWN": 0x22,
    "END": 0x23,
    "HOME": 0x24,
    "INSERT": 0x2D,
    "DELETE": 0x2E,
    "SPACE": 0x20,
    "TAB": 0x09,
    "ESCAPE": 0x1B,
    "ENTER": 0x0D,
    "F1": 0x70, "F2": 0x71, "F3": 0x72, "F4": 0x73,
    "F5": 0x74, "F6": 0x75, "F7": 0x76, "F8": 0x77,
    "F9": 0x78, "F10": 0x79, "F11": 0x7A, "F12": 0x7B,
}

def parse_hotkey(combo_str: str) -> Tuple[int, int]:
    """Parse hotkey string (e.g. 'Ctrl+Alt+Up') into (modifiers, vk_code)."""
    parts = [p.strip().upper() for p in combo_str.split("+") if p.strip()]
    modifiers = 0
    vk = 0
    for part in parts:
        if part in ("CTRL", "CONTROL"):
            modifiers |= MOD_CONTROL
        elif part in ("ALT", "MENU"):
            modifiers |= MOD_ALT
        elif part == "SHIFT":
            modifiers |= MOD_SHIFT
        elif part in ("WIN", "SUPER", "META"):
            modifiers |= MOD_WIN
        elif part in VK_MAP:
            vk = VK_MAP[part]
        elif len(part) == 1:
            vk = ord(part)
        else:
            raise ValueError(f"Unsupported key: {part}")
            
    if vk == 0:
        raise ValueError("Missing non-modifier key in combination")
    return modifiers, vk


class SettingsManager:
    def __init__(self, filepath: str = SETTINGS_FILE):
        self.filepath = os.path.abspath(filepath)
        self.settings: Dict[str, Any] = {}
        self.load()

    def load(self):
        if os.path.exists(self.filepath):
            try:
                with open(self.filepath, "r", encoding="utf-8") as f:
                    self.settings = json.load(f)
            except Exception as e:
                print(f"[SettingsManager] Error loading {self.filepath}: {e}, using defaults")
                self.settings = DEFAULT_SETTINGS.copy()
        else:
            self.settings = DEFAULT_SETTINGS.copy()
            self.save()

        # Merge defaults if missing keys
        for k, v in DEFAULT_SETTINGS.items():
            if k not in self.settings:
                self.settings[k] = v

    def save(self):
        try:
            with open(self.filepath, "w", encoding="utf-8") as f:
                json.dump(self.settings, f, indent=2)
        except Exception as e:
            print(f"[SettingsManager] Failed to save {self.filepath}: {e}")

    def get_all(self) -> Dict[str, Any]:
        return self.settings

    def update(self, new_data: Dict[str, Any]) -> Dict[str, Any]:
        self.settings.update(new_data)
        self.save()
        return self.settings

    def test_os_conflict(self, combo_str: str) -> Tuple[bool, str]:
        """Test Windows OS-level hotkey availability using RegisterHotKey probe."""
        if sys.platform != "win32":
            return True, "Platform does not support Win32 hotkey probe"

        try:
            modifiers, vk = parse_hotkey(combo_str)
        except ValueError as e:
            return False, str(e)

        test_id = 9876
        user32 = ctypes.windll.user32
        
        # Test register
        res = user32.RegisterHotKey(None, test_id, modifiers, vk)
        err = ctypes.GetLastError()
        
        if res:
            # Successfully registered, immediately unregister
            user32.UnregisterHotKey(None, test_id)
            return True, "Available and supported by OS"
        else:
            if err == 1409: # ERROR_HOTKEY_ALREADY_REGISTERED
                return False, "Conflict: Already registered by Windows or another running application (Discord, GeForce, etc.)"
            return False, f"Windows Win32 error code {err}"

    def validate_hotkey(self, combo_str: str, action_id: Optional[str] = None) -> Dict[str, Any]:
        """Validate format, internal duplicates, and OS availability."""
        combo_norm = "+".join(p.strip().title() for p in combo_str.split("+") if p.strip())
        
        # 1. Format check
        try:
            parse_hotkey(combo_str)
        except ValueError as e:
            return {
                "valid": False,
                "status": "invalid_format",
                "message": f"Invalid key combination: {str(e)}",
                "normalized": combo_str
            }

        # 2. Check internal duplicate with other actions
        hotkeys = self.settings.get("hotkeys", [])
        for hk in hotkeys:
            if hk.get("id") != action_id and hk.get("enabled", True):
                other_norm = "+".join(p.strip().title() for p in hk.get("key_combination", "").split("+") if p.strip())
                if other_norm.lower() == combo_norm.lower():
                    return {
                        "valid": False,
                        "status": "internal_conflict",
                        "message": f"Conflict: Already assigned to '{hk.get('action')}'",
                        "conflicting_action": hk.get("action"),
                        "normalized": combo_norm
                    }

        # 3. Test OS / system conflict
        is_os_ok, os_msg = self.test_os_conflict(combo_str)
        if not is_os_ok:
            return {
                "valid": False,
                "status": "system_conflict",
                "message": os_msg,
                "normalized": combo_norm
            }

        return {
            "valid": True,
            "status": "ok",
            "message": "Hotkey is valid and ready to use",
            "normalized": combo_norm
        }

settings_manager = SettingsManager()
