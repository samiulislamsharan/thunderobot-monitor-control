import socket
import winreg
import ctypes
from ctypes import wintypes
from typing import Dict, Any, Optional

user32 = ctypes.windll.user32

class DEVMODEW(ctypes.Structure):
    _fields_ = [
        ('dmDeviceName', wintypes.WCHAR * 32),
        ('dmSpecVersion', wintypes.WORD),
        ('dmDriverVersion', wintypes.WORD),
        ('dmSize', wintypes.WORD),
        ('dmDriverExtra', wintypes.WORD),
        ('dmFields', wintypes.DWORD),
        ('dmOrientation', ctypes.c_short),
        ('dmPaperSize', ctypes.c_short),
        ('dmPaperLength', ctypes.c_short),
        ('dmPaperWidth', ctypes.c_short),
        ('dmScale', ctypes.c_short),
        ('dmCopies', ctypes.c_short),
        ('dmDefaultSource', ctypes.c_short),
        ('dmPrintQuality', ctypes.c_short),
        ('dmColor', ctypes.c_short),
        ('dmDuplex', ctypes.c_short),
        ('dmYResolution', ctypes.c_short),
        ('dmTTOption', ctypes.c_short),
        ('dmCollate', ctypes.c_short),
        ('dmFormName', wintypes.WCHAR * 32),
        ('dmLogPixels', wintypes.WORD),
        ('dmBitsPerPel', wintypes.DWORD),
        ('dmPelsWidth', wintypes.DWORD),
        ('dmPelsHeight', wintypes.DWORD),
        ('dmDisplayFlags', wintypes.DWORD),
        ('dmDisplayFrequency', wintypes.DWORD),
        ('dmICMMethod', wintypes.DWORD),
        ('dmICMIntent', wintypes.DWORD),
        ('dmMediaType', wintypes.DWORD),
        ('dmDitherType', wintypes.DWORD),
        ('dmReserved1', wintypes.DWORD),
        ('dmReserved2', wintypes.DWORD),
        ('dmPanningWidth', wintypes.DWORD),
        ('dmPanningHeight', wintypes.DWORD),
    ]

ENUM_CURRENT_SETTINGS = -1

def get_lan_ip() -> str:
    """Detect the local machine's LAN IP address reachable by Wi-Fi/Ethernet."""
    s = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
    try:
        # Route lookup without sending actual traffic
        s.connect(('10.255.255.255', 1))
        ip = s.getsockname()[0]
    except Exception:
        try:
            ip = socket.gethostbyname(socket.gethostname())
        except Exception:
            ip = '127.0.0.1'
    finally:
        s.close()
    return ip

def get_windows_display_mode() -> Dict[str, Any]:
    """Retrieve current Windows display mode (resolution, refresh rate)."""
    dm = DEVMODEW()
    dm.dmSize = ctypes.sizeof(DEVMODEW)
    if user32.EnumDisplaySettingsW(None, ENUM_CURRENT_SETTINGS, ctypes.byref(dm)):
        return {
            "width": dm.dmPelsWidth,
            "height": dm.dmPelsHeight,
            "refresh_rate": dm.dmDisplayFrequency,
            "bits_per_pel": dm.dmBitsPerPel
        }
    return {
        "width": 3440,
        "height": 1440,
        "refresh_rate": 250,
        "bits_per_pel": 32
    }

def get_monitor_edid_info() -> Dict[str, Any]:
    """Read EDID strings from Windows Registry to find model name and vendor."""
    key_path = r"SYSTEM\CurrentControlSet\Enum\DISPLAY"
    monitor_info = {
        "model": "DQ34C240L",
        "vendor": "ThundeRobot",
        "full_name": "ThundeRobot DQ34C240L 34″ Fast VA 250Hz Curved Gaming Monitor",
        "device_id": "",
        "edid_found": False
    }

    try:
        display_key = winreg.OpenKey(winreg.HKEY_LOCAL_MACHINE, key_path)
    except Exception:
        return monitor_info

    subkeys_count = winreg.QueryInfoKey(display_key)[0]
    for i in range(subkeys_count):
        try:
            device_id = winreg.EnumKey(display_key, i)
            device_key = winreg.OpenKey(display_key, device_id)
            inst_count = winreg.QueryInfoKey(device_key)[0]
            for j in range(inst_count):
                inst_id = winreg.EnumKey(device_key, j)
                inst_key = winreg.OpenKey(device_key, inst_id)
                try:
                    device_params = winreg.OpenKey(inst_key, "Device Parameters")
                    edid, _ = winreg.QueryValueEx(device_params, "EDID")
                    # Parse EDID for ASCII name descriptor (tag 0xFC)
                    for desc_offset in (54, 72, 90, 108):
                        block = edid[desc_offset:desc_offset + 18]
                        if block[0:3] == b'\x00\x00\x00' and block[3] == 0xFC:
                            name = block[5:].split(b'\x0a')[0].decode('latin1', errors='ignore').strip()
                            if "DQ34" in name or "TRG" in device_id or "240" in name:
                                monitor_info["model"] = name or "DQ34C240L"
                                monitor_info["device_id"] = f"{device_id}\\{inst_id}"
                                monitor_info["edid_found"] = True
                                return monitor_info
                except Exception:
                    pass
        except Exception:
            pass

    return monitor_info

def get_complete_monitor_metadata(port: int = 8000) -> Dict[str, Any]:
    """Aggregate hardware, display mode, and network pairing endpoints."""
    lan_ip = get_lan_ip()
    edid = get_monitor_edid_info()
    display = get_windows_display_mode()

    return {
        "model": edid["model"],
        "vendor": edid["vendor"],
        "full_name": edid["full_name"],
        "resolution": f"{display['width']}x{display['height']}",
        "refresh_rate": display["refresh_rate"],
        "curved": True,
        "panel_type": "Fast VA",
        "aspect_ratio": "21:9 Ultrawide",
        "lan_ip": lan_ip,
        "port": port,
        "local_url": f"http://localhost:{port}",
        "phone_url": f"http://{lan_ip}:{port}"
    }
