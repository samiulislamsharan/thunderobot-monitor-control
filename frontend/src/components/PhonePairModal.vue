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
import { Button } from "@/components/ui/button";

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
        dark: "#09090b",
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
    class="rounded-lg bg-card border border-border text-card-foreground p-0 shadow-lg max-w-md w-full m-auto overflow-hidden animate-in fade-in zoom-in-95 duration-200"
  >
    <div class="p-6">
      <!-- Header -->
      <div
        class="flex items-center justify-between pb-4 border-b border-border"
      >
        <div class="flex items-center space-x-2.5">
          <img
            src="/icons/icon-192.png"
            alt="ThundeRobot Logo"
            class="w-9 h-9 rounded-md object-cover border border-border"
          />
          <div>
            <h3 class="text-base font-semibold text-foreground">Connect Phone Remote</h3>
            <p class="text-xs text-muted-foreground">
              Control monitor from any mobile device
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

      <!-- QR Code Container -->
      <div class="my-6 flex flex-col items-center justify-center">
        <div
          class="p-3 bg-white rounded-md shadow-sm border border-border"
        >
          <canvas ref="canvasRef" class="rounded"></canvas>
        </div>
        <p
          class="mt-3 text-xs text-muted-foreground text-center flex items-center gap-1.5"
        >
          <Wifi class="w-3.5 h-3.5 text-foreground" />
          Ensure phone is connected to the same Wi-Fi
        </p>
      </div>

      <!-- LAN URL Display & Copy -->
      <div class="space-y-2">
        <label
          class="text-xs font-medium text-muted-foreground uppercase tracking-wider block"
        >
          Local Network URL
        </label>
        <div class="flex items-center space-x-2">
          <div
            class="flex-1 bg-muted border border-input rounded-md px-3 py-2 text-xs font-mono text-foreground truncate"
          >
            {{ phoneUrl }}
          </div>
          <Button
            variant="outline"
            size="sm"
            @click="copyUrl"
            class="gap-1.5"
          >
            <Check v-if="copied" class="w-4 h-4 text-emerald-500" />
            <Copy v-else class="w-4 h-4" />
            <span>{{ copied ? "Copied!" : "Copy" }}</span>
          </Button>
        </div>
      </div>

      <!-- Tips for PWA -->
      <div
        class="mt-5 p-3 rounded-md bg-muted/40 border border-border text-xs text-muted-foreground space-y-1"
      >
        <p class="font-medium text-foreground">💡 Pro Tip:</p>
        <p>
          After opening on your phone, tap
          <span class="text-foreground font-semibold">"Add to Home Screen"</span>
          in Safari or Chrome for a full-screen, native-app experience!
        </p>
      </div>
    </div>
  </dialog>
</template>
