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
      profiles.value = data.profiles || [];
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
        addToast(`Failed to set ${name}: ${e.message}`, "error");
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
      addToast(`Error applying preset: ${e.message}`, "error");
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
      addToast("Error saving profile", "error");
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
      addToast("Failed to delete preset", "error");
    }
  };

  const resetMonitor = async (resetType) => {
    try {
      await api.resetMonitor(resetType);
      addToast(`Reset executed (${resetType})`, "success");
      setTimeout(fetchFeatures, 1000);
    } catch (e) {
      addToast("Reset error", "error");
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
      addToast(`Raw write failed: ${e.message}`, "error");
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
