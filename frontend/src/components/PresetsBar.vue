<script setup>
import {
  Crosshair,
  Moon,
  Film,
  Monitor,
  Sparkles,
  Plus,
  Trash2,
  Bookmark,
} from "lucide-vue-next";
import { Button } from "@/components/ui/button";
import { Badge } from "@/components/ui/badge";

const props = defineProps({
  profiles: Array,
  activeProfileId: String,
  updating: Boolean,
});

const emit = defineEmits(["applyProfile", "deleteProfile", "openSaveModal"]);

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
</script>

<template>
  <div class="space-y-2.5">
    <div class="flex items-center justify-between">
      <div class="flex items-center space-x-2">
        <Bookmark class="w-4 h-4 text-foreground" />
        <h2
          class="text-xs font-semibold tracking-wider uppercase text-muted-foreground"
        >
          Quick Profiles & Presets
        </h2>
      </div>
      <Button
        variant="outline"
        size="sm"
        @click="$emit('openSaveModal')"
      >
        <Plus class="w-3.5 h-3.5 mr-1.5" />
        <span>Save Current</span>
      </Button>
    </div>

    <!-- Presets Grid -->
    <div class="grid grid-cols-2 sm:grid-cols-2 lg:grid-cols-4 gap-2.5">
      <div
        v-for="profile in profiles"
        :key="profile.id"
        @click="$emit('applyProfile', profile.id)"
        :class="[
          'group relative p-3.5 rounded-lg border text-left cursor-pointer transition-all duration-200 select-none overflow-hidden',
          activeProfileId === profile.id
            ? 'bg-accent border-primary ring-1 ring-primary/20 shadow-sm'
            : 'bg-card hover:bg-accent/40 border-border',
        ]"
      >
        <!-- Header: Icon & Badge -->
        <div class="flex items-center justify-between mb-2">
          <div
            class="p-2 rounded-lg bg-muted text-foreground border border-border"
          >
            <component :is="getIcon(profile.icon)" class="w-4 h-4" />
          </div>

          <div class="flex items-center space-x-1.5">
            <Badge
              :variant="
                activeProfileId === profile.id ? 'default' : 'secondary'
              "
              class="text-[10px] tracking-wide uppercase font-semibold"
            >
              {{ profile.badge || "Preset" }}
            </Badge>
            <!-- Custom profile delete button -->
            <button
              v-if="!profile.built_in"
              @click.stop="$emit('deleteProfile', profile.id)"
              class="p-1 transition opacity-0 group-hover:opacity-100 text-muted-foreground hover:text-destructive"
              title="Delete Custom Preset"
            >
              <Trash2 class="w-3.5 h-3.5" />
            </button>
          </div>
        </div>

        <!-- Name & Summary -->
        <h3
          class="text-xs font-semibold text-foreground truncate sm:text-sm"
        >
          {{ profile.name }}
        </h3>
        <p class="text-[11px] text-muted-foreground line-clamp-1 mt-0.5">
          {{ profile.description }}
        </p>
      </div>
    </div>
  </div>
</template>
