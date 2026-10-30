<script setup>
import {
  Palette,
  RotateCcw,
  Thermometer,
  SlidersHorizontal,
  Sparkles,
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

const emit = defineEmits(["setFeature", "resetColor"]);

const colorPresets = [
  { value: 5, label: "6500K", subtitle: "Warm", color: "#f59e0b" },
  { value: 6, label: "7500K", subtitle: "Normal / Standard", color: "#38bdf8" },
  { value: 8, label: "9300K", subtitle: "Cool", color: "#818cf8" },
  { value: 11, label: "User Custom", subtitle: "Manual RGB", color: "#ec4899" },
];
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
          <Palette class="w-5 h-5" />
        </div>
        <div>
          <CardTitle>Color Calibration & White Point</CardTitle>
          <CardDescription
            >Color temperature presets, RGB gains & black
            levels</CardDescription
          >
        </div>
      </div>

      <Button
        variant="outline"
        size="sm"
        @click="$emit('resetColor')"
        title="Reset Color Defaults (0x08)"
      >
        <RotateCcw class="w-3.5 h-3.5 mr-1.5" />
        <span class="hidden sm:inline">Reset Defaults</span>
      </Button>
    </CardHeader>

    <CardContent class="pt-5 space-y-6">
      <!-- Color Temperature Preset Selection -->
      <div class="space-y-3">
        <div class="flex items-center space-x-2">
          <Thermometer class="w-4 h-4 text-foreground" />
          <label
            class="block text-xs font-semibold tracking-wider uppercase text-muted-foreground"
          >
            Color Temperature Preset (0x14)
          </label>
        </div>

        <div class="grid grid-cols-2 sm:grid-cols-4 gap-2.5">
          <Button
            v-for="preset in colorPresets"
            :key="preset.value"
            variant="outline"
            @click="$emit('setFeature', 'color_preset', preset.value, 0)"
            :class="[
              'h-auto p-3 rounded-lg flex flex-col items-center justify-center text-center space-y-1 transition-all select-none',
              features.color_preset === preset.value
                ? 'bg-accent text-accent-foreground border-primary shadow-xs ring-1 ring-primary/20'
                : 'bg-card text-muted-foreground hover:text-foreground border-border',
            ]"
          >
            <span class="text-sm font-semibold">{{ preset.label }}</span>
            <span class="text-[11px] text-muted-foreground font-normal">{{
              preset.subtitle
            }}</span>
          </Button>
        </div>
      </div>

      <!-- RGB Gain Sliders -->
      <div class="pt-2 space-y-4">
        <div class="flex items-center justify-between">
          <label
            class="block text-xs font-semibold tracking-wider uppercase text-muted-foreground"
          >
            RGB Gain Adjustments (0x16, 0x18, 0x1A)
          </label>
          <span class="text-xs text-muted-foreground"
            >Active in User / Custom mode</span
          >
        </div>

        <div class="space-y-3.5">
          <!-- Red Gain -->
          <div
            class="space-y-2 bg-muted/40 p-3.5 rounded-lg border border-border"
          >
            <div class="flex items-center justify-between text-xs">
              <span
                class="flex items-center gap-2 font-semibold text-foreground"
              >
                <span
                  class="w-2.5 h-2.5 rounded-full bg-red-500 inline-block shadow-sm"
                ></span>
                Red Gain
              </span>
              <span class="font-mono font-bold text-foreground"
                >{{ features.red_gain }}%</span
              >
            </div>
            <Slider
              :model-value="[features.red_gain]"
              :min="0"
              :max="100"
              :step="1"
              variant="red"
              @update:model-value="$emit('setFeature', 'red_gain', $event[0])"
            />
          </div>

          <!-- Green Gain -->
          <div
            class="space-y-2 bg-muted/40 p-3.5 rounded-lg border border-border"
          >
            <div class="flex items-center justify-between text-xs">
              <span
                class="flex items-center gap-2 font-semibold text-foreground"
              >
                <span
                  class="w-2.5 h-2.5 rounded-full bg-emerald-500 inline-block shadow-sm"
                ></span>
                Green Gain
              </span>
              <span class="font-mono font-bold text-foreground"
                >{{ features.green_gain }}%</span
              >
            </div>
            <Slider
              :model-value="[features.green_gain]"
              :min="0"
              :max="100"
              :step="1"
              variant="green"
              @update:model-value="$emit('setFeature', 'green_gain', $event[0])"
            />
          </div>

          <!-- Blue Gain -->
          <div
            class="space-y-2 bg-muted/40 p-3.5 rounded-lg border border-border"
          >
            <div class="flex items-center justify-between text-xs">
              <span
                class="flex items-center gap-2 font-semibold text-foreground"
              >
                <span
                  class="w-2.5 h-2.5 rounded-full bg-blue-500 inline-block shadow-sm"
                ></span>
                Blue Gain
              </span>
              <span class="font-mono font-bold text-foreground"
                >{{ features.blue_gain }}%</span
              >
            </div>
            <Slider
              :model-value="[features.blue_gain]"
              :min="0"
              :max="100"
              :step="1"
              variant="blue"
              @update:model-value="$emit('setFeature', 'blue_gain', $event[0])"
            />
          </div>
        </div>
      </div>
    </CardContent>
  </Card>
</template>
