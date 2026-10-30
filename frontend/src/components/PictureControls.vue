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
import {
  Card,
  CardHeader,
  CardTitle,
  CardDescription,
  CardContent,
} from "@/components/ui/card";
import { Slider } from "@/components/ui/slider";
import { Button } from "@/components/ui/button";

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
  <Card class="overflow-hidden">
    <!-- Header -->
    <CardHeader
      class="flex flex-row items-center justify-between pb-4 border-b border-border"
    >
      <div class="flex items-center space-x-3">
        <div
          class="p-2 border rounded-lg bg-muted text-foreground border-border"
        >
          <Sliders class="w-5 h-5" />
        </div>
        <div>
          <CardTitle>Picture & Luminance</CardTitle>
          <CardDescription
            >Backlight brightness, contrast & calibrated picture
            modes</CardDescription
          >
        </div>
      </div>

      <Button
        variant="outline"
        size="sm"
        @click="$emit('resetLuminance')"
        title="Reset Luminance Defaults (0x05)"
      >
        <RotateCcw class="w-3.5 h-3.5 mr-1.5" />
        <span class="hidden sm:inline">Reset Defaults</span>
      </Button>
    </CardHeader>

    <CardContent class="pt-5 space-y-6">
      <!-- Sliders Grid -->
      <div class="grid grid-cols-1 gap-6 md:grid-cols-2">
        <!-- Brightness Slider -->
        <div
          class="p-4 space-y-3 border bg-muted/40 rounded-lg border-border"
        >
          <div class="flex items-center justify-between">
            <div class="flex items-center space-x-2">
              <Sun class="w-4 h-4 text-foreground" />
              <label class="text-sm font-semibold text-foreground"
                >Brightness (Luminance)</label
              >
            </div>
            <div class="flex items-center space-x-1.5">
              <Button
                variant="secondary"
                size="sm"
                @click="adjustBrightness(-5)"
                class="p-0 font-bold w-7 h-7"
              >
                -5
              </Button>
              <span
                class="w-12 px-2 py-1 font-mono text-xs font-bold text-center border rounded text-foreground bg-background border-border"
              >
                {{ features.brightness }}%
              </span>
              <Button
                variant="secondary"
                size="sm"
                @click="adjustBrightness(5)"
                class="p-0 font-bold w-7 h-7"
              >
                +5
              </Button>
            </div>
          </div>

          <!-- shadcn-vue Slider -->
          <Slider
            :model-value="[features.brightness]"
            :min="0"
            :max="100"
            :step="1"
            variant="default"
            @update:model-value="$emit('setFeature', 'brightness', $event[0])"
          />

          <!-- Quick Jump Buttons -->
          <div class="flex items-center justify-between pt-1">
            <Button
              v-for="step in [0, 25, 50, 75, 100]"
              :key="step"
              variant="ghost"
              size="sm"
              @click="$emit('setFeature', 'brightness', step)"
              :class="
                features.brightness === step
                  ? 'bg-accent text-accent-foreground font-semibold border border-border shadow-xs'
                  : 'text-muted-foreground hover:text-foreground border border-transparent'
              "
              class="h-6 px-2 text-[11px] font-mono"
            >
              {{ step }}%
            </Button>
          </div>
        </div>

        <!-- Contrast Slider -->
        <div
          class="p-4 space-y-3 border bg-muted/40 rounded-lg border-border"
        >
          <div class="flex items-center justify-between">
            <div class="flex items-center space-x-2">
              <Contrast class="w-4 h-4 text-foreground" />
              <label class="text-sm font-semibold text-foreground"
                >Contrast Ratio</label
              >
            </div>
            <div class="flex items-center space-x-1.5">
              <Button
                variant="secondary"
                size="sm"
                @click="adjustContrast(-5)"
                class="p-0 font-bold w-7 h-7"
              >
                -5
              </Button>
              <span
                class="w-12 px-2 py-1 font-mono text-xs font-bold text-center text-foreground border rounded bg-background border-border"
              >
                {{ features.contrast }}%
              </span>
              <Button
                variant="secondary"
                size="sm"
                @click="adjustContrast(5)"
                class="p-0 font-bold w-7 h-7"
              >
                +5
              </Button>
            </div>
          </div>

          <!-- shadcn-vue Slider -->
          <Slider
            :model-value="[features.contrast]"
            :min="0"
            :max="100"
            :step="1"
            variant="default"
            @update:model-value="$emit('setFeature', 'contrast', $event[0])"
          />

          <!-- Quick Jump Buttons -->
          <div class="flex items-center justify-between pt-1">
            <Button
              v-for="step in [30, 40, 45, 50, 70]"
              :key="step"
              variant="ghost"
              size="sm"
              @click="$emit('setFeature', 'contrast', step)"
              :class="
                features.contrast === step
                  ? 'bg-accent text-accent-foreground font-semibold border border-border shadow-xs'
                  : 'text-muted-foreground hover:text-foreground border border-transparent'
              "
              class="h-6 px-2 text-[11px] font-mono"
            >
              {{ step }}%
            </Button>
          </div>
        </div>
      </div>

      <!-- Picture Modes Selection -->
      <div class="pt-2 space-y-3">
        <label
          class="block text-xs font-semibold tracking-wider uppercase text-muted-foreground"
        >
          Monitor Picture Modes (0xDC)
        </label>
        <div class="grid grid-cols-2 sm:grid-cols-3 lg:grid-cols-5 gap-2.5">
          <Button
            v-for="mode in pictureModes"
            :key="mode.value"
            variant="outline"
            @click="$emit('setFeature', 'picture_mode', mode.value, 0)"
            :class="[
              'h-auto p-3 rounded-lg flex flex-col items-center justify-center text-center space-y-1.5 transition-all select-none',
              features.picture_mode === mode.value
                ? 'bg-accent text-accent-foreground border-primary shadow-xs ring-1 ring-primary/20'
                : 'bg-card text-muted-foreground hover:text-foreground border-border',
            ]"
          >
            <component :is="mode.icon" class="w-5 h-5 text-foreground" />
            <span class="text-xs font-semibold">{{ mode.label }}</span>
            <span
              class="text-[10px] text-muted-foreground leading-tight font-normal"
              >{{ mode.desc }}</span
            >
          </Button>
        </div>
      </div>
    </CardContent>
  </Card>
</template>
