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
import { Card } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { Badge } from "@/components/ui/badge";

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
  <Card class="overflow-hidden transition-all">
    <!-- Collapsible Header -->
    <button
      @click="isExpanded = !isExpanded"
      class="flex items-center justify-between w-full p-4 text-left transition cursor-pointer sm:p-5 hover:bg-muted/50"
    >
      <div class="flex items-center space-x-3">
        <div
          class="p-2 border rounded-lg bg-muted text-foreground border-border"
        >
          <Terminal class="w-4 h-4 text-foreground" />
        </div>
        <div>
          <div class="flex items-center space-x-2">
            <h2 class="text-sm font-semibold text-foreground">
              Advanced DDC/CI Protocol Inspector
            </h2>
            <Badge variant="secondary" class="font-mono text-[10px]"
              >Expert</Badge
            >
          </div>
          <p class="text-xs text-muted-foreground">
            Direct register reader/writer & raw hex VCP testing
          </p>
        </div>
      </div>

      <div class="p-1 rounded-md text-muted-foreground">
        <ChevronUp v-if="isExpanded" class="w-5 h-5" />
        <ChevronDown v-else class="w-5 h-5" />
      </div>
    </button>

    <!-- Expanded Body -->
    <div
      v-if="isExpanded"
      class="p-5 space-y-5 border-t border-border bg-muted/20"
    >
      <div class="grid grid-cols-1 gap-4 sm:grid-cols-2">
        <!-- VCP Code Input -->
        <div class="space-y-1.5">
          <label class="block font-mono text-xs font-medium text-muted-foreground"
            >VCP Code (Hex or Dec)</label
          >
          <div class="flex items-center space-x-2">
            <input
              type="text"
              v-model="rawCodeInput"
              placeholder="e.g. 0x10"
              class="w-full px-3 py-2 font-mono text-sm border bg-background border-input rounded-md text-foreground placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 ring-offset-background"
            />
            <Button
              variant="secondary"
              size="sm"
              @click="readVcp"
              :disabled="queryLoading"
            >
              <Search class="w-3.5 h-3.5 mr-1" />
              <span>Read</span>
            </Button>
          </div>
        </div>

        <!-- Target Value Input & Write -->
        <div class="space-y-1.5">
          <label class="block font-mono text-xs font-medium text-muted-foreground"
            >Target Value</label
          >
          <div class="flex items-center space-x-2">
            <input
              type="number"
              v-model="rawValueInput"
              placeholder="e.g. 95"
              class="w-full px-3 py-2 font-mono text-sm text-foreground border bg-background border-input rounded-md placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 ring-offset-background"
            />
            <Button variant="default" size="sm" @click="sendRawVcp">
              <Send class="w-3.5 h-3.5 mr-1" />
              <span>Write</span>
            </Button>
          </div>
        </div>
      </div>

      <!-- Result Card -->
      <div
        v-if="queryResult"
        class="p-3.5 rounded-lg bg-background border border-border text-xs font-mono"
      >
        <div
          v-if="queryResult.error"
          class="flex items-center gap-2 text-destructive"
        >
          <span>❌ {{ queryResult.error }}</span>
        </div>
        <div v-else class="space-y-1 text-slate-300">
          <p class="font-bold text-cyan-400">
            VCP {{ queryResult.hex_code }} ({{
              queryResult.label || queryResult.name
            }})
          </p>
          <p>
            Current Value:
            <span class="font-bold text-emerald-400">{{
              queryResult.current
            }}</span>
            (0x{{ Number(queryResult.current).toString(16).toUpperCase() }})
          </p>
          <p>
            Maximum Value:
            <span class="font-bold text-purple-400">{{ queryResult.max }}</span>
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
          <Badge variant="outline" class="font-mono">0x10: Brightness</Badge>
          <Badge variant="outline" class="font-mono">0x12: Contrast</Badge>
          <Badge variant="outline" class="font-mono">0x14: Color Preset</Badge>
          <Badge variant="outline" class="font-mono">0x60: Input Source</Badge>
          <Badge variant="outline" class="font-mono">0xDC: Picture Mode</Badge>
          <Badge variant="outline" class="font-mono">0x3E: Sharpness</Badge>
          <Badge variant="outline" class="font-mono">0x68: Overdrive</Badge>
          <Badge variant="outline" class="font-mono">0xE3: MPRT</Badge>
          <Badge variant="outline" class="font-mono">0xFD: MStar Tunnel</Badge>
        </div>
      </div>
    </div>
  </Card>
</template>
