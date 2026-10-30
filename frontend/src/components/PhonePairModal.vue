<script setup>
import { ref, watch, onMounted, nextTick } from "vue";
import QRCode from "qrcode";
import {
  Smartphone,
  X,
  Copy,
  Check,
  Wifi,
  ExternalLink,
} from "lucide-vue-next";

const props = defineProps({
  isOpen: Boolean,
  phoneUrl: String,
  lanIp: String,
});

const emit = defineEmits(["close"]);
const dialogRef = ref(null);
const canvasRef = ref(null);
const copied = ref(false);

const renderQr = async () => {
  if (!canvasRef.value || !props.phoneUrl) return;
  try {
    await QRCode.toCanvas(canvasRef.value, props.phoneUrl, {
      width: 200,
      margin: 2,
      color: {
        dark: "#080c14",
        light: "#ffffff",
      },
    });
  } catch (err) {
    console.error("QR code generation failed:", err);
  }
};

watch(
  () => props.isOpen,
  async (open) => {
    if (open) {
      dialogRef.value?.showModal();
      await nextTick();
      renderQr();
    } else {
      dialogRef.value?.close();
    }
  },
);

const copyUrl = async () => {
  try {
    await navigator.clipboard.writeText(props.phoneUrl);
    copied.value = true;
    setTimeout(() => {
      copied.value = false;
    }, 2000);
  } catch (e) {
    console.error(e);
  }
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
    class="rounded-2xl bg-cyber-card border border-cyber-border text-slate-100 p-0 shadow-2xl max-w-md w-full m-auto overflow-hidden animate-in fade-in zoom-in-95 duration-200"
  >
    <div class="p-6">
      <!-- Header -->
      <div
        class="flex items-center justify-between pb-4 border-b border-slate-800"
      >
        <div class="flex items-center space-x-2.5">
          <img
            src="/icons/icon-192.png"
            alt="ThundeRobot Logo"
            class="w-10 h-10 rounded-xl object-cover border border-cyan-500/30"
          />
          <div>
            <h3 class="text-lg font-bold text-white">Connect Phone Remote</h3>
            <p class="text-xs text-slate-400">
              Control monitor from any mobile device
            </p>
          </div>
        </div>
        <button
          @click="emit('close')"
          class="p-1.5 rounded-lg text-slate-400 hover:text-white hover:bg-slate-800 transition"
        >
          <X class="w-5 h-5" />
        </button>
      </div>

      <!-- QR Code Container -->
      <div class="my-6 flex flex-col items-center justify-center">
        <div
          class="p-3 bg-white rounded-xl shadow-lg border-2 border-cyan-400/40"
        >
          <canvas ref="canvasRef" class="rounded"></canvas>
        </div>
        <p
          class="mt-3 text-xs text-slate-400 text-center flex items-center gap-1.5"
        >
          <Wifi class="w-3.5 h-3.5 text-cyan-400" />
          Ensure phone is connected to the same Wi-Fi
        </p>
      </div>

      <!-- LAN URL Display & Copy -->
      <div class="space-y-3">
        <label
          class="text-xs font-semibold text-slate-400 uppercase tracking-wider block"
        >
          Local Network URL
        </label>
        <div class="flex items-center space-x-2">
          <div
            class="flex-1 bg-slate-900 border border-slate-800 rounded-lg px-3 py-2 text-sm font-mono text-cyan-300 truncate"
          >
            {{ phoneUrl }}
          </div>
          <button
            @click="copyUrl"
            class="px-3 py-2 bg-slate-800 hover:bg-slate-700 text-slate-200 rounded-lg text-xs font-medium flex items-center space-x-1.5 transition border border-slate-700"
          >
            <Check v-if="copied" class="w-4 h-4 text-emerald-400" />
            <Copy v-else class="w-4 h-4" />
            <span>{{ copied ? "Copied!" : "Copy" }}</span>
          </button>
        </div>
      </div>

      <!-- Tips for PWA -->
      <div
        class="mt-5 p-3 rounded-xl bg-slate-900/60 border border-slate-800 text-xs text-slate-400 space-y-1"
      >
        <p class="font-medium text-slate-300">💡 Pro Tip:</p>
        <p>
          After opening on your phone, tap
          <span class="text-cyan-400 font-semibold">"Add to Home Screen"</span>
          in Safari or Chrome for a full-screen, native-app experience!
        </p>
      </div>
    </div>
  </dialog>
</template>
