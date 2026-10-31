<script setup>
import { ref, reactive, watch, onMounted, onBeforeUnmount } from "vue";
import {
  Settings,
  X,
  Keyboard,
  Monitor,
  Cpu,
  Smartphone,
  Check,
  AlertTriangle,
  AlertCircle,
  RefreshCw,
  RotateCcw,
  Sliders,
  ShieldCheck,
  Save,
} from "lucide-vue-next";
import { Button } from "@/components/ui/button";
import { Badge } from "@/components/ui/badge";
import { Switch } from "@/components/ui/switch";
import { Slider } from "@/components/ui/slider";
import { Tabs, TabsList, TabsTrigger, TabsContent } from "@/components/ui/tabs";
import { api } from "@/api/adapter";

const props = defineProps({
  isOpen: Boolean,
  monitorInfo: Object,
});

const emit = defineEmits(["close", "settingsSaved"]);
const dialogRef = ref(null);

const activeTab = ref("hotkeys");
const recordingActionId = ref(null);
const validationResults = reactive({});
const isSaving = ref(false);
const saveSuccess = ref(false);

const localSettings = reactive({
  autostart: false,
  close_to_tray: true,
  start_minimized: false,
  default_profile: "preset-fps-250",
  i2c_delay_ms: 35,
  telemetry_interval_sec: 3,
  phone_remote_enabled: true,
  phone_remote_port: 8000,
  hotkeys: [],
});

const loadSettings = async () => {
  try {
    const data = await api.getSettings();
    Object.assign(localSettings, data);
    await validateAllHotkeys();
  } catch (err) {
    console.error("Failed to load settings:", err);
  }
};

const validateAllHotkeys = async () => {
  for (const hk of localSettings.hotkeys) {
    if (!hk.enabled) {
      validationResults[hk.id] = {
        valid: true,
        status: "disabled",
        message: "Hotkey disabled",
      };
      continue;
    }
    try {
      const res = await api.validateHotkey(hk.key_combination, hk.id);
      validationResults[hk.id] = res;
    } catch (e) {
      validationResults[hk.id] = {
        valid: false,
        status: "error",
        message: "Validation error",
      };
    }
  }
};

watch(
  () => props.isOpen,
  async (open) => {
    if (open) {
      dialogRef.value?.showModal();
      await loadSettings();
    } else {
      stopRecording();
      dialogRef.value?.close();
    }
  },
);

// Hotkey Recording Logic
const startRecording = (actionId) => {
  recordingActionId.value = actionId;
};

const stopRecording = () => {
  recordingActionId.value = null;
};

const handleKeyDown = async (e) => {
  if (!recordingActionId.value) return;

  e.preventDefault();
  e.stopPropagation();

  // Escape cancels recording
  if (e.key === "Escape") {
    stopRecording();
    return;
  }

  // Ignore modifier-only presses
  if (["Control", "Alt", "Shift", "Meta"].includes(e.key)) {
    return;
  }

  const parts = [];
  if (e.ctrlKey) parts.push("Ctrl");
  if (e.altKey) parts.push("Alt");
  if (e.shiftKey) parts.push("Shift");
  if (e.metaKey) parts.push("Win");

  // Format main key
  let mainKey = e.key;
  if (mainKey.startsWith("Arrow")) {
    mainKey = mainKey.replace("Arrow", "");
  } else if (mainKey === " ") {
    mainKey = "Space";
  } else if (mainKey.length === 1) {
    mainKey = mainKey.toUpperCase();
  }

  parts.push(mainKey);
  const comboStr = parts.join("+");

  // Find and update the hotkey in localSettings
  const targetHk = localSettings.hotkeys.find(
    (h) => h.id === recordingActionId.value,
  );
  if (targetHk) {
    targetHk.key_combination = comboStr;
    const res = await api.validateHotkey(comboStr, targetHk.id);
    validationResults[targetHk.id] = res;
  }

  stopRecording();
};

onMounted(() => {
  window.addEventListener("keydown", handleKeyDown);
});

onBeforeUnmount(() => {
  window.removeEventListener("keydown", handleKeyDown);
});

const handleSave = async () => {
  isSaving.value = true;
  saveSuccess.value = false;
  try {
    await api.saveSettings(localSettings);
    saveSuccess.value = true;
    emit("settingsSaved", localSettings);
    setTimeout(() => {
      saveSuccess.value = false;
      emit("close");
    }, 600);
  } catch (err) {
    console.error("Failed to save settings:", err);
  } finally {
    isSaving.value = false;
  }
};

