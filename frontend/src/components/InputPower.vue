<script setup>
import {
  MonitorPlay,
  Power,
  Moon,
  AlertTriangle,
  RotateCcw,
} from "lucide-vue-next";
import {
  Card,
  CardHeader,
  CardTitle,
  CardDescription,
  CardContent,
} from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { Badge } from "@/components/ui/badge";

const props = defineProps({
  features: Object,
  updating: Boolean,
});

const emit = defineEmits(["setFeature", "openConfirmModal", "resetFactory"]);

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
  <Card class="overflow-hidden">
    <!-- Header -->
    <CardHeader
      class="flex flex-row items-center justify-between pb-4 border-b border-border"
    >
      <div class="flex items-center space-x-3">
        <div
          class="p-2 border rounded-lg bg-muted text-foreground border-border"
        >
          <MonitorPlay class="w-5 h-5" />
        </div>
        <div>
          <CardTitle>Input Source & Power Management</CardTitle>
          <CardDescription
            >Switch active display ports and manage power
            states</CardDescription
          >
        </div>
      </div>
    </CardHeader>

    <CardContent class="pt-5 space-y-6">
      <!-- Video Input Select -->
      <div class="space-y-3">
        <label
          class="block text-xs font-semibold tracking-wider uppercase text-muted-foreground"
        >
          Video Input Connectors (0x60)
        </label>
        <div class="grid grid-cols-2 sm:grid-cols-4 gap-2.5">
          <Button
            v-for="inp in inputs"
            :key="inp.value"
            variant="outline"
            @click="handleInputSelect(inp.value)"
            :class="[
              'h-auto p-3.5 rounded-lg flex flex-col items-center justify-center text-center space-y-1.5 transition-all select-none',
              features.input_source === inp.value
                ? 'bg-accent text-accent-foreground border-primary shadow-xs ring-1 ring-primary/20'
                : 'bg-card text-muted-foreground hover:text-foreground border-border',
            ]"
          >
            <component :is="inp.icon" class="w-5 h-5 text-foreground" />
            <span class="text-xs font-semibold">{{ inp.label }}</span>
            <Badge
              :variant="
                features.input_source === inp.value ? 'default' : 'secondary'
              "
              class="text-[10px] font-mono"
            >
              {{ inp.badge }}
            </Badge>
          </Button>
        </div>
      </div>

      <!-- Power & Reset Controls -->
      <div
        class="flex flex-wrap items-center justify-between gap-3 pt-4 border-t border-border"
      >
        <div class="flex items-center space-x-2">
          <!-- Standby Button -->
          <Button
            variant="outline"
            size="sm"
            @click="handleStandby"
            class="text-muted-foreground hover:text-foreground"
          >
            <Moon class="w-4 h-4 mr-1.5" />
            <span>Sleep / Standby</span>
          </Button>

          <!-- Power Off Button -->
          <Button
            variant="outline"
            size="sm"
            @click="handlePowerOff"
            class="text-destructive hover:bg-destructive hover:text-destructive-foreground border-destructive/30"
          >
            <Power class="w-4 h-4 mr-1.5" />
            <span>Turn Off Monitor</span>
          </Button>
        </div>

        <!-- Factory Reset -->
        <Button
          variant="outline"
          size="sm"
          @click="handleFactoryReset"
          class="text-muted-foreground hover:text-foreground"
        >
          <RotateCcw class="w-4 h-4 mr-1.5" />
          <span>Factory Reset All</span>
        </Button>
      </div>
    </CardContent>
  </Card>
</template>
