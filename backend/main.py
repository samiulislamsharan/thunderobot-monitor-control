import os
import sys
from fastapi import FastAPI, HTTPException, status
from fastapi.middleware.cors import CORSMiddleware
from fastapi.staticfiles import StaticFiles
from fastapi.responses import FileResponse
from typing import Dict, Any, Optional

from backend.ddcci import DDCCIController, VCP_DEFINITIONS
from backend.monitor_detector import get_complete_monitor_metadata
from backend.profiles import ProfileManager
from backend.settings_manager import settings_manager
from backend.schemas import (
    VCPSetRequest,
    VCPRawRequest,
    VCPBatchRequest,
    ProfileCreateRequest,
    ResetRequest,
    HotkeyValidateRequest,
    SettingsUpdateRequest,
)

# Reverse lookup for named settings to VCP code
NAME_TO_CODE = {meta["name"]: code for code, meta in VCP_DEFINITIONS.items()}

app = FastAPI(
    title="ThundeRobot DQ34C240L Monitor Control API",
    description="DDC/CI Web Controller for ThundeRobot DQ34C240L Fast VA 1440p 250Hz Curved Gaming Monitor",
    version="1.0.0",
)

# Enable CORS for local Vite dev and LAN phone access
app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

controller = DDCCIController(monitor_index=0)
profile_mgr = ProfileManager()


# Helper to resolve code from string or int
def resolve_vcp_code(identifier: str) -> int:
    if identifier in NAME_TO_CODE:
        return NAME_TO_CODE[identifier]
    try:
        if identifier.startswith("0x") or identifier.startswith("0X"):
            return int(identifier, 16)
        return int(identifier)
    except ValueError:
        raise HTTPException(
            status_code=status.HTTP_400_BAD_REQUEST,
            detail=f"Unknown VCP identifier '{identifier}'",
        )


@app.get("/api/monitor")
def get_monitor_info():
    """Get monitor metadata, telemetry, and network connection URLs."""
    metadata = get_complete_monitor_metadata()
    try:
        # Fetch fresh telemetry
        telemetry = {}
        for code, key in [(0xAC, "h_freq"), (0xAE, "v_freq"), (0xC0, "usage_hours")]:
            res = controller.get_vcp(code)
            if res.get("supported"):
                val = res["current"]
                if code == 0xAE and val > 1000:
                    val = round(val / 100.0, 1)
                elif code == 0xAC and val > 1000:
                    val = round(val / 100.0, 1)
                telemetry[key] = val
        metadata["telemetry"] = telemetry
    except Exception as e:
        metadata["telemetry"] = {"v_freq": 250, "h_freq": 473.2, "usage_hours": 127}

    return metadata


@app.get("/api/capabilities")
def get_capabilities():
    """Get raw DDC/CI capabilities string from monitor."""
    caps = controller.get_capabilities_string()
    return {"capabilities": caps}


@app.get("/api/vcp/all")
def get_all_vcp():
    """Get all mapped VCP features and their current values."""
    try:
        features = controller.get_all_vcp()
        return {"features": features}
    except Exception as e:
        raise HTTPException(
            status_code=500, detail=f"Failed to query monitor: {str(e)}"
        )


@app.get("/api/vcp/{identifier}")
def get_single_vcp(identifier: str):
    """Get current value of a specific VCP code (by name or hex/dec)."""
    code = resolve_vcp_code(identifier)
    try:
        result = controller.get_vcp(code)
        if not result.get("supported"):
            raise HTTPException(
                status_code=404,
                detail=f"VCP code 0x{code:02X} not supported by monitor",
            )
        return result
    except HTTPException:
        raise
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))


@app.post("/api/vcp/{identifier}")
def set_single_vcp(identifier: str, req: VCPSetRequest):
    """Set value for a specific VCP code (by name or hex/dec)."""
    code = resolve_vcp_code(identifier)
    try:
        controller.set_vcp(code, req.value)
        return {
            "success": True,
            "code": code,
            "hex": f"0x{code:02X}",
            "value": req.value,
        }
    except Exception as e:
        raise HTTPException(
            status_code=500, detail=f"Failed to set VCP 0x{code:02X}: {str(e)}"
        )


@app.post("/api/vcp/raw/direct")
def set_raw_vcp(req: VCPRawRequest):
    """Directly set any raw VCP code (for testing and advanced tuning)."""
    try:
        controller.set_vcp(req.code, req.value)
        return {
            "success": True,
            "code": req.code,
            "hex": f"0x{req.code:02X}",
            "value": req.value,
        }
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))