const handleResetDefaults = async () => {
  localSettings.i2c_delay_ms = 35;
  localSettings.autostart = false;
  localSettings.close_to_tray = true;
  localSettings.start_minimized = false;
  localSettings.default_profile = "preset-fps-250";
  // Reset hotkeys to standard defaults
  const defaults = [
    {
      id: "brightness_up",
      action: "Brightness Up (+5%)",
      key_combination: "Ctrl+Alt+Up",
      enabled: true,
      description: "Increase backlight luminance",
    },
    {
      id: "brightness_down",
      action: "Brightness Down (-5%)",
      key_combination: "Ctrl+Alt+Down",
      enabled: true,
      description: "Decrease backlight luminance",
    },
    {
      id: "contrast_up",
      action: "Contrast Up (+5%)",
      key_combination: "Ctrl+Alt+Right",
      enabled: true,
      description: "Increase contrast ratio",
    },
    {
      id: "contrast_down",
      action: "Contrast Down (-5%)",
      key_combination: "Ctrl+Alt+Left",
      enabled: true,
      description: "Decrease contrast ratio",
    },
    {
      id: "preset_fps",
      action: "Preset: 250Hz Esports",
      key_combination: "Ctrl+Alt+1",
      enabled: true,
      description: "250Hz FPS gaming preset",
    },
    {
      id: "preset_night",
      action: "Preset: Night Mode",
      key_combination: "Ctrl+Alt+2",
      enabled: true,
      description: "Low blue light eye care",
    },
    {
      id: "preset_cinema",
      action: "Preset: Cinema 21:9",
      key_combination: "Ctrl+Alt+3",
      enabled: true,
      description: "21:9 Cinema movie preset",
    },
    {
      id: "preset_work",
      action: "Preset: Standard Work",
      key_combination: "Ctrl+Alt+4",
      enabled: true,
      description: "Standard office preset",
    },
    {
      id: "toggle_standby",
      action: "Toggle Standby / Sleep",
      key_combination: "Ctrl+Alt+End",
      enabled: true,
      description: "Put monitor into standby",
    },
    {
      id: "toggle_window",
      action: "Show / Hide Window",
      key_combination: "Ctrl+Alt+M",
      enabled: true,
      description: "Toggle app window",
    },
  ];
  localSettings.hotkeys = defaults;
  await validateAllHotkeys();
};

const handleBackdropClick = (e) => {
  if (e.target === dialogRef.value) {
    emit("close");
  }
};
</script>

