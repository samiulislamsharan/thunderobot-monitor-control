<script setup>
import {
  Gamepad2,
  Gauge,
  Zap,
  Crosshair,
  ScreenShare,
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
import { Switch } from "@/components/ui/switch";
import { Button } from "@/components/ui/button";
import { Badge } from "@/components/ui/badge";

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
  <Card class="overflow-hidden">
    <!-- Header -->
    <CardHeader
      class="flex flex-row items-center justify-between pb-4 border-b border-border"
    >
      <div class="flex items-center space-x-3">
        <div
          class="p-2 border rounded-lg bg-muted text-foreground border-border"
        >
          <Gamepad2 class="w-5 h-5" />
        </div>
        <div>
          <CardTitle>Gaming & Motion Features</CardTitle>
          <CardDescription
            >Response time overdrive, black level clarity & motion
            blur</CardDescription
          >
        </div>
      </div>

      <Badge variant="default" class="font-mono"> 250Hz Overclock Tuner </Badge>
    </CardHeader>

    <CardContent class="pt-5 space-y-6">
      <!-- Overdrive Selection -->
      <div class="space-y-3">
        <div class="flex items-center space-x-2">
          <Gauge class="w-4 h-4 text-foreground" />
          <label
            class="block text-xs font-semibold tracking-wider uppercase text-muted-foreground"
          >
            Pixel Overdrive / Response Time (0x68)
          </label>
        </div>

        <div class="grid grid-cols-2 gap-2 sm:grid-cols-3 lg:grid-cols-6">
          <Button
            v-for="level in overdriveLevels"
            :key="level.value"
            variant="outline"
            @click="$emit('setFeature', 'overdrive', level.value, 0)"
            :class="[
              'h-auto p-2.5 rounded-lg flex flex-col items-center justify-center text-center transition-all select-none',
              features.overdrive === level.value
                ? 'bg-accent text-accent-foreground border-primary shadow-xs ring-1 ring-primary/20'
                : 'bg-card text-muted-foreground hover:text-foreground border-border',
            ]"
          >
            <span class="text-xs font-bold">{{ level.label }}</span>
            <span class="text-[10px] text-muted-foreground font-normal">{{
              level.desc
            }}</span>
          </Button>
        </div>
      </div>

      <!-- Sharpness Slider & MPRT Switch -->
      <div class="grid grid-cols-1 gap-6 pt-2 md:grid-cols-2">
        <!-- Sharpness / Black Equalizer -->
        <div class="p-4 space-y-3 border rounded-lg bg-muted/40 border-border">
          <div class="flex items-center justify-between">
            <div class="flex items-center space-x-2">
              <Sparkles class="w-4 h-4 text-foreground" />
              <label class="text-sm font-semibold text-foreground"
                >Sharpness / Black EQ (0x3E)</label
              >
            </div>
            <span
              class="font-mono font-bold text-foreground bg-background px-2 py-0.5 rounded border border-border text-xs"
            >
              {{ features.gaming_sharpness }}%
            </span>
          </div>

          <Slider
            :model-value="[features.gaming_sharpness]"
            :min="0"
            :max="100"
            :step="1"
            variant="default"
            @update:model-value="
              $emit('setFeature', 'gaming_sharpness', $event[0])
            "
          />

          <p class="text-[11px] text-muted-foreground">
            Clarifies dark shadow areas in competitive FPS titles without
            blowing out highlights.
          </p>
        </div>

        <!-- MPRT Motion Blur Strobe Switch -->
        <div
          class="flex flex-col justify-between p-4 border rounded-lg bg-muted/40 border-border"
        >
          <div>
            <div class="flex items-center justify-between mb-1.5">
              <div class="flex items-center space-x-2">
                <Zap class="w-4 h-4 text-foreground" />
                <label class="text-sm font-semibold text-foreground"
                  >1ms MPRT Motion Blur (0xE3)</label
                >
              </div>

              <!-- shadcn-vue Switch -->
              <Switch
                :checked="features.mprt_aim === 1"
                @update:checked="
                  $emit('setFeature', 'mprt_aim', $event ? 1 : 0, 0)
                "
              />
            </div>
            <p class="text-[11px] text-muted-foreground">
              Backlight strobing for crystal-clear motion tracking. (May disable
              variable refresh rate).
            </p>
          </div>

          <!-- OSD Orientation -->
          <div
            class="flex items-center justify-between pt-3 mt-4 border-t border-border"
          >
            <span
              class="text-xs text-muted-foreground flex items-center gap-1.5"
            >
              <ScreenShare class="w-3.5 h-3.5 text-foreground" />
              Orientation (0xAA)
            </span>
            <div class="flex items-center space-x-1.5">
              <Button
                variant="outline"
                size="sm"
                @click="$emit('setFeature', 'orientation', 1, 0)"
                :class="
                  features.orientation === 1
                    ? 'bg-accent text-accent-foreground font-semibold border-border'
                    : 'text-muted-foreground border-transparent'
                "
              >
                Landscape (0°)
              </Button>
              <Button
                variant="outline"
                size="sm"
                @click="$emit('setFeature', 'orientation', 2, 0)"
                :class="
                  features.orientation === 2
                    ? 'bg-accent text-accent-foreground font-semibold border-border'
                    : 'text-muted-foreground border-transparent'
                "
              >
                Portrait (90°)
              </Button>
            </div>
          </div>
        </div>
      </div>
    </CardContent>
  </Card>
</template>
