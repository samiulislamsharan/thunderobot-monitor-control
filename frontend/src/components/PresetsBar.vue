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
        <Bookmark class="w-4 h-4 text-cyan-400" />
        <h2
          class="text-sm font-semibold uppercase tracking-wider text-slate-300"
        >
          Quick Profiles & Presets
        </h2>
      </div>
      <button
        @click="$emit('openSaveModal')"
        class="inline-flex items-center space-x-1.5 px-3 py-1 rounded-lg text-xs font-medium bg-slate-900 hover:bg-slate-800 border border-slate-700 text-cyan-400 hover:border-cyan-500/50 transition"
      >
        <Plus class="w-3.5 h-3.5" />
        <span>Save Current</span>
      </button>
    </div>

    <!-- Presets Grid -->
    <div class="grid grid-cols-2 sm:grid-cols-2 lg:grid-cols-4 gap-2.5">
      <div
        v-for="profile in profiles"
        :key="profile.id"
        @click="$emit('applyProfile', profile.id)"
        :class="[
          'group relative p-3 rounded-xl border text-left cursor-pointer transition-all duration-200 select-none overflow-hidden',
          activeProfileId === profile.id
            ? 'bg-slate-900/90 border-cyan-400 shadow-cyan-glow ring-1 ring-cyan-400/50'
            : 'bg-cyber-card/60 hover:bg-slate-800/80 border-slate-800/80 hover:border-slate-700',
        ]"
      >
        <!-- Header: Icon & Badge -->
        <div class="flex items-center justify-between mb-2">
          <div
            class="p-2 rounded-lg"
            :style="{
              backgroundColor: `${profile.color || '#00f0ff'}15`,
              color: profile.color || '#00f0ff',
              border: `1px solid ${profile.color || '#00f0ff'}30`,
            }"
          >
            <component :is="getIcon(profile.icon)" class="w-4 h-4" />
          </div>

          <div class="flex items-center space-x-1.5">
            <span
              class="text-[10px] font-bold px-1.5 py-0.5 rounded tracking-wide uppercase"
              :class="
                activeProfileId === profile.id
                  ? 'bg-cyan-950 text-cyan-300 border border-cyan-500/40'
                  : 'bg-slate-800 text-slate-400'
              "
            >
              {{ profile.badge || "Preset" }}
            </span>
            <!-- Custom profile delete button -->
            <button
              v-if="!profile.built_in"
              @click.stop="$emit('deleteProfile', profile.id)"
              class="opacity-0 group-hover:opacity-100 p-1 text-slate-500 hover:text-red-400 transition"
              title="Delete Custom Preset"
            >
              <Trash2 class="w-3.5 h-3.5" />
            </button>
          </div>
        </div>

        <!-- Name & Summary -->
        <h3
          class="text-xs sm:text-sm font-bold text-white group-hover:text-cyan-300 transition-colors truncate"
        >
          {{ profile.name }}
        </h3>
        <p class="text-[11px] text-slate-400 line-clamp-1 mt-0.5">
          {{ profile.description }}
        </p>

        <!-- Active indicator line -->
        <div
          v-if="activeProfileId === profile.id"
          class="absolute bottom-0 left-0 right-0 h-0.5 bg-cyan-400"
        ></div>
      </div>
    </div>
  </div>
</template>
