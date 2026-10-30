<script setup>
import { Monitor, Smartphone, Activity, Zap, RefreshCw, Clock, Download } from 'lucide-vue-next';
import { canInstall, promptInstall, isStandalone } from '../registerServiceWorker';

const props = defineProps({
  monitorInfo: Object,
  connected: Boolean,
  updating: Boolean
});

const emit = defineEmits(['openPhoneModal', 'refresh']);
</script>

<template>
  <header
    class="border-b border-cyber-border bg-cyber-card/80 backdrop-blur-md sticky top-0 z-30 px-4 lg:px-8 py-3.5 transition-all"
  >
    <div
      class="max-w-7xl mx-auto flex flex-wrap items-center justify-between gap-3"
    >
      <!-- Left: Brand & Model -->
      <div class="flex items-center space-x-3">
        <img
          src="/icons/icon-192.png"
          alt="ThundeRobot Logo"
          class="w-10 h-10 rounded-xl object-cover border border-cyan-500/40 shadow-cyan-glow"
        />
        <div>
          <div class="flex items-center space-x-2">
            <h1
              class="text-base sm:text-lg font-bold tracking-wider text-white"
            >
              THUNDEROBOT
              <span class="text-cyan-400 font-mono text-sm sm:text-base">{{
                monitorInfo.model || "DQ34C240L"
              }}</span>
            </h1>
            <span
              class="inline-flex items-center px-2 py-0.5 rounded text-[11px] font-semibold bg-cyan-950/80 text-cyan-400 border border-cyan-500/30"
            >
              34″ Fast VA
            </span>
          </div>
          <p class="text-xs text-slate-400 hidden sm:block">
            UWQHD 3440×1440 • 1500R Curved • DDC/CI Hardware Sync
          </p>
        </div>
      </div>

      <!-- Center / Right Telemetry Badges -->
      <div class="flex items-center flex-wrap gap-2 text-xs">
        <!-- Live Refresh Rate -->
        <div
          class="flex items-center space-x-1.5 px-2.5 py-1.5 rounded-lg bg-slate-900 border border-slate-800 text-slate-200"
        >
          <Zap class="w-3.5 h-3.5 text-cyan-400" />
          <span class="font-mono font-bold text-cyan-300">
            {{
              monitorInfo.telemetry?.v_freq || monitorInfo.refresh_rate || 250
            }}
            Hz
          </span>
        </div>

        <!-- Power On Hours -->
        <div
          class="hidden md:flex items-center space-x-1.5 px-2.5 py-1.5 rounded-lg bg-slate-900 border border-slate-800 text-slate-300"
        >
          <Clock class="w-3.5 h-3.5 text-purple-400" />
          <span>{{ monitorInfo.telemetry?.usage_hours || 127 }}h On</span>
        </div>

        <!-- Connection Status -->
        <div
          class="flex items-center space-x-1.5 px-2.5 py-1.5 rounded-lg bg-slate-900 border border-slate-800"
        >
          <span class="relative flex h-2 w-2">
            <span
              v-if="connected"
              class="animate-ping absolute inline-flex h-full w-full rounded-full bg-emerald-400 opacity-75"
            ></span>
            <span
              :class="connected ? 'bg-emerald-400' : 'bg-red-400'"
              class="relative inline-flex rounded-full h-2 w-2"
            ></span>
          </span>
          <span
            :class="connected ? 'text-emerald-400' : 'text-red-400'"
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
        </div>

        <!-- Refresh Button -->
        <button
          @click="$emit('refresh')"
          class="p-1.5 rounded-lg bg-slate-900 hover:bg-slate-800 border border-slate-800 text-slate-400 hover:text-cyan-400 transition"
          title="Reload Hardware State"
        >
          <RefreshCw class="w-4 h-4" :class="{ 'animate-spin': updating }" />
        </button>

        <!-- PWA Install App Button (shown when installable) -->
        <button
          v-if="canInstall"
          @click="promptInstall"
          class="flex items-center space-x-1.5 px-3 py-1.5 rounded-lg bg-emerald-500/20 hover:bg-emerald-500/30 border border-emerald-500/40 text-emerald-300 font-medium transition shadow-sm animate-pulse"
        >
          <Download class="w-3.5 h-3.5 text-emerald-400" />
          <span>Install App</span>
        </button>

        <!-- Phone Remote Pairing Button -->
        <button
          @click="$emit('openPhoneModal')"
          class="flex items-center space-x-1.5 px-3 py-1.5 rounded-lg bg-gradient-to-r from-cyan-500/20 to-purple-500/20 hover:from-cyan-500/30 hover:to-purple-500/30 border border-cyan-500/40 text-cyan-300 font-medium transition shadow-sm hover:shadow-cyan-glow"
        >
          <Smartphone class="w-3.5 h-3.5 text-cyan-400" />
          <span>Phone Remote</span>
        </button>
      </div>
    </div>
  </header>
</template>
