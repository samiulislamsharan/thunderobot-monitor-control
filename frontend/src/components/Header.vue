<script setup>
import {
  Monitor,
  Smartphone,
  Activity,
  Zap,
  RefreshCw,
  Clock,
  Download,
} from "lucide-vue-next";
import {
  canInstall,
  promptInstall,
  isStandalone,
} from "../registerServiceWorker";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";

const props = defineProps({
  monitorInfo: Object,
  connected: Boolean,
  updating: Boolean,
});

const emit = defineEmits(["openPhoneModal", "refresh"]);
</script>

<template>
  <header
    class="sticky top-0 z-30 w-full border-b border-border bg-background/95 backdrop-blur supports-[backdrop-filter]:bg-background/60 px-4 lg:px-8 py-3 transition-all"
  >
    <div
      class="flex flex-wrap items-center justify-between gap-3 mx-auto max-w-7xl"
    >
      <!-- Left: Brand & Model -->
      <div class="flex items-center space-x-3">
        <img
          src="/icons/icon-192.png"
          alt="ThundeRobot Logo"
          class="object-cover border rounded-md w-9 h-9 border-border"
        />
        <div>
          <div class="flex items-center space-x-2">
            <h1
              class="text-base font-semibold tracking-tight text-foreground sm:text-lg flex items-center gap-1.5"
            >
              <span>THUNDEROBOT</span>
              <span
                class="font-mono text-xs font-normal text-muted-foreground sm:text-sm"
              >
                {{ monitorInfo.model || "DQ34C240L" }}
              </span>
            </h1>
            <Badge variant="secondary"> 34″ Fast VA </Badge>
          </div>
          <p class="hidden text-xs text-muted-foreground sm:block">
            UWQHD 3440×1440 • 1500R Curved • DDC/CI Hardware Sync
          </p>
        </div>
      </div>

      <!-- Center / Right Telemetry Badges -->
      <div class="flex flex-wrap items-center gap-2 text-xs">
        <!-- Live Refresh Rate -->
        <Badge variant="outline" class="font-mono py-1 px-2.5 gap-1.5">
          <Zap class="w-3.5 h-3.5 text-foreground" />
          <span class="font-bold">
            {{
              monitorInfo.telemetry?.v_freq || monitorInfo.refresh_rate || 250
            }}
            Hz
          </span>
        </Badge>

        <!-- Power On Hours -->
        <Badge
          variant="outline"
          class="hidden md:flex py-1 px-2.5 gap-1.5 text-muted-foreground"
        >
          <Clock class="w-3.5 h-3.5" />
          <span>{{ monitorInfo.telemetry?.usage_hours || 127 }}h On</span>
        </Badge>

        <!-- Connection Status -->
        <Badge variant="outline" class="py-1 px-2.5 gap-1.5">
          <span class="relative flex w-2 h-2">
            <span
              v-if="connected"
              class="absolute inline-flex w-full h-full rounded-full opacity-75 animate-ping bg-emerald-500"
            ></span>
            <span
              :class="connected ? 'bg-emerald-500' : 'bg-destructive'"
              class="relative inline-flex w-2 h-2 rounded-full"
            ></span>
          </span>
          <span
            :class="connected ? 'text-foreground' : 'text-destructive'"
            class="font-medium"
          >
            {{
              connected
                ? updating
                  ? "Syncing..."
                  : "Connected"
                : "Disconnected"
            }}
          </span>
        </Badge>

        <!-- Refresh Button -->
        <Button
          variant="outline"
          size="icon"
          @click="$emit('refresh')"
          class="w-8 h-8"
          title="Reload Hardware State"
        >
          <RefreshCw class="w-4 h-4" :class="{ 'animate-spin': updating }" />
        </Button>

        <!-- PWA Install App Button (shown when installable) -->
        <Button
          v-if="canInstall"
          variant="default"
          size="sm"
          @click="promptInstall"
          class="gap-1.5"
        >
          <Download class="w-3.5 h-3.5" />
          <span>Install App</span>
        </Button>

        <!-- Phone Remote Pairing Button -->
        <Button
          variant="outline"
          size="sm"
          @click="$emit('openPhoneModal')"
          class="gap-1.5"
        >
          <Smartphone class="w-3.5 h-3.5 text-muted-foreground" />
          <span>Phone Remote</span>
        </Button>
      </div>
    </div>
  </header>
</template>
