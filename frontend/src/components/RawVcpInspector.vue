<script setup>
import { ref } from "vue";
import {
  Terminal,
  Send,
  Search,
  ChevronDown,
  ChevronUp,
  Cpu,
  Info,
} from "lucide-vue-next";

const props = defineProps({
  updating: Boolean,
});

const emit = defineEmits(["setRawVcp"]);

const isExpanded = ref(false);
const rawCodeInput = ref("0x10");
const rawValueInput = ref(95);
const queryResult = ref(null);
const queryLoading = ref(false);

const readVcp = async () => {
  queryLoading.value = true;
  queryResult.value = null;
  try {
    const res = await fetch(`/api/vcp/${rawCodeInput.value}`);
    if (res.ok) {
      queryResult.value = await res.json();
    } else {
      const err = await res.json();
      queryResult.value = { error: err.detail || "Read failed" };
    }
  } catch (e) {
    queryResult.value = { error: e.message };
  } finally {
    queryLoading.value = false;
  }
};

const sendRawVcp = () => {
  let code = rawCodeInput.value;
  if (typeof code === "string" && code.startsWith("0x")) {
    code = parseInt(code, 16);
  } else {
    code = parseInt(code, 10);
  }
  emit("setRawVcp", code, Number(rawValueInput.value));
};
</script>

<template>
  <div
    class="rounded-2xl bg-cyber-card border border-cyber-border overflow-hidden transition-all"
  >
    <!-- Collapsible Header -->
    <button
      @click="isExpanded = !isExpanded"
      class="w-full p-4 sm:p-5 flex items-center justify-between hover:bg-slate-900/50 transition text-left"
    >
      <div class="flex items-center space-x-2.5">
        <div
          class="p-2 rounded-xl bg-slate-800 text-slate-300 border border-slate-700"
        >
          <Terminal class="w-4 h-4 text-cyan-400" />
        </div>
        <div>
          <div class="flex items-center space-x-2">
            <h2 class="text-sm font-bold text-white">
              Advanced DDC/CI Protocol Inspector
            </h2>
            <span
              class="text-[10px] px-1.5 py-0.5 rounded bg-slate-800 text-slate-400 font-mono"
              >Expert</span
            >
          </div>
          <p class="text-xs text-slate-400">
            Direct register reader/writer & raw hex VCP testing
          </p>
        </div>
      </div>

      <div class="p-1 rounded-lg text-slate-400">
        <ChevronUp v-if="isExpanded" class="w-5 h-5" />
        <ChevronDown v-else class="w-5 h-5" />
      </div>
    </button>

    <!-- Expanded Body -->
    <div
      v-if="isExpanded"
      class="p-5 border-t border-slate-800 space-y-5 bg-slate-950/40"
    >
      <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
        <!-- VCP Code Input -->
        <div class="space-y-1.5">
          <label class="text-xs font-semibold text-slate-400 block font-mono"
            >VCP Code (Hex or Dec)</label
          >
          <div class="flex items-center space-x-2">
            <input
              type="text"
              v-model="rawCodeInput"
              placeholder="e.g. 0x10"
              class="w-full bg-slate-900 border border-slate-800 rounded-xl px-3 py-2 text-sm font-mono text-cyan-400 focus:outline-none focus:border-cyan-500"
            />
            <button
              @click="readVcp"
              :disabled="queryLoading"
              class="px-3.5 py-2 rounded-xl bg-slate-800 hover:bg-slate-700 text-slate-200 text-xs font-semibold flex items-center space-x-1.5 transition"
            >
              <Search class="w-3.5 h-3.5" />
              <span>Read</span>
            </button>
          </div>
        </div>

        <!-- Target Value Input & Write -->
        <div class="space-y-1.5">
          <label class="text-xs font-semibold text-slate-400 block font-mono"
            >Target Value</label
          >
          <div class="flex items-center space-x-2">
            <input
              type="number"
              v-model="rawValueInput"
              placeholder="e.g. 95"
              class="w-full bg-slate-900 border border-slate-800 rounded-xl px-3 py-2 text-sm font-mono text-purple-400 focus:outline-none focus:border-purple-500"
            />
            <button
              @click="sendRawVcp"
              class="px-3.5 py-2 rounded-xl bg-cyan-500 hover:bg-cyan-400 text-slate-950 text-xs font-bold flex items-center space-x-1.5 transition shadow-sm"
            >
              <Send class="w-3.5 h-3.5" />
              <span>Write</span>
            </button>
          </div>
        </div>
      </div>

      <!-- Result Card -->
      <div
        v-if="queryResult"
        class="p-3.5 rounded-xl bg-slate-900 border border-slate-800 text-xs font-mono"
      >
        <div
          v-if="queryResult.error"
          class="text-red-400 flex items-center gap-2"
        >
          <span>❌ {{ queryResult.error }}</span>
        </div>
        <div v-else class="space-y-1 text-slate-300">
          <p class="text-cyan-400 font-bold">
            VCP {{ queryResult.hex_code }} ({{
              queryResult.label || queryResult.name
            }})
          </p>
          <p>
            Current Value:
            <span class="text-emerald-400 font-bold">{{
              queryResult.current
            }}</span>
            (0x{{ Number(queryResult.current).toString(16).toUpperCase() }})
          </p>
          <p>
            Maximum Value:
            <span class="text-purple-400 font-bold">{{ queryResult.max }}</span>
          </p>
          <p>
            Control Type:
            {{
              queryResult.type === 1
                ? "Continuous / Slider"
                : "Discrete / Trigger"
            }}
          </p>
        </div>
      </div>

      <!-- Common Mapped Hex Codes Cheat Sheet -->
      <div
        class="text-xs text-slate-400 space-y-1.5 pt-2 border-t border-slate-900"
      >
        <p class="font-semibold text-slate-300">
          Quick Reference for ThundeRobot DQ34C240L:
        </p>
        <div class="flex flex-wrap gap-2 text-[11px] font-mono">
          <span class="px-2 py-0.5 rounded bg-slate-900 border border-slate-800"
            >0x10: Brightness</span
          >
          <span class="px-2 py-0.5 rounded bg-slate-900 border border-slate-800"
            >0x12: Contrast</span
          >
          <span class="px-2 py-0.5 rounded bg-slate-900 border border-slate-800"
            >0x14: Color Preset</span
          >
          <span class="px-2 py-0.5 rounded bg-slate-900 border border-slate-800"
            >0x60: Input Source</span
          >
          <span class="px-2 py-0.5 rounded bg-slate-900 border border-slate-800"
            >0xDC: Picture Mode</span
          >
          <span class="px-2 py-0.5 rounded bg-slate-900 border border-slate-800"
            >0x3E: Sharpness</span
          >
          <span class="px-2 py-0.5 rounded bg-slate-900 border border-slate-800"
            >0x68: Overdrive</span
          >
          <span class="px-2 py-0.5 rounded bg-slate-900 border border-slate-800"
            >0xE3: MPRT</span
          >
          <span class="px-2 py-0.5 rounded bg-slate-900 border border-slate-800"
            >0xFD: MStar Tunnel</span
          >
        </div>
      </div>
    </div>
  </div>
</template>
