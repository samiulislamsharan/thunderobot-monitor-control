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
