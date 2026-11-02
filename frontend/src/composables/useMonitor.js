import { ref, reactive, computed } from "vue";
import { api } from "@/api/adapter";

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

const DEFAULT_PRESETS = [
  {
    id: "preset-fps-250",
    name: "Competitive FPS 250Hz",
    description: "Max brightness, FPS mode, fast overdrive, crisp clarity for esports",
    icon: "Crosshair",
    built_in: true,
    badge: "250Hz Esports",
    color: "#00f0ff",
    settings: {
      brightness: 100,
      contrast: 50,
      picture_mode: 5,
      color_preset: 11,
      red_gain: 50,
      green_gain: 50,
      blue_gain: 50,
      gaming_sharpness: 60,
      overdrive: 3,
      mprt_aim: 1,
    },
  },
  {
    id: "preset-night-reading",
    name: "Night Mode / Low Blue",
    description: "Gentle backlight, warm 6500K color temperature to eliminate eye strain",
    icon: "Moon",
    built_in: true,
    badge: "Eye Care",
    color: "#f59e0b",
    settings: {
      brightness: 25,
      contrast: 42,
      picture_mode: 0,
      color_preset: 5,
      red_gain: 55,
      green_gain: 48,
      blue_gain: 38,
      gaming_sharpness: 50,
      overdrive: 0,
      mprt_aim: 0,
    },
  },
  {
    id: "preset-cinema",
    name: "Cinema 21:9 Movie",
    description: "Deep blacks, high dynamic range feel, vibrant color grading for ultrawide films",
    icon: "Film",
    built_in: true,
    badge: "HDR / Cinema",
    color: "#a855f7",
    settings: {
      brightness: 80,
      contrast: 55,
      picture_mode: 2,
      color_preset: 6,
      red_gain: 52,
      green_gain: 50,
      blue_gain: 50,
      gaming_sharpness: 55,
      overdrive: 1,
      mprt_aim: 0,
    },
  },
  {
    id: "preset-standard-work",
    name: "Standard Office / Work",
    description: "Balanced luminance and neutral 50/50 color balance for all-day coding and work",
    icon: "Monitor",
    built_in: true,
    badge: "Productivity",
    color: "#10b981",
    settings: {
      brightness: 65,
      contrast: 50,
      picture_mode: 0,
      color_preset: 6,
      red_gain: 50,
      green_gain: 50,
      blue_gain: 50,
      gaming_sharpness: 50,
      overdrive: 1,
      mprt_aim: 0,
    },
  },
];

const featureMeta = ref({});
const profiles = ref(DEFAULT_PRESETS);
const activeProfileId = ref(null);
const loading = ref(true);
const updating = ref(false);
const connected = ref(false);
const toasts = ref([]);

// Debounce timer map for each feature
const debounceTimers = new Map();

const formatError = (e) => {
  if (!e) return "Unknown error";
  if (typeof e === "string") return e;
  if (e.message) return e.message;
  return JSON.stringify(e);
};

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
      const data = await api.getMonitorInfo();
      monitorInfo.value = data;
      connected.value = true;
    } catch (e) {
      console.warn("Failed to fetch monitor info:", e);
      connected.value = false;
    }
  };

  const fetchFeatures = async () => {
    try {
      const data = await api.getFeatures();
      featureMeta.value = data.features || {};
      for (const [key, item] of Object.entries(data.features || {})) {
        if (key in features && item.value !== undefined) {
          features[key] = item.value;
        }
      }
      connected.value = true;
    } catch (e) {
      console.warn("Failed to fetch VCP features:", e);
      connected.value = false;
    } finally {
      loading.value = false;
    }
  };

  const fetchProfiles = async () => {
    try {
      const data = await api.getProfiles();
      if (data && Array.isArray(data.profiles) && data.profiles.length > 0) {
        profiles.value = data.profiles;
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
        await api.setFeature(name, value);
      } catch (e) {
        addToast(`Failed to set ${name}: ${formatError(e)}`, "error");
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
      await api.applyProfile(profileId);
      activeProfileId.value = profileId;
      const profile = profiles.value.find((p) => p.id === profileId);
      if (profile && profile.settings) {
        Object.assign(features, profile.settings);
      }
      addToast(`Applied preset: ${profile?.name || "Custom"}`, "success");
    } catch (e) {
      addToast(`Error applying preset: ${formatError(e)}`, "error");
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

      const res = await api.createProfile({
        name,
        description,
        icon,
        settings: settingsSnapshot,
      });

      if (res && res.profile) {
        profiles.value.push(res.profile);
        activeProfileId.value = res.profile.id;
        addToast(`Profile "${name}" created!`, "success");
        return true;
      }
    } catch (e) {
      addToast(`Error saving profile: ${formatError(e)}`, "error");
    }
    return false;
  };

  const deleteProfile = async (profileId) => {
    try {
      await api.deleteProfile(profileId);
      profiles.value = profiles.value.filter((p) => p.id !== profileId);
      if (activeProfileId.value === profileId) {
        activeProfileId.value = null;
      }
      addToast("Preset deleted", "info");
    } catch (e) {
      addToast(`Failed to delete preset: ${formatError(e)}`, "error");
    }
  };

  const resetMonitor = async (resetType) => {
    try {
      await api.resetMonitor(resetType);
      addToast(`Reset executed (${resetType})`, "success");
      setTimeout(fetchFeatures, 1000);
    } catch (e) {
      addToast(`Reset error: ${formatError(e)}`, "error");
    }
  };

  const setRawVcp = async (code, value) => {
    try {
      await api.setRawVcp(Number(code), Number(value));
      addToast(
        `VCP 0x${Number(code).toString(16).toUpperCase()} set to ${value}`,
        "success",
      );
      return true;
    } catch (e) {
      addToast(`Raw write failed: ${formatError(e)}`, "error");
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
