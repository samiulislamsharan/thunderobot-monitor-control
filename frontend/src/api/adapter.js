import { invoke } from '@tauri-apps/api/core';

/**
 * Universal Hardware & Settings API Adapter
 * Bridges seamlessly between native Tauri IPC invoke() and standard Web REST fetch().
 */

const isTauriEnv = () => {
  return (
    typeof window !== 'undefined' &&
    (!!window.__TAURI_INTERNALS__ ||
      !!window.__TAURI__ ||
      !!window.__TAURI_IPC__)
  );
};

export const isTauri = isTauriEnv();
const API_BASE = '/api';

export const api = {
  isTauri,

  async getMonitorInfo() {
    if (isTauri) {
      try {
        return await invoke('get_monitor_info');
      } catch (e) {
        console.error('[Adapter] invoke get_monitor_info error:', e);
      }
    }
    const res = await fetch(`${API_BASE}/monitor`);
    if (!res.ok) throw new Error('Failed to fetch monitor info');
    return await res.json();
  },

  async getFeatures() {
    if (isTauri) {
      try {
        return await invoke('get_features');
      } catch (e) {
        console.error('[Adapter] invoke get_features error:', e);
      }
    }
    const res = await fetch(`${API_BASE}/vcp/all`);
    if (!res.ok) throw new Error('Failed to fetch VCP features');
    return await res.json();
  },

  async setFeature(name, value, vcpType = 0) {
    if (isTauri) {
      return await invoke('set_feature', {
        name: String(name),
        code: name,
        value: Number(value),
        vcpType,
      });
    }
    const res = await fetch(`${API_BASE}/vcp/${name}`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ value: Number(value) }),
    });
    if (!res.ok) {
      const err = await res.json().catch(() => ({}));
      throw new Error(err.detail || `Failed to set feature ${name}`);
    }
    return await res.json();
  },

  async setRawVcp(code, value) {
    if (isTauri) {
      return await invoke('set_raw_vcp', {
        code: Number(code),
        opcode: Number(code),
        value: Number(value),
      });
    }
    const res = await fetch(`${API_BASE}/vcp/raw/direct`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ code: Number(code), value: Number(value) }),
    });
    if (!res.ok) {
      const err = await res.json().catch(() => ({}));
      throw new Error(
        err.detail || `Failed to set raw VCP 0x${Number(code).toString(16)}`
      );
    }
    return await res.json();
  },

  async resetMonitor(resetType = 'all') {
    if (isTauri) {
      return await invoke('reset_monitor', {
        resetType,
        reset_type: resetType,
      });
    }
    const res = await fetch(`${API_BASE}/vcp/reset`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ reset_type: resetType }),
    });
    if (!res.ok) {
      const err = await res.json().catch(() => ({}));
      throw new Error(err.detail || 'Failed to reset monitor');
    }
    return await res.json();
  },

  async getProfiles() {
    if (isTauri) {
      try {
        return await invoke('get_profiles');
      } catch (e) {
        console.error('[Adapter] invoke get_profiles error:', e);
      }
    }
    const res = await fetch(`${API_BASE}/profiles`);
    if (!res.ok) throw new Error('Failed to fetch profiles');
    return await res.json();
  },

  async applyProfile(profileId) {
    if (isTauri) {
      return await invoke('apply_profile', {
        profileId,
        profile_id: profileId,
      });
    }
    const res = await fetch(`${API_BASE}/profiles/${profileId}/apply`, {
      method: 'POST',
    });
    if (!res.ok) {
      const err = await res.json().catch(() => ({}));
      throw new Error(err.detail || `Failed to apply profile ${profileId}`);
    }
    return await res.json();
  },

  async createProfile(data) {
    if (isTauri) {
      return await invoke('create_profile', { profile: data });
    }
    const res = await fetch(`${API_BASE}/profiles`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(data),
    });
    if (!res.ok) {
      const err = await res.json().catch(() => ({}));
      throw new Error(err.detail || 'Failed to create profile');
    }
    return await res.json();
  },

  async deleteProfile(profileId) {
    if (isTauri) {
      return await invoke('delete_profile', {
        profileId,
        profile_id: profileId,
      });
    }
    const res = await fetch(`${API_BASE}/profiles/${profileId}`, {
      method: 'DELETE',
    });
    if (!res.ok) {
      const err = await res.json().catch(() => ({}));
      throw new Error(err.detail || 'Failed to delete profile');
    }
    return await res.json();
  },

  async getSettings() {
    if (isTauri) {
      try {
        return await invoke('get_settings');
      } catch (e) {
        console.error('[Adapter] invoke get_settings error:', e);
      }
    }
    const res = await fetch(`${API_BASE}/settings`);
    if (!res.ok) throw new Error('Failed to load settings');
    return await res.json();
  },

  async saveSettings(settings) {
    if (isTauri) {
      return await invoke('save_settings_cmd', { settings });
    }
    const res = await fetch(`${API_BASE}/settings`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(settings),
    });
    if (!res.ok) {
      const err = await res.json().catch(() => ({}));
      throw new Error(err.detail || 'Failed to save settings');
    }
    return await res.json();
  },

  async validateHotkey(keyCombination, actionId = null) {
    if (isTauri) {
      return await invoke('validate_hotkey_cmd', {
        keyCombination,
        key_combination: keyCombination,
        actionId: actionId || null,
        action_id: actionId || null,
      });
    }
    const res = await fetch(`${API_BASE}/hotkeys/validate`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({
        key_combination: keyCombination,
        action_id: actionId,
      }),
    });
    if (!res.ok) {
      const err = await res.json().catch(() => ({}));
      throw new Error(err.detail || 'Failed to validate hotkey');
    }
    return await res.json();
  },

  async openMainWindow() {
    if (isTauri) {
      return await invoke('open_main_window_cmd');
    }
  },

  async hideQuickPanel() {
    if (isTauri) {
      return await invoke('hide_quickpanel_cmd');
    }
  },
};
