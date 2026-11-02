<script setup>
import { computed } from "vue";
import {
  Sun,
  Contrast,
  Maximize2,
  X,
  Crosshair,
  Moon,
  Film,
  Monitor,
  Sparkles,
  Zap,
  Sliders,
  ExternalLink,
} from "lucide-vue-next";
import { Button } from "@/components/ui/button";
import { Badge } from "@/components/ui/badge";
import { Slider } from "@/components/ui/slider";
import { Switch } from "@/components/ui/switch";
import { useMonitor } from "@/composables/useMonitor";
import { api } from "@/api/adapter";

const {
  features,
  profiles,
  activeProfileId,
  updating,
  setFeature,
  applyProfile,
} = useMonitor();

const getIcon = (iconName) => {
  switch (iconName) {
    case "Crosshair":
      return Crosshair;
    case "Moon":
      return Moon;
    case "Film":
      return Film;
    case "Monitor":
      return Monitor;
    default:
      return Sparkles;
  }
};

const handleOpenFull = async () => {
  await api.openMainWindow();
};

const handleClose = async () => {
  await api.hideQuickPanel();
};

const adjustBrightness = (delta) => {
  const newVal = Math.min(100, Math.max(0, features.brightness + delta));
  setFeature("brightness", newVal);
};

const adjustContrast = (delta) => {
  const newVal = Math.min(100, Math.max(0, features.contrast + delta));
  setFeature("contrast", newVal);
};

const handleBrightnessWheel = (e) => {
  e.preventDefault();
  const step = e.shiftKey ? 5 : 2;
  const delta = e.deltaY < 0 ? step : -step;
  adjustBrightness(delta);
};

const handleContrastWheel = (e) => {
  e.preventDefault();
  const step = e.shiftKey ? 5 : 2;
  const delta = e.deltaY < 0 ? step : -step;
  adjustContrast(delta);
};

const overdriveLevels = [
  { level: 0, label: "Off" },
  { level: 1, label: "L1" },
  { level: 2, label: "L2" },
  { level: 3, label: "L3" },
  { level: 4, label: "L4" },
  { level: 5, label: "L5" },
];
</script>

