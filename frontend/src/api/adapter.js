/**
 * Universal Hardware & Settings API Adapter
 * Bridges seamlessly between native Tauri IPC invoke() and standard Web REST fetch().
 */

const isTauriEnv = () => {
  return typeof window !== 'undefined' && (
    !!window.__TAURI_INTERNALS__ || 
    !!window.__TAURI__ || 
    !!window.__TAURI_IPC__
  );
};

export const isTauri = isTauriEnv();

// Dynamically import Tauri core invoke if running inside Tauri
let tauriInvoke = null;
if (isTauri) {
  try {
    import('@tauri-apps/api/core').then((module) => {
      tauriInvoke = module.invoke;
    });
  } catch (e) {
    console.warn('[Adapter] Running in Tauri environment but @tauri-apps/api/core load failed:', e);
  }
}

const API_BASE = '/api';

export const api = {
  isTauri,

  async getMonitorInfo() {
    if (isTauri && tauriInvoke) {
      return await tauriInvoke('get_monitor_info');
    }
    const res = await fetch(`${API_BASE}/monitor`);
    if (!res.ok) throw new Error('Failed to fetch monitor info');
    return await res.json();
  },

  async getFeatures() {
    if (isTauri && tauriInvoke) {
      return await tauriInvoke('get_features');
    }
    const res = await fetch(`${API_BASE}/features`);
    if (!res.ok) throw new Error('Failed to fetch VCP features');
    return await res.json();
  },

  async setFeature(name, value, vcpType = 0) {
    if (isTauri && tauriInvoke) {
      return await tauriInvoke('set_feature', { name, value, vcpType });
    }
    const res = await fetch(`${API_BASE}/features/${name}`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ value }),
    });
    if (!res.ok) throw new Error(`Failed to set feature ${name}`);
    return await res.json();
  },

  async getProfiles() {
    if (isTauri && tauriInvoke) {
      return await tauriInvoke('get_profiles');
    }
    const res = await fetch(`${API_BASE}/profiles`);
    if (!res.ok) throw new Error('Failed to fetch profiles');
    return await res.json();
  },

  async applyProfile(profileId) {
    if (isTauri && tauriInvoke) {
      return await tauriInvoke('apply_profile', { profileId });
    }
    const res = await fetch(`${API_BASE}/profiles/${profileId}/apply`, {
      method: 'POST',
    });
    if (!res.ok) throw new Error(`Failed to apply profile ${profileId}`);
    return await res.json();
  },

  async createProfile(data) {
    if (isTauri && tauriInvoke) {
      return await tauriInvoke('create_profile', { profile: data });
    }
    const res = await fetch(`${API_BASE}/profiles`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(data),
    });
    if (!res.ok) throw new Error('Failed to create profile');
    return await res.json();
  },

  async deleteProfile(profileId) {
    if (isTauri && tauriInvoke) {
      return await tauriInvoke('delete_profile', { profileId });
    }
    const res = await fetch(`${API_BASE}/profiles/${profileId}`, {
      method: 'DELETE',
    });
    if (!res.ok) throw new Error('Failed to delete profile');
    return await res.json();
  },

  async resetMonitor(resetType) {
    if (isTauri && tauriInvoke) {
      return await tauriInvoke('reset_monitor', { resetType });
    }
    const res = await fetch(`${API_BASE}/reset`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ reset_type: resetType }),
    });
    if (!res.ok) throw new Error('Failed to reset monitor');
    return await res.json();
  },

  async setRawVcp(code, value) {
    if (isTauri && tauriInvoke) {
      return await tauriInvoke('set_raw_vcp', { code, value });
    }
    const res = await fetch(`${API_BASE}/vcp/raw`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ code, value }),
    });
    if (!res.ok) throw new Error('Failed to set raw VCP');
    return await res.json();
  },

  async getSettings() {
    if (isTauri && tauriInvoke) {
      return await tauriInvoke('get_settings');
    }
    const res = await fetch(`${API_BASE}/settings`);
    if (!res.ok) throw new Error('Failed to fetch settings');
    return await res.json();
  },

  async saveSettings(data) {
    if (isTauri && tauriInvoke) {
      return await tauriInvoke('save_settings', { settings: data });
    }
    const res = await fetch(`${API_BASE}/settings`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(data),
    });
    if (!res.ok) throw new Error('Failed to save settings');
    return await res.json();
  },

  async validateHotkey(keyCombination, actionId = null) {
    if (isTauri && tauriInvoke) {
      return await tauriInvoke('validate_hotkey', { keyCombination, actionId });
    }
    const res = await fetch(`${API_BASE}/hotkeys/validate`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({
        key_combination: keyCombination,
        action_id: actionId,
      }),
    });
    if (!res.ok) throw new Error('Failed to validate hotkey');
    return await res.json();
  },
};
