<script setup>
import { ref, onMounted } from "vue";
import { useMonitor } from "./composables/useMonitor";
import Header from "./components/Header.vue";
import PresetsBar from "./components/PresetsBar.vue";
import PictureControls from "./components/PictureControls.vue";
import ColorControls from "./components/ColorControls.vue";
import GamingControls from "./components/GamingControls.vue";
import InputPower from "./components/InputPower.vue";
import RawVcpInspector from "./components/RawVcpInspector.vue";
import PhonePairModal from "./components/PhonePairModal.vue";
import ConfirmModal from "./components/ConfirmModal.vue";
import SaveProfileModal from "./components/SaveProfileModal.vue";
import { CheckCircle2, AlertCircle, Info, RefreshCw } from "lucide-vue-next";

const {
  monitorInfo,
  features,
  profiles,
  activeProfileId,
  loading,
  updating,
  connected,
  toasts,
  setFeature,
  applyProfile,
  createProfile,
  deleteProfile,
  resetMonitor,
  setRawVcp,
  init,
} = useMonitor();

// Modal States
const isPhoneModalOpen = ref(false);
const isSaveModalOpen = ref(false);
const isConfirmModalOpen = ref(false);
const confirmConfig = ref(null);

onMounted(async () => {
  await init();
  // Handle PWA shortcut parameters (e.g. ?profile=preset-fps-250)
  const params = new URLSearchParams(window.location.search);
  const targetProfile = params.get("profile");
  if (targetProfile) {
    setTimeout(() => {
      applyProfile(targetProfile);
    }, 500);
  }
});

const openConfirmModal = (config) => {
  confirmConfig.value = config;
  isConfirmModalOpen.value = true;
};

const handleConfirm = () => {
  if (confirmConfig.value?.action) {
    confirmConfig.value.action();
  }
  isConfirmModalOpen.value = false;
};

const handleSaveProfile = async (profileData) => {
  await createProfile(
    profileData.name,
    profileData.description,
    profileData.icon,
  );
  isSaveModalOpen.value = false;
};
</script>

<template>
  <div
    class="min-h-screen flex flex-col bg-cyber-bg text-slate-100 pb-12 selection:bg-cyan-500 selection:text-black"
  >
    <!-- Top Header & Telemetry -->
    <Header
      :monitorInfo="monitorInfo"
      :connected="connected"
      :updating="updating"
      @openPhoneModal="isPhoneModalOpen = true"
      @refresh="init"
    />

    <!-- Main Container -->
    <main class="flex-1 max-w-7xl w-full mx-auto px-4 lg:px-8 py-6 space-y-6">
      <!-- Loading State -->
      <div
        v-if="loading"
        class="flex flex-col items-center justify-center py-24 space-y-4"
      >
        <RefreshCw class="w-10 h-10 text-cyan-400 animate-spin" />
        <p class="text-sm font-semibold tracking-wide text-slate-400">
          Probing ThundeRobot DQ34C240L DDC/CI Hardware Bus...
        </p>
      </div>

      <template v-else>
        <!-- Quick Presets Carousel / Bar -->
        <PresetsBar
          :profiles="profiles"
          :activeProfileId="activeProfileId"
          :updating="updating"
          @applyProfile="applyProfile"
          @deleteProfile="deleteProfile"
          @openSaveModal="isSaveModalOpen = true"
        />

        <!-- Controls Grid -->
        <div class="grid grid-cols-1 gap-6">
          <!-- Picture & Luminance -->
          <PictureControls
            :features="features"
            :updating="updating"
            @setFeature="setFeature"
            @resetLuminance="resetMonitor('luminance')"
          />

          <!-- Gaming & Motion 250Hz Overclock Tuner -->
          <GamingControls
            :features="features"
            :updating="updating"
            @setFeature="setFeature"
          />

          <!-- Color Calibration & RGB Gains -->
          <ColorControls
            :features="features"
            :updating="updating"
            @setFeature="setFeature"
            @resetColor="resetMonitor('color')"
          />

          <!-- Input Source & Power Management -->
          <InputPower
            :features="features"
            :updating="updating"
            @setFeature="setFeature"
            @openConfirmModal="openConfirmModal"
            @resetFactory="resetMonitor('factory')"
          />

          <!-- Advanced Hex VCP Inspector -->
          <RawVcpInspector :updating="updating" @setRawVcp="setRawVcp" />
        </div>
      </template>
    </main>

    <!-- Modals -->
    <PhonePairModal
      :isOpen="isPhoneModalOpen"
      :phoneUrl="monitorInfo.phone_url"
      :lanIp="monitorInfo.lan_ip"
      @close="isPhoneModalOpen = false"
    />

    <ConfirmModal
      :isOpen="isConfirmModalOpen"
      :config="confirmConfig"
      @close="isConfirmModalOpen = false"
      @confirm="handleConfirm"
    />

    <SaveProfileModal
      :isOpen="isSaveModalOpen"
      :features="features"
      @close="isSaveModalOpen = false"
      @save="handleSaveProfile"
    />

    <!-- Toast Notifications -->
    <div
      class="fixed bottom-4 right-4 z-50 flex flex-col space-y-2 pointer-events-none max-w-sm w-full"
    >
      <div
        v-for="toast in toasts"
        :key="toast.id"
        class="pointer-events-auto p-3.5 rounded-xl border shadow-xl flex items-center space-x-2.5 backdrop-blur-md transition-all animate-in slide-in-from-bottom-2 duration-150"
        :class="{
          'bg-emerald-950/90 border-emerald-500/50 text-emerald-200':
            toast.type === 'success',
          'bg-red-950/90 border-red-500/50 text-red-200':
            toast.type === 'error',
          'bg-slate-900/90 border-slate-700 text-slate-200':
            toast.type === 'info',
        }"
      >
        <CheckCircle2
          v-if="toast.type === 'success'"
          class="w-4 h-4 text-emerald-400 shrink-0"
        />
        <AlertCircle
          v-else-if="toast.type === 'error'"
          class="w-4 h-4 text-red-400 shrink-0"
        />
        <Info v-else class="w-4 h-4 text-cyan-400 shrink-0" />
        <span class="text-xs font-medium">{{ toast.message }}</span>
      </div>
    </div>
  </div>
</template>