<template>
  <div
    class="w-full h-full min-h-screen bg-background/95 backdrop-blur-xl text-foreground flex flex-col justify-between border border-border/80 shadow-2xl rounded-2xl overflow-hidden select-none box-border"
  >
    <!-- Top Header -->
    <div
      class="flex items-center justify-between px-3.5 py-2.5 border-b border-border bg-muted/30"
    >
      <div class="flex items-center space-x-2">
        <div
          class="w-6 h-6 rounded-lg bg-primary/10 border border-primary/20 flex items-center justify-center text-primary"
        >
          <Sliders class="w-3.5 h-3.5" />
        </div>
        <div>
          <h1 class="text-xs font-bold text-foreground leading-tight">
            ThundeRobot DQ34C240L
          </h1>
          <p class="text-[10px] text-muted-foreground font-medium">
            34″ Curved • 250Hz Quick Panel
          </p>
        </div>
      </div>

      <div class="flex items-center space-x-1">
        <Button
          variant="ghost"
          size="icon"
          class="w-7 h-7 rounded-lg text-muted-foreground hover:text-foreground"
          @click="handleOpenFull"
          title="Open Full Dashboard"
        >
          <Maximize2 class="w-3.5 h-3.5" />
        </Button>
        <Button
          variant="ghost"
          size="icon"
          class="w-7 h-7 rounded-lg text-muted-foreground hover:text-foreground"
          @click="handleClose"
          title="Close Quick Panel"
        >
          <X class="w-3.5 h-3.5" />
        </Button>
      </div>
    </div>

    <!-- Main Content Body -->
    <div class="flex-1 px-3.5 py-3 space-y-3 overflow-y-auto">
      <!-- 1. Brightness Slider -->
      <div
        class="space-y-2 p-3 rounded-xl border border-border bg-card/60 transition-colors hover:border-border/80 group/brightness"
        @wheel.prevent="handleBrightnessWheel"
        title="Scroll with mouse wheel to adjust brightness"
      >
        <div class="flex items-center justify-between">
          <div class="flex items-center space-x-2">
            <Sun class="w-4 h-4 text-primary" />
            <span class="text-xs font-semibold text-foreground">Brightness</span>
          </div>
          <div class="flex items-center space-x-1">
            <Button
              variant="outline"
              size="sm"
              class="h-6 w-6 p-0 text-[11px] font-bold"
              @click="adjustBrightness(-5)"
            >
              -5
            </Button>
            <Badge
              variant="secondary"
              class="font-mono text-xs px-1.5 py-0.5 font-bold min-w-[38px] text-center"
            >
              {{ features.brightness }}%
            </Badge>
            <Button
              variant="outline"
              size="sm"
              class="h-6 w-6 p-0 text-[11px] font-bold"
              @click="adjustBrightness(5)"
            >
              +5
            </Button>
          </div>
        </div>

        <Slider
          :model-value="[features.brightness]"
          :min="0"
          :max="100"
          :step="1"
          @update:model-value="setFeature('brightness', $event[0])"
          @wheel.prevent="handleBrightnessWheel"
        />

        <div class="flex items-center justify-between pt-1">
          <Button
            v-for="step in [0, 25, 50, 75, 100]"
            :key="step"
            variant="ghost"
            size="sm"
            class="h-5 px-1.5 text-[10px] text-muted-foreground hover:text-foreground"
            @click="setFeature('brightness', step)"
          >
            {{ step }}%
          </Button>
        </div>
      </div>

      <!-- 2. Contrast Slider -->
      <div
        class="space-y-2 p-3 rounded-xl border border-border bg-card/60 transition-colors hover:border-border/80 group/contrast"
        @wheel.prevent="handleContrastWheel"
        title="Scroll with mouse wheel to adjust contrast"
      >
        <div class="flex items-center justify-between">
          <div class="flex items-center space-x-2">
            <Contrast class="w-4 h-4 text-primary" />
            <span class="text-xs font-semibold text-foreground">Contrast</span>
          </div>
          <div class="flex items-center space-x-1">
            <Button
              variant="outline"
              size="sm"
              class="h-6 w-6 p-0 text-[11px] font-bold"
              @click="adjustContrast(-5)"
            >
              -5
            </Button>
            <Badge
              variant="secondary"
              class="font-mono text-xs px-1.5 py-0.5 font-bold min-w-[38px] text-center"
            >
              {{ features.contrast }}%
            </Badge>
            <Button
              variant="outline"
              size="sm"
              class="h-6 w-6 p-0 text-[11px] font-bold"
              @click="adjustContrast(5)"
            >
              +5
            </Button>
          </div>
        </div>

        <Slider
          :model-value="[features.contrast]"
          :min="0"
          :max="100"
          :step="1"
          @update:model-value="setFeature('contrast', $event[0])"
          @wheel.prevent="handleContrastWheel"
        />
      </div>

      <!-- 3. Quick Presets Grid (The Monitor's Default Presets) -->
      <div class="space-y-1.5">
        <div class="flex items-center justify-between">
          <span
            class="text-[11px] font-semibold uppercase tracking-wider text-muted-foreground"
          >
            Default Presets
          </span>
          <span class="text-[10px] text-muted-foreground">
            1-Click Switch
          </span>
        </div>

        <div class="grid grid-cols-2 gap-2">
          <button
            v-for="profile in profiles"
            :key="profile.id"
            @click="applyProfile(profile.id)"
            :class="[
              'p-2.5 rounded-xl border text-left transition-all duration-150 flex flex-col justify-between space-y-1.5 group',
              activeProfileId === profile.id
                ? 'bg-primary/10 border-primary ring-1 ring-primary/30'
                : 'bg-card hover:bg-muted/50 border-border',
            ]"
          >
            <div class="flex items-center justify-between w-full">
              <div
                class="w-6 h-6 rounded-lg bg-muted flex items-center justify-center text-foreground group-hover:text-primary transition"
              >
                <component :is="getIcon(profile.icon)" class="w-3.5 h-3.5" />
              </div>
              <Badge
                variant="secondary"
                class="text-[9px] px-1.5 py-0 font-medium tracking-tight uppercase"
              >
                {{ profile.badge || "Mode" }}
              </Badge>
            </div>
            <div>
              <div class="text-xs font-semibold text-foreground truncate">
                {{ profile.name }}
              </div>
              <div class="text-[10px] text-muted-foreground truncate">
                {{ profile.settings?.brightness }}% Lum •
                {{ profile.settings?.contrast }}% Cont
              </div>
            </div>
          </button>
        </div>
      </div>

      <!-- 4. Gaming Quick Overdrive & Strobe -->
      <div class="p-3 rounded-xl border border-border bg-card/60 space-y-2">
        <div class="flex items-center justify-between">
          <div class="flex items-center space-x-2">
            <Zap class="w-3.5 h-3.5 text-primary" />
            <span class="text-xs font-semibold text-foreground"
              >Pixel Overdrive</span
            >
          </div>
          <Badge variant="outline" class="text-[10px] font-mono">
            {{
              features.overdrive === 0
                ? "Off"
                : `Level ${features.overdrive}`
            }}
          </Badge>
        </div>

        <div class="grid grid-cols-6 gap-1">
          <button
            v-for="item in overdriveLevels"
            :key="item.level"
            @click="setFeature('overdrive', item.level)"
            :class="[
              'py-1 text-center rounded-lg text-[11px] font-bold border transition',
              features.overdrive === item.level
                ? 'bg-primary text-primary-foreground border-primary'
                : 'bg-muted/40 hover:bg-muted border-border text-muted-foreground',
            ]"
          >
            {{ item.label }}
          </button>
        </div>

        <div
          class="flex items-center justify-between pt-1 border-t border-border/60"
        >
          <div class="text-[11px] text-foreground font-medium">
            1ms MPRT Motion Blur Strobe
          </div>
          <Switch
            :checked="features.mprt_aim === 1"
            @update:checked="setFeature('mprt_aim', $event ? 1 : 0)"
          />
        </div>
      </div>
    </div>

    <!-- Bottom Footer Button -->
    <div class="p-3 border-t border-border bg-muted/20">
      <Button
        variant="default"
        class="w-full h-9 text-xs font-semibold flex items-center justify-center space-x-1.5 shadow-sm"
        @click="handleOpenFull"
      >
        <span>Open Full Dashboard</span>
        <ExternalLink class="w-3.5 h-3.5" />
      </Button>
    </div>
  </div>
</template>
