<script setup>
import {
  Sun,
  Contrast,
  Sliders,
  RotateCcw,
  Film,
  Gamepad2,
  Target,
  Swords,
  LayoutTemplate,
} from "lucide-vue-next";

const props = defineProps({
  features: Object,
  updating: Boolean,
});

const emit = defineEmits(["setFeature", "resetLuminance"]);

const pictureModes = [
  {
    value: 0,
    label: "Standard",
    icon: LayoutTemplate,
    desc: "Balanced default",
  },
  { value: 2, label: "Movie", icon: Film, desc: "Enhanced contrast" },
  { value: 3, label: "Game", icon: Gamepad2, desc: "Dynamic gaming" },
  { value: 5, label: "FPS", icon: Target, desc: "Max shadow visibility" },
  { value: 7, label: "RTS", icon: Swords, desc: "Vibrant strategy colors" },
];

const adjustBrightness = (delta) => {
  const newVal = Math.min(100, Math.max(0, props.features.brightness + delta));
  emit("setFeature", "brightness", newVal);
};

const adjustContrast = (delta) => {
  const newVal = Math.min(100, Math.max(0, props.features.contrast + delta));
  emit("setFeature", "contrast", newVal);
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
          <Sliders class="w-5 h-5" />
        </div>
        <div>
          <h2 class="text-base font-bold text-white">Picture & Luminance</h2>
          <p class="text-xs text-slate-400">
            Backlight brightness, contrast & calibrated picture modes
          </p>
        </div>
      </div>

      <button
        @click="$emit('resetLuminance')"
        class="inline-flex items-center space-x-1.5 px-2.5 py-1 rounded-lg text-xs font-medium text-slate-400 hover:text-amber-400 bg-slate-900 hover:bg-slate-800 border border-slate-800 transition"
        title="Reset Luminance Defaults (0x05)"
      >
        <RotateCcw class="w-3.5 h-3.5" />
        <span class="hidden sm:inline">Reset Defaults</span>
      </button>
    </div>

    <!-- Sliders Grid -->
    <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
      <!-- Brightness Slider -->
      <div
        class="space-y-3 bg-slate-900/40 p-4 rounded-xl border border-slate-800/80"
      >
        <div class="flex items-center justify-between">
          <div class="flex items-center space-x-2">
            <Sun class="w-4 h-4 text-cyan-400" />
            <label class="text-sm font-semibold text-slate-200"
              >Brightness (Luminance)</label
            >
          </div>
          <div class="flex items-center space-x-1.5">
            <button
              @click="adjustBrightness(-5)"
              class="w-7 h-7 rounded-lg bg-slate-800 hover:bg-slate-700 text-slate-300 flex items-center justify-center font-bold text-xs"
            >
              -5
            </button>
            <span
              class="w-12 text-center font-mono font-bold text-cyan-400 bg-slate-950 px-2 py-1 rounded border border-slate-800 text-sm"
            >
              {{ features.brightness }}%
            </span>
            <button
              @click="adjustBrightness(5)"
              class="w-7 h-7 rounded-lg bg-slate-800 hover:bg-slate-700 text-slate-300 flex items-center justify-center font-bold text-xs"
            >
              +5
            </button>
          </div>
        </div>

        <input
          type="range"
          min="0"
          max="100"
          :value="features.brightness"
          @input="
            $emit('setFeature', 'brightness', Number($event.target.value))
          "
          class="w-full"
        />

        <!-- Quick Jump Buttons -->
        <div class="flex items-center justify-between pt-1">
          <button
            v-for="step in [0, 25, 50, 75, 100]"
            :key="step"
            @click="$emit('setFeature', 'brightness', step)"
            :class="
              features.brightness === step
                ? 'text-cyan-400 font-bold border-cyan-400/50 bg-cyan-950/40'
                : 'text-slate-500 hover:text-slate-300 border-slate-800'
            "
            class="px-2 py-0.5 rounded text-[11px] border font-mono transition"
          >
            {{ step }}%
          </button>
        </div>
      </div>

      <!-- Contrast Slider -->
      <div
        class="space-y-3 bg-slate-900/40 p-4 rounded-xl border border-slate-800/80"
      >
        <div class="flex items-center justify-between">
          <div class="flex items-center space-x-2">
            <Contrast class="w-4 h-4 text-purple-400" />
            <label class="text-sm font-semibold text-slate-200"
              >Contrast Ratio</label
            >
          </div>
          <div class="flex items-center space-x-1.5">
            <button
              @click="adjustContrast(-5)"
              class="w-7 h-7 rounded-lg bg-slate-800 hover:bg-slate-700 text-slate-300 flex items-center justify-center font-bold text-xs"
            >
              -5
            </button>
            <span
              class="w-12 text-center font-mono font-bold text-purple-400 bg-slate-950 px-2 py-1 rounded border border-slate-800 text-sm"
            >
              {{ features.contrast }}%
            </span>
            <button
              @click="adjustContrast(5)"
              class="w-7 h-7 rounded-lg bg-slate-800 hover:bg-slate-700 text-slate-300 flex items-center justify-center font-bold text-xs"
            >
              +5
            </button>
          </div>
        </div>

        <input
          type="range"
          min="0"
          max="100"
          :value="features.contrast"
          @input="$emit('setFeature', 'contrast', Number($event.target.value))"
          class="w-full"
        />

        <!-- Quick Jump Buttons -->
        <div class="flex items-center justify-between pt-1">
          <button
            v-for="step in [30, 40, 45, 50, 70]"
            :key="step"
            @click="$emit('setFeature', 'contrast', step)"
            :class="
              features.contrast === step
                ? 'text-purple-400 font-bold border-purple-400/50 bg-purple-950/40'
                : 'text-slate-500 hover:text-slate-300 border-slate-800'
            "
            class="px-2 py-0.5 rounded text-[11px] border font-mono transition"
          >
            {{ step }}%
          </button>
        </div>
      </div>
    </div>

    <!-- Picture Modes Selection -->
    <div class="space-y-3 pt-2">
      <label
        class="text-xs font-semibold text-slate-400 uppercase tracking-wider block"
      >
        Monitor Picture Modes (0xDC)
      </label>
      <div class="grid grid-cols-2 sm:grid-cols-3 lg:grid-cols-5 gap-2.5">
        <button
          v-for="mode in pictureModes"
          :key="mode.value"
          @click="$emit('setFeature', 'picture_mode', mode.value, 0)"
          :class="[
            'p-3 rounded-xl border flex flex-col items-center justify-center text-center space-y-1.5 transition-all select-none',
            features.picture_mode === mode.value
              ? 'bg-cyan-950/40 border-cyan-400 text-cyan-300 shadow-cyan-glow'
              : 'bg-slate-900/60 border-slate-800/80 text-slate-400 hover:text-white hover:border-slate-700',
          ]"
        >
          <component :is="mode.icon" class="w-5 h-5" />
          <span class="text-xs font-bold">{{ mode.label }}</span>
          <span class="text-[10px] text-slate-500 leading-tight">{{
            mode.desc
          }}</span>
        </button>
      </div>
    </div>
  </div>
</template>
