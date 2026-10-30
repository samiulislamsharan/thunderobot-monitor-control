import { ref, reactive, computed } from "vue";

const monitorInfo = ref({
  model: "DQ34C240L",
  vendor: "ThundeRobot",
  full_name: "ThundeRobot DQ34C240L 34″ Fast VA 250Hz Curved Gaming Monitor",
  resolution: "3440x1440",
  refresh_rate: 250,
  curved: true,
  panel_type: "Fast VA",
  aspect_ratio: "21:9 Ultrawide",
  lan_ip: "127.0.0.1",
  port: 8000,
  local_url: "http://localhost:8000",
  phone_url: "http://localhost:8000",
  telemetry: {
    v_freq: 250,
    h_freq: 473.2,
    usage_hours: 127,
  },
});

const features = reactive({
  brightness: 95,
  contrast: 45,
  picture_mode: 0,
  color_preset: 11,
  red_gain: 50,
  green_gain: 50,
  blue_gain: 50,
  red_black_level: 50,
  green_black_level: 50,
  blue_black_level: 50,
  input_source: 15,
  orientation: 1,
  gaming_sharpness: 50,
  overdrive: 0,
  mprt_aim: 0,
  power_mode: 1,
});

const featureMeta = ref({});
const profiles = ref([]);
const activeProfileId = ref(null);
const loading = ref(true);
const updating = ref(false);
const connected = ref(false);
const toasts = ref([]);

// Debounce timer map for each feature
const debounceTimers = new Map();

export function useMonitor() {
  const addToast = (message, type = "info", duration = 3000) => {
    const id = Date.now() + Math.random();
    toasts.value.push({ id, message, type });
    setTimeout(() => {
      toasts.value = toasts.value.filter((t) => t.id !== id);
    }, duration);
  };

  const fetchMonitorInfo = async () => {
    try {
      const res = await fetch("/api/monitor");
      if (res.ok) {
        monitorInfo.value = await res.json();
        connected.value = true;
      }
    } catch (e) {
      console.warn("Failed to fetch monitor info:", e);
      connected.value = false;
    }
  };

  const fetchFeatures = async () => {
    try {
      const res = await fetch("/api/vcp/all");
      if (res.ok) {
        const data = await res.json();
        featureMeta.value = data.features || {};
        for (const [key, item] of Object.entries(data.features || {})) {
          if (key in features && item.value !== undefined) {
            features[key] = item.value;
          }
        }
        connected.value = true;
      }
    } catch (e) {
      console.warn("Failed to fetch VCP features:", e);
      connected.value = false;
    } finally {
      loading.value = false;
    }
  };

  const fetchProfiles = async () => {
    try {
      const res = await fetch("/api/profiles");
      if (res.ok) {
        const data = await res.json();
        profiles.value = data.profiles || [];
      }
    } catch (e) {
      console.warn("Failed to fetch profiles:", e);
    }
  };

  const setFeature = (name, value, debounceMs = 60) => {
    // 1. Optimistic local update
    features[name] = value;
    activeProfileId.value = null; // Changed manually from active preset

    // 2. Clear pending debounce timer for this feature
    if (debounceTimers.has(name)) {
      clearTimeout(debounceTimers.get(name));
    }

    // 3. Set debounced dispatch
    const timer = setTimeout(async () => {
      updating.value = true;
      try {
        const res = await fetch(`/api/vcp/${name}`, {
          method: "POST",
          headers: { "Content-Type": "application/json" },
          body: JSON.stringify({ value }),
        });
        if (!res.ok) {
          const err = await res.json();
          addToast(
            `Failed to set ${name}: ${err.detail || "Hardware error"}`,
            "error",
          );
        }
      } catch (e) {
        addToast(`Network error setting ${name}`, "error");
      } finally {
        updating.value = false;
        debounceTimers.delete(name);
      }
    }, debounceMs);

    debounceTimers.set(name, timer);
  };

  const applyProfile = async (profileId) => {
    try {
      updating.value = true;
      const res = await fetch(`/api/profiles/${profileId}/apply`, {
        method: "POST",
      });
      if (res.ok) {
        activeProfileId.value = profileId;
        const profile = profiles.value.find((p) => p.id === profileId);
        if (profile && profile.settings) {
          Object.assign(features, profile.settings);
        }
        addToast(`Applied preset: ${profile?.name || "Custom"}`, "success");
      } else {
        addToast("Failed to apply preset", "error");
      }
    } catch (e) {
      addToast("Error applying preset", "error");
    } finally {
      updating.value = false;
    }
  };

  const createProfile = async (name, description, icon) => {
    try {
      const settingsSnapshot = {
        brightness: features.brightness,
        contrast: features.contrast,
        picture_mode: features.picture_mode,
        color_preset: features.color_preset,
        red_gain: features.red_gain,
        green_gain: features.green_gain,
        blue_gain: features.blue_gain,
        gaming_sharpness: features.gaming_sharpness,
        overdrive: features.overdrive,
        mprt_aim: features.mprt_aim,
      };

      const res = await fetch("/api/profiles", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          name,
          description,
          icon,
          settings: settingsSnapshot,
        }),
      });

      if (res.ok) {
        const data = await res.json();
        profiles.value.push(data.profile);
        activeProfileId.value = data.profile.id;
        addToast(`Profile "${name}" created!`, "success");
        return true;
      }
    } catch (e) {
      addToast("Error saving profile", "error");
    }
    return false;
  };

  const deleteProfile = async (profileId) => {
    try {
      const res = await fetch(`/api/profiles/${profileId}`, {
        method: "DELETE",
      });
      if (res.ok) {
        profiles.value = profiles.value.filter((p) => p.id !== profileId);
        if (activeProfileId.value === profileId) {
          activeProfileId.value = null;
        }
        addToast("Preset deleted", "info");
      }
    } catch (e) {
      addToast("Failed to delete preset", "error");
    }
  };

  const resetMonitor = async (resetType) => {
    try {
      const res = await fetch("/api/vcp/reset", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ reset_type: resetType }),
      });
      if (res.ok) {
        addToast(`Reset executed (${resetType})`, "success");
        // Refresh features after reset
        setTimeout(fetchFeatures, 1000);
      } else {
        addToast("Reset command failed", "error");
      }
    } catch (e) {
      addToast("Reset error", "error");
    }
  };

  const setRawVcp = async (code, value) => {
    try {
      const res = await fetch("/api/vcp/raw/direct", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ code: Number(code), value: Number(value) }),
      });
      if (res.ok) {
        addToast(
          `VCP 0x${Number(code).toString(16).toUpperCase()} set to ${value}`,
          "success",
        );
        return true;
      } else {
        const err = await res.json();
        addToast(`Raw write failed: ${err.detail}`, "error");
        return false;
      }
    } catch (e) {
      addToast("Network error during raw write", "error");
      return false;
    }
  };

  const init = async () => {
    await fetchMonitorInfo();
    await fetchFeatures();
    await fetchProfiles();
  };

  return {
    monitorInfo,
    features,
    featureMeta,
    profiles,
    activeProfileId,
    loading,
    updating,
    connected,
    toasts,
    addToast,
    fetchMonitorInfo,
    fetchFeatures,
    fetchProfiles,
    setFeature,
    applyProfile,
    createProfile,
    deleteProfile,
    resetMonitor,
    setRawVcp,
    init,
  };
}
