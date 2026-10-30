# ThundeRobot DQ34C240L DDC/CI Web Control Suite

A modern, high-performance web application to control your **ThundeRobot DQ34C240L 34″ Fast VA 1440p 250Hz Curved Gaming Monitor** directly from your web browser or smartphone over local Wi-Fi, completely replacing the awkward physical joystick OSD button.

![ThundeRobot DQ34C240L](https://img.shields.io/badge/Monitor-DQ34C240L%2034″-00f0ff?style=for-the-badge)
![Refresh Rate](https://img.shields.io/badge/Refresh%20Rate-250Hz%20Overclock-a855f7?style=for-the-badge)
![Vue.js 3](https://img.shields.io/badge/Frontend-Vue.js%203%20%2B%20Tailwind-10b981?style=for-the-badge)
![FastAPI](https://img.shields.io/badge/Backend-FastAPI%20%2B%20DXVA2-38bdf8?style=for-the-badge)

---

## Features

- **Direct Hardware DDC/CI Control**: Native Windows DXVA2 (`dxva2.dll`) communication over DisplayPort/HDMI I2C bus with thread-safe rate-limiting.
- **Picture & Luminance**: Silky-smooth debounced Brightness and Contrast sliders with live numerical indicators, quick-step buttons, and factory reset.
- **Calibrated Picture Modes**: Instant switching between **Standard**, **Movie**, **Game**, **FPS**, and **RTS** modes (`0xDC`).
- **Color Calibration**: Color temperature presets (6500K Warm, 7500K Normal, 9300K Cool, User RGB) with fine-grained Red, Green, and Blue Gain sliders.
- **Gaming & 250Hz Tuner**: Pixel Overdrive tuning (Levels 0–5), Sharpness / Black Equalizer (`0x3E`), and 1ms MPRT Motion Blur Strobe toggle (`0xE3`).
- **Input Source Switching**: Switch between DisplayPort 1 (`0x0F`), DisplayPort 2 (`0x10`), HDMI 1 (`0x11`), and HDMI 2 (`0x12`).
- **One-Click Presets & Custom Profiles**: Built-in esports FPS, Night Mode (Low Blue Light), Cinema, and Standard presets, plus the ability to snapshot and save custom presets.
- **📱 Phone Remote over Wi-Fi**:
  - Automatically binds to `0.0.0.0:8000` on your local network.
  - Generates an in-browser **QR Code** for instant camera scanning from your phone.
  - Fully touch-optimized responsive layout.
  - Tap **"Add to Home Screen"** on iOS or Android for a full-screen, native PWA app experience!
- **Power Management & Safety**: Sleep / Standby (`0x04`) and Power Off (`0x05`) with double-confirmation dialogs.
- **Advanced Hex VCP Inspector**: Direct hex register reader and writer for experimenting with any MCCS / MStar DDC/CI opcode.

---

## Quick Start

### Option 1: Double-Click Launcher (Windows)
Simply double-click **`start.bat`** in the project directory.

### Option 2: Python Command Line
```powershell
python run.py
```

The script will automatically:
1. Verify backend dependencies and compile the Vue 3 frontend with Vite (if not already built).
2. Detect your local LAN IP address (e.g. `192.168.50.143`).
3. Launch the server on `0.0.0.0:8000`.
4. Open your default browser to `http://localhost:8000`.

---

## 📱 Controlling from Your Smartphone

1. Ensure your smartphone is connected to the same Wi-Fi network as your PC.
2. In the desktop web application header, click **"📱 Phone Remote"**.
3. Scan the displayed **QR Code** using your phone's camera.
4. *(Optional)* In your phone's browser, tap **"Add to Home Screen"** to install it as a standalone app icon.

---

## Reverse-Engineered DDC/CI VCP Map for ThundeRobot DQ34C240L

| VCP Code | Name | Range / Values | Type | Description |
|---|---|---|---|---|
| **0x10** | Luminance | 0 – 100 | Continuous | Backlight brightness |
| **0x12** | Contrast | 0 – 100 | Continuous | Contrast ratio |
| **0x14** | Color Temperature | `0x05`, `0x06`, `0x08`, `0x0B` | Discrete | 6500K, 7500K, 9300K, User RGB |
| **0x16** | Red Gain | 0 – 100 | Continuous | Red channel multiplier |
| **0x18** | Green Gain | 0 – 100 | Continuous | Green channel multiplier |
| **0x1A** | Blue Gain | 0 – 100 | Continuous | Blue channel multiplier |
| **0x6C** | Red Black Level | 0 – 255 | Continuous | Red shadow offset |
| **0x6E** | Green Black Level | 0 – 255 | Continuous | Green shadow offset |
| **0x70** | Blue Black Level | 0 – 255 | Continuous | Blue shadow offset |
| **0xDC** | Picture Mode | `0x00`, `0x02`, `0x03`, `0x05`, `0x07` | Discrete | Standard, Movie, Game, FPS, RTS |
| **0x60** | Input Select | `0x0F`, `0x10`, `0x11`, `0x12` | Discrete | DP1 (250Hz), DP2, HDMI1, HDMI2 |
| **0xAA** | Orientation | `0x01`, `0x02` | Discrete | Landscape (0°), Portrait (90°) |
| **0xD6** | Power Mode | `0x01`, `0x04`, `0x05` | Action | Power On, Standby, Power Off |
| **0x3E** | Sharpness / Black EQ | 0 – 100 | Continuous | Shadow clarity tuning |
| **0x68** | Overdrive | 0 – 5 | Discrete | Pixel response acceleration |
| **0xE3** | MPRT / Aim Assist | 0 – 1 | Toggle | 1ms motion blur strobe |
| **0xAC** | Horizontal Frequency | Read-only | Telemetry | Current line scan frequency |
| **0xAE** | Vertical Frequency | Read-only | Telemetry | Live refresh rate (~250 Hz) |
| **0xC0** | Power-On Hours | Read-only | Telemetry | Total backlight operating hours |
| **0x05** | Reset Luminance | Trigger | Action | Restore factory brightness/contrast |
| **0x08** | Reset Color | Trigger | Action | Restore factory color gains |
| **0x04** | Factory Reset | Trigger | Action | Restore all factory defaults |

---

## Project Structure

```
thunderobot-monitor-control/
├── PLAN.md                   # Full architecture and DDC/CI specification
├── README.md                 # Project guide and documentation
├── run.py                    # Main launcher (checks, builds & runs server)
├── start.bat                 # 1-click Windows runner
├── backend/
│   ├── ddcci.py              # Win32 DXVA2 ctypes engine & I2C rate limiter
│   ├── monitor_detector.py   # EDID, model resolution & LAN IP resolver
│   ├── profiles.py           # Presets manager (persists to profiles.json)
│   ├── schemas.py            # Pydantic request & response models
│   ├── main.py               # FastAPI application & static SPA server
│   └── requirements.txt      # Python dependencies
├── frontend/
│   ├── index.html            # PWA-ready HTML5 entry
│   ├── vite.config.js        # Vite + Vue 3 config with proxy
│   ├── tailwind.config.js    # Cyberpunk/gaming dark theme config
│   ├── src/
│   │   ├── main.js           # Vue 3 entry point
│   │   ├── App.vue           # Root layout & modal coordinator
│   │   ├── style.css         # Custom range sliders, backdrop blur & scrollbars
│   │   ├── composables/
│   │   │   └── useMonitor.js # Reactive monitor state & debounced updates
│   │   └── components/
│   │       ├── Header.vue            # Live 250Hz badge, telemetry & pairing
│   │       ├── PresetsBar.vue        # 1-click profiles & custom preset cards
│   │       ├── PictureControls.vue   # Brightness, contrast & picture modes
│   │       ├── ColorControls.vue     # Color temperature & RGB gain sliders
│   │       ├── GamingControls.vue    # Overdrive, sharpness/black EQ & MPRT
│   │       ├── InputPower.vue        # Input switching & power management
│   │       ├── RawVcpInspector.vue   # Direct hex VCP reader/writer
│   │       ├── PhonePairModal.vue    # Scannable QR code & LAN URL modal
│   │       ├── ConfirmModal.vue      # HTML5 <dialog> safety confirmation
│   │       └── SaveProfileModal.vue  # Custom preset creator modal
└── tests/
    └── test_backend.py       # Automated hardware & endpoint verification
```

---

## License
MIT
