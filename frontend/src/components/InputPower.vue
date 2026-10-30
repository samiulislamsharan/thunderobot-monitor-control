<script setup>
import {
  MonitorPlay,
  Power,
  Moon,
  AlertTriangle,
  RotateCcw,
} from "lucide-vue-next";

const props = defineProps({
  features: Object,
  updating: Boolean,
});

const emit = defineEmits(["setFeature", "openConfirmModal"]);

const inputs = [
  {
    value: 15,
    label: "DisplayPort 1",
    badge: "Primary (250Hz)",
    icon: MonitorPlay,
  },
  { value: 16, label: "DisplayPort 2", badge: "Secondary", icon: MonitorPlay },
  { value: 17, label: "HDMI 1", badge: "144Hz Max", icon: MonitorPlay },
  { value: 18, label: "HDMI 2", badge: "144Hz Max", icon: MonitorPlay },
];

const handleInputSelect = (inputValue) => {
  if (inputValue !== props.features.input_source) {
    emit("openConfirmModal", {
      title: "Switch Video Input?",
      message: `Switching to ${inputs.find((i) => i.value === inputValue)?.label}. Ensure that video cable is plugged in, otherwise screen will go to standby.`,
      confirmText: "Switch Input",
      confirmColor: "bg-cyan-500 hover:bg-cyan-400 text-slate-950",
      action: () => emit("setFeature", "input_source", inputValue, 0),
    });
  }
};

const handleStandby = () => {
  emit("openConfirmModal", {
    title: "Enter Monitor Standby?",
    message:
      "The monitor will enter low-power standby mode. Moving your mouse or pressing a keyboard key will wake it.",
    confirmText: "Enter Standby",
    confirmColor: "bg-purple-600 hover:bg-purple-500 text-white",
    action: () => emit("setFeature", "power_mode", 4, 0),
  });
};

const handlePowerOff = () => {
  emit("openConfirmModal", {
    title: "Turn Off Monitor?",
    message:
      "Powering off the monitor via DDC/CI will shut down the screen. You may need to press the physical monitor joystick to turn it back on.",
    confirmText: "Power Off",
    confirmColor: "bg-red-600 hover:bg-red-500 text-white",
    action: () => emit("setFeature", "power_mode", 5, 0),
  });
};

const handleFactoryReset = () => {
  emit("openConfirmModal", {
    title: "Factory Reset All Settings?",
    message:
      "This will reset all brightness, contrast, color calibrations, and picture settings to factory original state.",
    confirmText: "Factory Reset",
    confirmColor: "bg-red-600 hover:bg-red-500 text-white",
    action: () => emit("resetFactory"),
  });
};
</script>

<template>
  <div
    class="rounded-2xl bg-cyber-card border border-cyber-border p-5 lg:p-6 space-y-6"
  >
    <!-- Header -->
    <div
      class="flex items-center justify-between pb-4 border-b border-slate-800"
    >
      <div class="flex items-center space-x-2.5">
        <div
          class="p-2 rounded-xl bg-cyan-500/10 text-cyan-400 border border-cyan-500/20"
        >
          <MonitorPlay class="w-5 h-5" />
        </div>
        <div>
          <h2 class="text-base font-bold text-white">
            Input Source & Power Management
          </h2>
          <p class="text-xs text-slate-400">
            Switch active display ports and manage power states
          </p>
        </div>
      </div>
    </div>

    <!-- Video Input Select -->
    <div class="space-y-3">
      <label
        class="text-xs font-semibold text-slate-400 uppercase tracking-wider block"
      >
        Video Input Connectors (0x60)
      </label>
      <div class="grid grid-cols-2 sm:grid-cols-4 gap-2.5">
        <button
          v-for="inp in inputs"
          :key="inp.value"
          @click="handleInputSelect(inp.value)"
          :class="[
            'p-3.5 rounded-xl border flex flex-col items-center justify-center text-center space-y-1.5 transition-all select-none',
            features.input_source === inp.value
              ? 'bg-cyan-950/60 border-cyan-400 text-cyan-300 shadow-cyan-glow'
              : 'bg-slate-900/60 border-slate-800/80 text-slate-400 hover:text-white hover:border-slate-700',
          ]"
        >
          <component :is="inp.icon" class="w-5 h-5" />
          <span class="text-xs font-bold">{{ inp.label }}</span>
          <span
            class="text-[10px] px-1.5 py-0.5 rounded font-mono"
            :class="
              features.input_source === inp.value
                ? 'bg-cyan-900/80 text-cyan-200'
                : 'bg-slate-800 text-slate-400'
            "
          >
            {{ inp.badge }}
          </span>
        </button>
      </div>
    </div>

    <!-- Power & Reset Controls -->
    <div
      class="pt-2 border-t border-slate-800/80 flex flex-wrap items-center justify-between gap-3"
    >
      <div class="flex items-center space-x-2">
        <!-- Standby Button -->
        <button
          @click="handleStandby"
          class="px-3.5 py-2 rounded-xl bg-slate-900 hover:bg-slate-800 text-purple-400 hover:text-purple-300 border border-slate-800 hover:border-purple-500/40 text-xs font-medium flex items-center space-x-2 transition"
        >
          <Moon class="w-4 h-4" />
          <span>Sleep / Standby</span>
        </button>

        <!-- Power Off Button -->
        <button
          @click="handlePowerOff"
          class="px-3.5 py-2 rounded-xl bg-slate-900 hover:bg-red-950/40 text-red-400 hover:text-red-300 border border-slate-800 hover:border-red-500/40 text-xs font-medium flex items-center space-x-2 transition"
        >
          <Power class="w-4 h-4" />
          <span>Turn Off Monitor</span>
        </button>
      </div>

      <!-- Factory Reset -->
      <button
        @click="handleFactoryReset"
        class="px-3.5 py-2 rounded-xl bg-slate-900 hover:bg-slate-800 text-slate-400 hover:text-amber-400 border border-slate-800 text-xs font-medium flex items-center space-x-2 transition"
      >
        <RotateCcw class="w-4 h-4" />
        <span>Factory Reset All</span>
      </button>
    </div>
  </div>
</template>
