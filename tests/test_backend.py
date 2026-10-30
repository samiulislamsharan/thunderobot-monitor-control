import sys
import os
import json

# Ensure project root is in path
root_dir = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
sys.path.insert(0, root_dir)

from backend.main import get_monitor_info, get_all_vcp, list_profiles, serve_spa, set_single_vcp, serve_sw, serve_manifest
from backend.schemas import VCPSetRequest

def run_tests():
    print("=== 1. Testing Monitor Metadata ===")
    mon = get_monitor_info()
    print("Detected Model:", mon["model"])
    print("Resolution:", mon["resolution"])
    print("Refresh Rate:", mon["refresh_rate"])
    print("LAN IP:", mon["lan_ip"])
    print("Phone URL:", mon["phone_url"])
    assert mon["model"] == "DQ34C240L", f"Expected DQ34C240L, got {mon['model']}"
    assert mon["refresh_rate"] == 250, f"Expected 250, got {mon['refresh_rate']}"
    assert mon["lan_ip"] != "", "LAN IP should not be empty"

    print("\n=== 2. Testing VCP Features Enumeration ===")
    vcp_data = get_all_vcp()
    features = vcp_data.get("features", {})
    print(f"Total features queried: {len(features)}")
    for feat_name in ["brightness", "contrast", "color_preset", "picture_mode", "input_source"]:
        assert feat_name in features, f"Missing feature: {feat_name}"
        print(f"  {feat_name}: val={features[feat_name]['value']}, hex={features[feat_name]['hex']}")

    print("\n=== 3. Testing Profiles Management ===")
    profs = list_profiles()
    profiles_list = profs.get("profiles", [])
    print(f"Loaded {len(profiles_list)} profile(s):")
    for p in profiles_list:
        print(f"  - [{p['id']}] {p['name']} ({p.get('badge', '')})")
    assert len(profiles_list) >= 4, "Expected at least 4 default presets"

    print("\n=== 4. Testing Static SPA Index.html Serving ===")
    resp = serve_spa("")
    print("SPA response status:", resp.status_code)
    assert resp.status_code == 200, "SPA index.html should be served with 200 OK"

    print("\n=== 5. Testing Non-Destructive Hardware Write & Read-back ===")
    cur_brightness = features["brightness"]["value"]
    print(f"Current Brightness: {cur_brightness}")
    test_val = cur_brightness - 1 if cur_brightness > 0 else 50
    print(f"Setting brightness to: {test_val}...")
    set_single_vcp("brightness", VCPSetRequest(value=test_val))
    
    # Read back to verify
    vcp_after = get_all_vcp()
    new_brightness = vcp_after["features"]["brightness"]["value"]
    print(f"Verified new brightness: {new_brightness}")
    assert new_brightness == test_val, f"Expected {test_val}, got {new_brightness}"

    # Restore original brightness
    print(f"Restoring brightness to: {cur_brightness}...")
    set_single_vcp("brightness", VCPSetRequest(value=cur_brightness))
    vcp_restored = get_all_vcp()
    restored_brightness = vcp_restored["features"]["brightness"]["value"]
    print(f"Verified restored brightness: {restored_brightness}")
    assert restored_brightness == cur_brightness, f"Expected {cur_brightness}, got {restored_brightness}"

    print("\n=== 6. Testing PWA Specification Assets ===")
    manifest_resp = serve_manifest()
    print("Manifest status:", manifest_resp.status_code, "media_type:", manifest_resp.media_type)
    assert manifest_resp.status_code == 200
    assert manifest_resp.media_type == "application/manifest+json"

    sw_resp = serve_sw()
    print("Service Worker status:", sw_resp.status_code, "media_type:", sw_resp.media_type)
    assert sw_resp.status_code == 200
    assert sw_resp.media_type == "application/javascript"
    assert sw_resp.headers.get("Service-Worker-Allowed") == "/"

    icon_resp = serve_spa("icons/icon-192.png")
    print("Icon 192 status:", icon_resp.status_code)
    assert icon_resp.status_code == 200

    icon_maskable_resp = serve_spa("icons/icon-512-maskable.png")
    print("Icon 512 Maskable status:", icon_maskable_resp.status_code)
    assert icon_maskable_resp.status_code == 200

    print("\n==========================================")
    print("[SUCCESS] ALL TESTS PASSED SUCCESSFULLY!")
    print("==========================================")

if __name__ == "__main__":
    run_tests()
