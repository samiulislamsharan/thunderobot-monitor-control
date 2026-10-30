from pydantic import BaseModel, Field
from typing import Optional, Dict, Any, List

class VCPSetRequest(BaseModel):
    value: int = Field(..., description="Target integer value for the VCP code")

class VCPRawRequest(BaseModel):
    code: int = Field(..., description="VCP code in integer or hex (0-255)")
    value: int = Field(..., description="Target value for the VCP code")

class VCPBatchRequest(BaseModel):
    settings: Dict[str, int] = Field(
        ...,
        description="Key-value pairs of feature name or hex code to integer value",
        example={"brightness": 85, "contrast": 50, "picture_mode": 0}
    )

class ProfileCreateRequest(BaseModel):
    name: str = Field(..., min_length=1, max_length=50, description="Display name for preset")
    description: Optional[str] = Field("", max_length=200, description="Optional description")
    icon: Optional[str] = Field("Sliders", description="Icon name (lucide)")
    settings: Dict[str, int] = Field(..., description="VCP settings map")

class ResetRequest(BaseModel):
    reset_type: str = Field(
        ...,
        description="Type of reset: 'luminance' (0x05), 'color' (0x08), or 'factory' (0x04)"
    )

class HotkeyValidateRequest(BaseModel):
    key_combination: str = Field(..., description="Key combination e.g. 'Ctrl+Alt+Up'")
    action_id: Optional[str] = Field(None, description="Current action ID being configured")

class SettingsUpdateRequest(BaseModel):
    autostart: Optional[bool] = None
    close_to_tray: Optional[bool] = None
    start_minimized: Optional[bool] = None
    default_profile: Optional[str] = None
    i2c_delay_ms: Optional[int] = None
    telemetry_interval_sec: Optional[int] = None
    phone_remote_enabled: Optional[bool] = None
    phone_remote_port: Optional[int] = None
    hotkeys: Optional[List[Dict[str, Any]]] = None
