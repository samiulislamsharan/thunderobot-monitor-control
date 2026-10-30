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
import { Button } from "@/components/ui/button";

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
    class="w-full max-w-md p-0 m-auto overflow-hidden duration-150 border shadow-lg rounded-lg bg-card border-border text-card-foreground animate-in fade-in zoom-in-95"
  >
    <div class="p-6 space-y-5">
      <!-- Header -->
      <div
        class="flex items-center justify-between pb-3 border-b border-border"
      >
        <div class="flex items-center space-x-2.5">
          <div
            class="p-2 border rounded-lg bg-muted text-foreground border-border"
          >
            <BookmarkPlus class="w-5 h-5" />
          </div>
          <div>
            <h3 class="text-base font-semibold text-foreground">Save Current Preset</h3>
            <p class="text-xs text-muted-foreground">
              Capture active hardware settings into a 1-click profile
            </p>
          </div>
        </div>
        <Button
          variant="ghost"
          size="icon"
          @click="emit('close')"
          class="h-8 w-8 text-muted-foreground hover:text-foreground"
        >
          <X class="w-4 h-4" />
        </Button>
      </div>

      <!-- Settings Snapshot Badge -->
      <div
        class="p-3 space-y-1 font-mono text-xs border rounded-lg bg-muted/40 border-border text-foreground"
      >
        <div
          class="text-[11px] text-muted-foreground uppercase font-semibold font-sans"
        >
          Snapshot to be saved:
        </div>
        <div class="flex flex-wrap gap-2 text-[11px]">
          <span>Brightness: {{ features?.brightness }}%</span>
          <span>•</span>
          <span>Contrast: {{ features?.contrast }}%</span>
          <span>•</span>
          <span>Mode: 0x{{ Number(features?.picture_mode).toString(16) }}</span>
          <span>•</span>
          <span>Overdrive: {{ features?.overdrive }}</span>
        </div>
      </div>

      <!-- Form Inputs -->
      <div class="space-y-4">
        <div>
          <label class="text-xs font-medium text-foreground block mb-1.5"
            >Preset Name</label
          >
          <input
            type="text"
            v-model="profileName"
            placeholder="e.g. Apex Legends 250Hz or Late Night Work"
            maxlength="50"
            class="w-full bg-background border border-input rounded-md px-3 py-2 text-sm text-foreground placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 ring-offset-background transition"
          />
        </div>

        <div>
          <label class="text-xs font-medium text-foreground block mb-1.5"
            >Description (Optional)</label
          >
          <input
            type="text"
            v-model="profileDesc"
            placeholder="e.g. High refresh rate with maximized visibility"
            maxlength="100"
            class="w-full bg-background border border-input rounded-md px-3 py-2 text-sm text-foreground placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 ring-offset-background transition"
          />
        </div>

        <!-- Icon Picker -->
        <div>
          <label class="text-xs font-medium text-foreground block mb-1.5"
            >Choose Icon</label
          >
          <div class="flex items-center space-x-2">
            <button
              v-for="item in icons"
              :key="item.name"
              type="button"
              @click="selectedIcon = item.name"
              :class="[
                'p-2.5 rounded-lg border flex flex-col items-center justify-center transition select-none cursor-pointer',
                selectedIcon === item.name
                  ? 'bg-accent border-primary text-accent-foreground ring-1 ring-primary/20'
                  : 'bg-card border-border text-muted-foreground hover:text-foreground',
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
        class="flex items-center justify-end space-x-2 pt-3 border-t border-border"
      >
        <Button
          variant="outline"
          size="sm"
          @click="emit('close')"
        >
          Cancel
        </Button>
        <Button
          variant="default"
          size="sm"
          @click="handleSave"
          :disabled="!profileName.trim()"
        >
          <Check class="w-4 h-4 mr-1.5" />
          <span>Save Preset</span>
        </Button>
      </div>
    </div>
  </dialog>
</template>