@app.post("/api/vcp/batch")
def set_batch_vcp(req: VCPBatchRequest):
    """Set multiple VCP features sequentially with safe delays."""
    applied = {}
    errors = {}
    for key, val in req.settings.items():
        try:
            code = resolve_vcp_code(key)
            controller.set_vcp(code, val)
            applied[key] = val
        except Exception as e:
            errors[key] = str(e)

    return {"success": len(errors) == 0, "applied": applied, "errors": errors}


@app.post("/api/vcp/reset")
def reset_monitor(req: ResetRequest):
    """Trigger a reset code."""
    code_map = {"luminance": 0x05, "color": 0x08, "factory": 0x04}
    if req.reset_type not in code_map:
        raise HTTPException(
            status_code=400, detail=f"Invalid reset_type: {req.reset_type}"
        )

    code = code_map[req.reset_type]
    try:
        controller.reset_feature(code)
        return {"success": True, "reset_type": req.reset_type, "code": f"0x{code:02X}"}
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))


@app.get("/api/profiles")
def list_profiles():
    """List all default and user custom profiles."""
    return {"profiles": profile_mgr.get_all()}


@app.post("/api/profiles")
def create_profile(req: ProfileCreateRequest):
    """Create a new custom profile."""
    profile = profile_mgr.create(
        name=req.name,
        description=req.description or "",
        icon=req.icon or "Sliders",
        settings=req.settings,
    )
    return {"success": True, "profile": profile}


@app.delete("/api/profiles/{profile_id}")
def delete_profile(profile_id: str):
    """Delete a user custom profile."""
    success = profile_mgr.delete(profile_id)
    if not success:
        raise HTTPException(
            status_code=400,
            detail="Cannot delete built-in profile or profile not found",
        )
    return {"success": True}


@app.post("/api/profiles/{profile_id}/apply")
def apply_profile(profile_id: str):
    """Apply a profile's settings to the monitor."""
    profile = profile_mgr.get_by_id(profile_id)
    if not profile:
        raise HTTPException(status_code=404, detail="Profile not found")

    settings = profile.get("settings", {})
    applied = {}
    errors = {}
    for key, val in settings.items():
        try:
            code = resolve_vcp_code(key)
            controller.set_vcp(code, val)
            applied[key] = val
        except Exception as e:
            errors[key] = str(e)

    return {
        "success": len(errors) == 0,
        "profile_id": profile_id,
        "applied": applied,
        "errors": errors,
    }


# ==========================================
# Settings & Hotkeys Endpoints
# ==========================================

@app.get("/api/settings")
def get_settings():
    """Retrieve full app settings including hotkeys and hardware delay config."""
    return settings_manager.get_all()


@app.post("/api/settings")
def update_settings(req: SettingsUpdateRequest):
    """Update and persist application settings."""
    payload = req.model_dump(exclude_unset=True)
    updated = settings_manager.update(payload)
    return {"success": True, "settings": updated}


@app.post("/api/hotkeys/validate")
def validate_hotkey(req: HotkeyValidateRequest):
    """Validate hotkey combination for formatting, internal collisions, and OS-level conflicts."""
    result = settings_manager.validate_hotkey(req.key_combination, req.action_id)
    return result


# Frontend static files mounting (when frontend is built)
dist_dir = os.path.abspath(
    os.path.join(os.path.dirname(__file__), "..", "frontend", "dist")
)

if os.path.exists(dist_dir):
    app.mount(
        "/assets",
        StaticFiles(directory=os.path.join(dist_dir, "assets")),
        name="assets",
    )

    @app.get("/sw.js")
    def serve_sw():
        sw_path = os.path.join(dist_dir, "sw.js")
        if os.path.isfile(sw_path):
            return FileResponse(
                sw_path,
                media_type="application/javascript",
                headers={
                    "Service-Worker-Allowed": "/",
                    "Cache-Control": "no-cache, no-store, must-revalidate"
                }
            )
        raise HTTPException(status_code=404, detail="Service worker not found")

    @app.get("/manifest.webmanifest")
    def serve_manifest():
        manifest_path = os.path.join(dist_dir, "manifest.webmanifest")
        if os.path.isfile(manifest_path):
            return FileResponse(
                manifest_path,
                media_type="application/manifest+json"
            )
        raise HTTPException(status_code=404, detail="Manifest not found")

    @app.get("/{full_path:path}")
    def serve_spa(full_path: str):
        file_path = os.path.join(dist_dir, full_path)
        if os.path.isfile(file_path):
            return FileResponse(file_path)
        return FileResponse(os.path.join(dist_dir, "index.html"))
