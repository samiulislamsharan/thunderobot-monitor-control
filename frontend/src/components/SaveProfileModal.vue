<script setup>
import { ref, watch } from "vue";
import {
  BookmarkPlus,
  X,
  Crosshair,
  Moon,
  Film,
  Monitor,
  Sparkles,
  Check,
} from "lucide-vue-next";

const props = defineProps({
  isOpen: Boolean,
  features: Object,
});

const emit = defineEmits(["close", "save"]);
const dialogRef = ref(null);

const profileName = ref("");
const profileDesc = ref("");
const selectedIcon = ref("Sparkles");

const icons = [
  { name: "Sparkles", icon: Sparkles, label: "Custom" },
  { name: "Crosshair", icon: Crosshair, label: "Gaming" },
  { name: "Moon", icon: Moon, label: "Night" },
  { name: "Film", icon: Film, label: "Cinema" },
  { name: "Monitor", icon: Monitor, label: "Work" },
];

watch(
  () => props.isOpen,
  (open) => {
    if (open) {
      profileName.value = "";
      profileDesc.value = "";
      selectedIcon.value = "Sparkles";
      dialogRef.value?.showModal();
    } else {
      dialogRef.value?.close();
    }
  },
);

const handleSave = () => {
  if (!profileName.value.trim()) return;
  emit("save", {
    name: profileName.value.trim(),
    description: profileDesc.value.trim(),
    icon: selectedIcon.value,
  });
};

const handleBackdropClick = (e) => {
  if (e.target === dialogRef.value) {
    emit("close");
  }
};
</script>

<template>
  <dialog
    ref="dialogRef"
    @click="handleBackdropClick"
    @cancel="emit('close')"
    class="rounded-2xl bg-cyber-card border border-cyber-border text-slate-100 p-0 shadow-2xl max-w-md w-full m-auto overflow-hidden animate-in fade-in zoom-in-95 duration-150"
  >
    <div class="p-6 space-y-5">
      <!-- Header -->
      <div
        class="flex items-center justify-between pb-3 border-b border-slate-800"
      >
        <div class="flex items-center space-x-2.5">
          <div
            class="p-2 rounded-xl bg-cyan-500/10 text-cyan-400 border border-cyan-500/20"
          >
            <BookmarkPlus class="w-5 h-5" />
          </div>
          <div>
            <h3 class="text-base font-bold text-white">Save Current Preset</h3>
            <p class="text-xs text-slate-400">
              Capture active hardware settings into a 1-click profile
            </p>
          </div>
        </div>
        <button
          @click="emit('close')"
          class="p-1 text-slate-400 hover:text-white rounded-lg transition"
        >
          <X class="w-5 h-5" />
        </button>
      </div>

      <!-- Settings Snapshot Badge -->
      <div
        class="p-3 rounded-xl bg-slate-900/60 border border-slate-800 text-xs text-slate-300 space-y-1 font-mono"
      >
        <div
          class="text-[11px] text-slate-400 uppercase font-semibold font-sans"
        >
          Snapshot to be saved:
        </div>
        <div class="flex flex-wrap gap-2 text-[11px]">
          <span class="text-cyan-400"
            >Brightness: {{ features?.brightness }}%</span
          >
          <span class="text-purple-400"
            >Contrast: {{ features?.contrast }}%</span
          >
          <span class="text-emerald-400"
            >Mode: 0x{{ Number(features?.picture_mode).toString(16) }}</span
          >
          <span class="text-amber-400"
            >Overdrive: {{ features?.overdrive }}</span
          >
        </div>
      </div>

      <!-- Form Inputs -->
      <div class="space-y-4">
        <div>
          <label class="text-xs font-semibold text-slate-300 block mb-1.5"
            >Preset Name</label
          >
          <input
            type="text"
            v-model="profileName"
            placeholder="e.g. Apex Legends 250Hz or Late Night Work"
            maxlength="50"
            class="w-full bg-slate-900 border border-slate-800 rounded-xl px-3.5 py-2.5 text-sm text-white placeholder-slate-500 focus:outline-none focus:border-cyan-400 transition"
          />
        </div>

        <div>
          <label class="text-xs font-semibold text-slate-300 block mb-1.5"
            >Description (Optional)</label
          >
          <input
            type="text"
            v-model="profileDesc"
            placeholder="e.g. High refresh rate with maximized visibility"
            maxlength="100"
            class="w-full bg-slate-900 border border-slate-800 rounded-xl px-3.5 py-2.5 text-sm text-white placeholder-slate-500 focus:outline-none focus:border-cyan-400 transition"
          />
        </div>

        <!-- Icon Picker -->
        <div>
          <label class="text-xs font-semibold text-slate-300 block mb-1.5"
            >Choose Icon</label
          >
          <div class="flex items-center space-x-2">
            <button
              v-for="item in icons"
              :key="item.name"
              type="button"
              @click="selectedIcon = item.name"
              :class="[
                'p-2.5 rounded-xl border flex flex-col items-center justify-center transition select-none',
                selectedIcon === item.name
                  ? 'bg-cyan-950/60 border-cyan-400 text-cyan-300'
                  : 'bg-slate-900 border-slate-800 text-slate-400 hover:text-white',
              ]"
            >
              <component :is="item.icon" class="w-4 h-4" />
              <span class="text-[10px] mt-1">{{ item.label }}</span>
            </button>
          </div>
        </div>
      </div>

      <!-- Actions -->
      <div
        class="flex items-center justify-end space-x-2.5 pt-3 border-t border-slate-800"
      >
        <button
          @click="emit('close')"
          class="px-4 py-2 rounded-xl text-xs font-semibold text-slate-400 hover:text-white hover:bg-slate-800 transition"
        >
          Cancel
        </button>
        <button
          @click="handleSave"
          :disabled="!profileName.trim()"
          class="px-5 py-2 rounded-xl text-xs font-bold bg-cyan-500 hover:bg-cyan-400 disabled:opacity-50 disabled:cursor-not-allowed text-slate-950 transition shadow-cyan-glow flex items-center space-x-1.5"
        >
          <Check class="w-4 h-4" />
          <span>Save Preset</span>
        </button>
      </div>
    </div>
  </dialog>
</template>