<template>
  <dialog
    ref="dialogRef"
    @click="handleBackdropClick"
    @cancel="emit('close')"
    class="w-full max-w-2xl p-0 m-auto overflow-hidden duration-150 border shadow-2xl rounded-xl bg-card border-border text-card-foreground animate-in fade-in zoom-in-95"
  >
    <div class="flex flex-col h-[650px] max-h-[90vh]">
      <!-- Header -->
      <div
        class="flex items-center justify-between px-6 py-4 border-b border-border"
      >
        <div class="flex items-center space-x-3">
          <div
            class="p-2 border rounded-lg bg-muted text-foreground border-border"
          >
            <Settings class="w-5 h-5" />
          </div>
          <div>
            <h2 class="text-base font-semibold leading-tight text-foreground">
              Preferences & System Settings
            </h2>
            <p class="text-xs text-muted-foreground">
              Customize global hotkeys, conflict detection & hardware timing
            </p>
          </div>
        </div>

        <Button
          variant="ghost"
          size="icon"
          @click="emit('close')"
          class="w-8 h-8 text-muted-foreground hover:text-foreground"
        >
          <X class="w-4 h-4" />
        </Button>
      </div>

      <!-- Main Body with Tabs -->
      <div class="flex-1 overflow-hidden">
        <Tabs
          v-model="activeTab"
          default-value="hotkeys"
          class="flex flex-col h-full"
        >
          <!-- Tab Navigation Bar -->
          <div class="px-6 pt-3 border-b border-border bg-muted/20">
            <TabsList
              class="justify-start w-full h-auto gap-1 p-0 bg-transparent"
            >
              <TabsTrigger value="hotkeys" class="gap-1.5 py-2">
                <Keyboard class="w-3.5 h-3.5" />
                <span>Hotkeys & Shortcuts</span>
              </TabsTrigger>
              <TabsTrigger value="system" class="gap-1.5 py-2">
                <Monitor class="w-3.5 h-3.5" />
                <span>System & Tray</span>
              </TabsTrigger>
              <TabsTrigger value="hardware" class="gap-1.5 py-2">
                <Cpu class="w-3.5 h-3.5" />
                <span>DDC/CI Bus</span>
              </TabsTrigger>
              <TabsTrigger value="remote" class="gap-1.5 py-2">
                <Smartphone class="w-3.5 h-3.5" />
                <span>Phone Remote</span>
              </TabsTrigger>
            </TabsList>
          </div>

          <!-- Content: Hotkeys Tab -->
          <TabsContent
            value="hotkeys"
            class="flex-1 p-6 m-0 space-y-4 overflow-y-auto"
          >
            <div
              class="flex items-center justify-between pb-2 border-b border-border"
            >
              <div>
                <h3
                  class="text-xs font-semibold tracking-wider uppercase text-muted-foreground"
                >
                  Global System Keyboard Shortcuts
                </h3>
                <p class="text-[11px] text-muted-foreground">
                  Adjust monitor parameters from anywhere without switching
                  windows
                </p>
              </div>

              <div class="flex items-center space-x-2">
                <Button
                  variant="outline"
                  size="sm"
                  @click="validateAllHotkeys"
                  class="gap-1.5 text-xs"
                >
                  <ShieldCheck class="w-3.5 h-3.5 text-primary" />
                  <span>Verify Conflicts</span>
                </Button>
                <Button
                  variant="ghost"
                  size="sm"
                  @click="handleResetDefaults"
                  class="text-xs text-muted-foreground"
                >
                  <RotateCcw class="w-3.5 h-3.5 mr-1" />
                  <span>Defaults</span>
                </Button>
              </div>
            </div>

            <!-- Hotkeys List -->
            <div class="space-y-2.5">
              <div
                v-for="hk in localSettings.hotkeys"
                :key="hk.id"
                class="flex flex-wrap items-center justify-between gap-2 p-3 border rounded-lg bg-muted/30 border-border"
              >
                <!-- Left: Action Name & Description -->
                <div class="flex-1 min-w-[200px]">
                  <div class="flex items-center space-x-2">
                    <span class="text-xs font-semibold text-foreground">{{
                      hk.action
                    }}</span>
                    <!-- Status Conflict Badges -->
                    <template v-if="hk.enabled && validationResults[hk.id]">
                      <!-- OK Status -->
                      <Badge
                        v-if="validationResults[hk.id].status === 'ok'"
                        variant="outline"
                        class="text-[10px] bg-emerald-500/10 text-emerald-500 border-emerald-500/30 gap-1"
                      >
                        <Check class="w-3 h-3" />
                        <span>Ready</span>
                      </Badge>

                      <!-- Internal App Conflict -->
                      <Badge
                        v-else-if="
                          validationResults[hk.id].status ===
                          'internal_conflict'
                        "
                        variant="outline"
                        class="text-[10px] bg-amber-500/10 text-amber-500 border-amber-500/30 gap-1"
                        :title="validationResults[hk.id].message"
                      >
                        <AlertTriangle class="w-3 h-3" />
                        <span>Duplicate</span>
                      </Badge>

                      <!-- System OS Conflict -->
                      <Badge
                        v-else-if="
                          validationResults[hk.id].status === 'system_conflict'
                        "
                        variant="outline"
                        class="text-[10px] bg-destructive/10 text-destructive border-destructive/30 gap-1"
                        :title="validationResults[hk.id].message"
                      >
                        <AlertCircle class="w-3 h-3" />
                        <span>OS Taken</span>
                      </Badge>
                    </template>
                  </div>
                  <p class="text-[11px] text-muted-foreground mt-0.5">
                    {{ hk.description }}
                  </p>
                </div>

                <!-- Right: Hotkey Recorder Box & Enable Switch -->
                <div class="flex items-center space-x-3">
                  <!-- Record Button / Display -->
                  <Button
                    variant="outline"
                    size="sm"
                    @click="startRecording(hk.id)"
                    :disabled="!hk.enabled"
                    :class="[
                      'font-mono text-xs px-3 py-1.5 h-8 min-w-[120px] justify-center transition-all',
                      recordingActionId === hk.id
                        ? 'bg-primary text-primary-foreground animate-pulse border-primary ring-2 ring-primary/30'
                        : 'bg-background hover:bg-accent text-foreground border-border',
                    ]"
                  >
                    <span v-if="recordingActionId === hk.id"
                      >Press key combo...</span
                    >
                    <span v-else>{{ hk.key_combination }}</span>
                  </Button>

                  <!-- Toggle Enable/Disable -->
                  <Switch
                    :checked="hk.enabled"
                    @update:checked="
                      hk.enabled = $event;
                      validateAllHotkeys();
                    "
                  />
                </div>

                <!-- Error Message banner if conflict -->
                <div
                  v-if="
                    hk.enabled &&
                    validationResults[hk.id] &&
                    !validationResults[hk.id].valid
                  "
                  class="w-full text-[11px] px-2.5 py-1 rounded bg-destructive/10 border border-destructive/20 text-destructive flex items-center gap-1.5"
                >
                  <AlertCircle class="w-3.5 h-3.5 shrink-0" />
                  <span>{{ validationResults[hk.id].message }}</span>
                </div>
              </div>
            </div>
          </TabsContent>

          <!-- Content: System & Tray Tab -->
          <TabsContent
            value="system"
            class="flex-1 p-6 m-0 space-y-5 overflow-y-auto"
          >
            <div class="space-y-4">
              <!-- Autostart -->
              <div
                class="flex items-center justify-between p-4 border rounded-lg bg-muted/30 border-border"
              >
                <div class="space-y-0.5">
                  <label class="text-sm font-semibold text-foreground"
                    >Launch on Windows Startup</label
                  >
                  <p class="text-xs text-muted-foreground">
                    Automatically starts monitor control utility when your
                    computer boots
                  </p>
                </div>
                <Switch
                  :checked="localSettings.autostart"
                  @update:checked="localSettings.autostart = $event"
                />
              </div>

              <!-- Close to Tray -->
              <div
                class="flex items-center justify-between p-4 border rounded-lg bg-muted/30 border-border"
              >
                <div class="space-y-0.5">
                  <label class="text-sm font-semibold text-foreground"
                    >Close to System Tray</label
                  >
                  <p class="text-xs text-muted-foreground">
                    Clicking the window close (X) button minimizes to the system
                    notification area
                  </p>
                </div>
                <Switch
                  :checked="localSettings.close_to_tray"
                  @update:checked="localSettings.close_to_tray = $event"
                />
              </div>

              <!-- Start Minimized -->
              <div
                class="flex items-center justify-between p-4 border rounded-lg bg-muted/30 border-border"
              >
                <div class="space-y-0.5">
                  <label class="text-sm font-semibold text-foreground"
                    >Start Minimized</label
                  >
                  <p class="text-xs text-muted-foreground">
                    Launch silently in the background tray without popping up
                    the main window
                  </p>
                </div>
                <Switch
                  :checked="localSettings.start_minimized"
                  @update:checked="localSettings.start_minimized = $event"
                />
              </div>

              <!-- Default Profile on Startup -->
              <div
                class="p-4 space-y-2 border rounded-lg bg-muted/30 border-border"
              >
                <label class="block text-sm font-semibold text-foreground"
                  >Default Startup Preset</label
                >
                <p class="text-xs text-muted-foreground">
                  Select which profile hardware settings to automatically apply
                  upon application launch
                </p>
                <div class="grid grid-cols-2 gap-2 pt-1 sm:grid-cols-4">
                  <Button
                    v-for="prof in [
                      { id: 'preset-fps-250', label: '250Hz Esports' },
                      { id: 'preset-night-reading', label: 'Night Mode' },
                      { id: 'preset-cinema', label: 'Cinema 21:9' },
                      { id: 'preset-standard-work', label: 'Standard 6500K' },
                    ]"
                    :key="prof.id"
                    variant="outline"
                    size="sm"
                    @click="localSettings.default_profile = prof.id"
                    :class="[
                      'text-xs justify-center',
                      localSettings.default_profile === prof.id
                        ? 'bg-accent text-accent-foreground border-primary font-semibold ring-1 ring-primary/20'
                        : 'bg-card text-muted-foreground border-border',
                    ]"
                  >
                    {{ prof.label }}
                  </Button>
                </div>
              </div>
            </div>
          </TabsContent>

          <!-- Content: Hardware DDC/CI Tab -->
          <TabsContent
            value="hardware"
            class="flex-1 p-6 m-0 space-y-5 overflow-y-auto"
          >
            <div class="space-y-4">
              <!-- I2C Delay Slider -->
              <div
                class="p-4 space-y-3 border rounded-lg bg-muted/30 border-border"
              >
                <div class="flex items-center justify-between">
                  <div>
                    <label class="text-sm font-semibold text-foreground"
                      >I2C Bus Command Delay</label
                    >
                    <p class="text-xs text-muted-foreground">
                      Rate-limiting delay between DDC/CI register writes
                    </p>
                  </div>
                  <span
                    class="font-mono text-xs font-bold px-2.5 py-1 rounded border bg-background border-border text-foreground"
                  >
                    {{ localSettings.i2c_delay_ms }} ms
                  </span>
                </div>

                <Slider
                  :model-value="[localSettings.i2c_delay_ms]"
                  :min="20"
                  :max="100"
                  :step="5"
                  @update:model-value="localSettings.i2c_delay_ms = $event[0]"
                />

                <p class="text-[11px] text-muted-foreground">
                  💡 <strong>Recommended: 35ms</strong>. The ThundeRobot
                  DQ34C240L uses an MStar scaler. Setting below 25ms can cause
                  command drops during rapid slider movement.
                </p>
              </div>

              <!-- Telemetry Polling -->
              <div
                class="p-4 space-y-3 border rounded-lg bg-muted/30 border-border"
              >
                <div class="flex items-center justify-between">
                  <div>
                    <label class="text-sm font-semibold text-foreground"
                      >Live Telemetry Polling</label
                    >
                    <p class="text-xs text-muted-foreground">
                      Frequency to query live vertical refresh rate (250Hz) and
                      usage hours
                    </p>
                  </div>
                  <span
                    class="font-mono text-xs font-bold px-2.5 py-1 rounded border bg-background border-border text-foreground"
                  >
                    {{ localSettings.telemetry_interval_sec }} sec
                  </span>
                </div>

                <Slider
                  :model-value="[localSettings.telemetry_interval_sec]"
                  :min="1"
                  :max="10"
                  :step="1"
                  @update:model-value="
                    localSettings.telemetry_interval_sec = $event[0]
                  "
                />
              </div>
            </div>
          </TabsContent>

          <!-- Content: Phone Remote Tab -->
          <TabsContent
            value="remote"
            class="flex-1 p-6 m-0 space-y-5 overflow-y-auto"
          >
            <div class="space-y-4">
              <div
                class="flex items-center justify-between p-4 border rounded-lg bg-muted/30 border-border"
              >
                <div class="space-y-0.5">
                  <label class="text-sm font-semibold text-foreground"
                    >Enable Mobile Wi-Fi Server</label
                  >
                  <p class="text-xs text-muted-foreground">
                    Permits smartphones on your local Wi-Fi to control the
                    monitor
                  </p>
                </div>
                <Switch
                  :checked="localSettings.phone_remote_enabled"
                  @update:checked="localSettings.phone_remote_enabled = $event"
                />
              </div>

              <div
                class="p-4 space-y-2 border rounded-lg bg-muted/30 border-border"
              >
                <label class="block text-sm font-semibold text-foreground"
                  >HTTP Server Port</label
                >
                <div class="flex items-center max-w-xs space-x-2">
                  <input
                    type="number"
                    v-model.number="localSettings.phone_remote_port"
                    min="1024"
                    max="65535"
                    class="w-full bg-background border border-input rounded-md px-3 py-1.5 text-sm font-mono text-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
                  />
                  <Badge variant="secondary" class="font-mono text-xs"
                    >Default 8000</Badge
                  >
                </div>
              </div>

              <div
                class="p-4 border rounded-lg bg-muted/40 border-border text-xs text-muted-foreground space-y-1.5"
              >
                <p class="font-semibold text-foreground">
                  Pairing Information:
                </p>
                <p>
                  Local IP:
                  <span class="font-mono font-semibold text-foreground">{{
                    monitorInfo?.lan_ip || "127.0.0.1"
                  }}</span>
                </p>
                <p>
                  Phone URL:
                  <span class="font-mono font-semibold text-foreground">{{
                    monitorInfo?.phone_url || "http://localhost:8000"
                  }}</span>
                </p>
              </div>
            </div>
          </TabsContent>
        </Tabs>
      </div>

      <!-- Footer Buttons -->
      <div
        class="flex items-center justify-between px-6 py-3.5 border-t border-border bg-muted/20"
      >
        <Button variant="ghost" size="sm" @click="emit('close')">
          Cancel
        </Button>

        <div class="flex items-center space-x-2">
          <Button
            variant="default"
            size="sm"
            @click="handleSave"
            :disabled="isSaving"
            class="gap-1.5 min-w-[120px]"
          >
            <Check v-if="saveSuccess" class="w-4 h-4 text-emerald-400" />
            <RefreshCw v-else-if="isSaving" class="w-4 h-4 animate-spin" />
            <Save v-else class="w-4 h-4" />
            <span>{{
              saveSuccess
                ? "Saved!"
                : isSaving
                  ? "Saving..."
                  : "Save Preferences"
            }}</span>
          </Button>
        </div>
      </div>
    </div>
  </dialog>
</template>
