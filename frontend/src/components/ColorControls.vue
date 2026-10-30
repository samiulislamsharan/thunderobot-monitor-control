<script setup>
import {
  Palette,
  RotateCcw,
  Thermometer,
  SlidersHorizontal,
  Sparkles,
} from "lucide-vue-next";

const props = defineProps({
  features: Object,
  updating: Boolean,
});

const emit = defineEmits(["setFeature", "resetColor"]);

const colorPresets = [
  { value: 5, label: "6500K", subtitle: "Warm", color: "#f59e0b" },
  { value: 6, label: "7500K", subtitle: "Normal / Standard", color: "#38bdf8" },
  { value: 8, label: "9300K", subtitle: "Cool", color: "#818cf8" },
  { value: 11, label: "User Custom", subtitle: "Manual RGB", color: "#ec4899" },
];
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
          class="p-2 rounded-xl bg-purple-500/10 text-purple-400 border border-purple-500/20"
        >
          <Palette class="w-5 h-5" />
        </div>
        <div>
          <h2 class="text-base font-bold text-white">
            Color Calibration & White Point
          </h2>
          <p class="text-xs text-slate-400">
            Color temperature presets, RGB gains & black levels
          </p>
        </div>
      </div>

      <button
        @click="$emit('resetColor')"
        class="inline-flex items-center space-x-1.5 px-2.5 py-1 rounded-lg text-xs font-medium text-slate-400 hover:text-amber-400 bg-slate-900 hover:bg-slate-800 border border-slate-800 transition"
        title="Reset Color Defaults (0x08)"
      >
        <RotateCcw class="w-3.5 h-3.5" />
        <span class="hidden sm:inline">Reset Defaults</span>
      </button>
    </div>

    <!-- Color Temperature Preset Selection -->
    <div class="space-y-3">
      <div class="flex items-center space-x-2">
        <Thermometer class="w-4 h-4 text-purple-400" />
        <label
          class="text-xs font-semibold text-slate-400 uppercase tracking-wider block"
        >
          Color Temperature Preset (0x14)
        </label>
      </div>

      <div class="grid grid-cols-2 sm:grid-cols-4 gap-2.5">
        <button
          v-for="preset in colorPresets"
          :key="preset.value"
          @click="$emit('setFeature', 'color_preset', preset.value, 0)"
          :class="[
            'p-3 rounded-xl border flex flex-col items-center justify-center text-center space-y-1 transition-all select-none',
            features.color_preset === preset.value
              ? 'bg-purple-950/40 border-purple-400 text-purple-200 shadow-purple-glow'
              : 'bg-slate-900/60 border-slate-800/80 text-slate-400 hover:text-white hover:border-slate-700',
          ]"
        >
          <span class="text-sm font-bold">{{ preset.label }}</span>
          <span class="text-[11px] text-slate-400">{{ preset.subtitle }}</span>
        </button>
      </div>
    </div>

    <!-- RGB Gain Sliders -->
    <div class="space-y-4 pt-2">
      <div class="flex items-center justify-between">
        <label
          class="text-xs font-semibold text-slate-400 uppercase tracking-wider block"
        >
          RGB Gain Adjustments (0x16, 0x18, 0x1A)
        </label>
        <span class="text-xs text-slate-500">Active in User / Custom mode</span>
      </div>

      <div class="space-y-3.5">
        <!-- Red Gain -->
        <div
          class="space-y-1.5 bg-slate-900/40 p-3 rounded-xl border border-slate-800/60"
        >
          <div class="flex items-center justify-between text-xs">
            <span class="font-bold text-red-400 flex items-center gap-1.5">
              <span
                class="w-2.5 h-2.5 rounded-full bg-red-500 inline-block shadow-sm"
              ></span>
              Red Gain
            </span>
            <span class="font-mono font-bold text-red-400"
              >{{ features.red_gain }}%</span
            >
          </div>
          <input
            type="range"
            min="0"
            max="100"
            :value="features.red_gain"
            @input="
              $emit('setFeature', 'red_gain', Number($event.target.value))
            "
            class="w-full slider-red"
          />
        </div>

        <!-- Green Gain -->
        <div
          class="space-y-1.5 bg-slate-900/40 p-3 rounded-xl border border-slate-800/60"
        >
          <div class="flex items-center justify-between text-xs">
            <span class="font-bold text-emerald-400 flex items-center gap-1.5">
              <span
                class="w-2.5 h-2.5 rounded-full bg-emerald-500 inline-block shadow-sm"
              ></span>
              Green Gain
            </span>
            <span class="font-mono font-bold text-emerald-400"
              >{{ features.green_gain }}%</span
            >
          </div>
          <input
            type="range"
            min="0"
            max="100"
            :value="features.green_gain"
            @input="
              $emit('setFeature', 'green_gain', Number($event.target.value))
            "
            class="w-full slider-green"
          />
        </div>

        <!-- Blue Gain -->
        <div
          class="space-y-1.5 bg-slate-900/40 p-3 rounded-xl border border-slate-800/60"
        >
          <div class="flex items-center justify-between text-xs">
            <span class="font-bold text-blue-400 flex items-center gap-1.5">
              <span
                class="w-2.5 h-2.5 rounded-full bg-blue-500 inline-block shadow-sm"
              ></span>
              Blue Gain
            </span>
            <span class="font-mono font-bold text-blue-400"
              >{{ features.blue_gain }}%</span
            >
          </div>
          <input
            type="range"
            min="0"
            max="100"
            :value="features.blue_gain"
            @input="
              $emit('setFeature', 'blue_gain', Number($event.target.value))
            "
            class="w-full slider-blue"
          />
        </div>
      </div>
    </div>
  </div>
</template>
