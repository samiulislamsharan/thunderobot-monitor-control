<script setup>
import {
  Gamepad2,
  Gauge,
  Zap,
  Crosshair,
  ScreenShare,
  Sparkles,
} from "lucide-vue-next";

const props = defineProps({
  features: Object,
  updating: Boolean,
});

const emit = defineEmits(["setFeature"]);

const overdriveLevels = [
  { value: 0, label: "Off", desc: "Default latency" },
  { value: 1, label: "Level 1", desc: "Mild speedup" },
  { value: 2, label: "Level 2", desc: "Balanced" },
  { value: 3, label: "Level 3", desc: "Fast" },
  { value: 4, label: "Level 4", desc: "Very Fast" },
  { value: 5, label: "Level 5", desc: "Extreme" },
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
          class="p-2 rounded-xl bg-cyan-500/10 text-cyan-400 border border-cyan-500/20"
        >
          <Gamepad2 class="w-5 h-5" />
        </div>
        <div>
          <h2 class="text-base font-bold text-white">
            Gaming & Motion Features
          </h2>
          <p class="text-xs text-slate-400">
            Response time overdrive, black level clarity & motion blur
          </p>
        </div>
      </div>

      <span
        class="px-2.5 py-1 rounded-full text-[11px] font-bold bg-cyan-950 text-cyan-300 border border-cyan-500/30 font-mono"
      >
        250Hz Overclock Tuner
      </span>
    </div>

    <!-- Overdrive Selection -->
    <div class="space-y-3">
      <div class="flex items-center space-x-2">
        <Gauge class="w-4 h-4 text-cyan-400" />
        <label
          class="text-xs font-semibold text-slate-400 uppercase tracking-wider block"
        >
          Pixel Overdrive / Response Time (0x68)
        </label>
      </div>

      <div class="grid grid-cols-2 sm:grid-cols-3 lg:grid-cols-6 gap-2">
        <button
          v-for="level in overdriveLevels"
          :key="level.value"
          @click="$emit('setFeature', 'overdrive', level.value, 0)"
          :class="[
            'p-2.5 rounded-xl border flex flex-col items-center justify-center text-center transition-all select-none',
            features.overdrive === level.value
              ? 'bg-cyan-950/60 border-cyan-400 text-cyan-300 shadow-cyan-glow'
              : 'bg-slate-900/60 border-slate-800/80 text-slate-400 hover:text-white hover:border-slate-700',
          ]"
        >
          <span class="text-xs font-bold">{{ level.label }}</span>
          <span class="text-[10px] text-slate-500">{{ level.desc }}</span>
        </button>
      </div>
    </div>

    <!-- Sharpness / Black Equalizer Slider & Motion Blur Toggle -->
    <div class="grid grid-cols-1 md:grid-cols-2 gap-6 pt-2">
      <!-- Sharpness / Black Equalizer -->
      <div
        class="space-y-3 bg-slate-900/40 p-4 rounded-xl border border-slate-800/80"
      >
        <div class="flex items-center justify-between">
          <div class="flex items-center space-x-2">
            <Sparkles class="w-4 h-4 text-amber-400" />
            <label class="text-sm font-semibold text-slate-200"
              >Sharpness / Black EQ (0x3E)</label
            >
          </div>
          <span
            class="font-mono font-bold text-amber-400 bg-slate-950 px-2 py-0.5 rounded border border-slate-800 text-xs"
          >
            {{ features.gaming_sharpness }}%
          </span>
        </div>

        <input
          type="range"
          min="0"
          max="100"
          :value="features.gaming_sharpness"
          @input="
            $emit('setFeature', 'gaming_sharpness', Number($event.target.value))
          "
          class="w-full"
        />

        <p class="text-[11px] text-slate-400">
          Clarifies dark shadow areas in competitive FPS titles without blowing
          out highlights.
        </p>
      </div>

      <!-- MPRT Motion Blur Strobe Toggle -->
      <div
        class="flex flex-col justify-between bg-slate-900/40 p-4 rounded-xl border border-slate-800/80"
      >
        <div>
          <div class="flex items-center justify-between mb-1.5">
            <div class="flex items-center space-x-2">
              <Zap class="w-4 h-4 text-cyan-400" />
              <label class="text-sm font-semibold text-slate-200"
                >1ms MPRT Motion Blur (0xE3)</label
              >
            </div>
            <button
              @click="
                $emit(
                  'setFeature',
                  'mprt_aim',
                  features.mprt_aim === 1 ? 0 : 1,
                  0,
                )
              "
              :class="
                features.mprt_aim === 1
                  ? 'bg-cyan-500 text-slate-950'
                  : 'bg-slate-800 text-slate-400'
              "
              class="relative inline-flex h-6 w-11 items-center rounded-full transition-colors focus:outline-none"
            >
              <span
                :class="
                  features.mprt_aim === 1 ? 'translate-x-6' : 'translate-x-1'
                "
                class="inline-block h-4 w-4 transform rounded-full bg-white transition-transform"
              />
            </button>
          </div>
          <p class="text-[11px] text-slate-400">
            Backlight strobing for crystal-clear motion tracking. (May disable
            variable refresh rate).
          </p>
        </div>

        <!-- OSD Orientation -->
        <div
          class="mt-4 pt-3 border-t border-slate-800 flex items-center justify-between"
        >
          <span class="text-xs text-slate-300 flex items-center gap-1.5">
            <ScreenShare class="w-3.5 h-3.5 text-purple-400" />
            Orientation (0xAA)
          </span>
          <div class="flex items-center space-x-1">
            <button
              @click="$emit('setFeature', 'orientation', 1, 0)"
              :class="
                features.orientation === 1
                  ? 'bg-slate-800 text-cyan-300 font-bold border-cyan-500/40'
                  : 'text-slate-500 border-transparent'
              "
              class="px-2 py-0.5 rounded text-xs border transition"
            >
              Landscape (0°)
            </button>
            <button
              @click="$emit('setFeature', 'orientation', 2, 0)"
              :class="
                features.orientation === 2
                  ? 'bg-slate-800 text-cyan-300 font-bold border-cyan-500/40'
                  : 'text-slate-500 border-transparent'
              "
              class="px-2 py-0.5 rounded text-xs border transition"
            >
              Portrait (90°)
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
