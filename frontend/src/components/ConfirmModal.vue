<script setup>
import { ref, watch } from "vue";
import { AlertTriangle, X } from "lucide-vue-next";

const props = defineProps({
  isOpen: Boolean,
  config: Object,
});

const emit = defineEmits(["close", "confirm"]);
const dialogRef = ref(null);

watch(
  () => props.isOpen,
  (open) => {
    if (open) {
      dialogRef.value?.showModal();
    } else {
      dialogRef.value?.close();
    }
  },
);

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
    class="rounded-2xl bg-cyber-card border border-cyber-border text-slate-100 p-0 shadow-2xl max-w-sm w-full m-auto overflow-hidden animate-in fade-in zoom-in-95 duration-150"
  >
    <div class="p-5 space-y-4">
      <div class="flex items-start space-x-3">
        <div
          class="p-2.5 rounded-xl bg-amber-500/10 text-amber-400 border border-amber-500/20 shrink-0"
        >
          <AlertTriangle class="w-5 h-5" />
        </div>
        <div>
          <h3 class="text-base font-bold text-white">
            {{ config?.title || "Confirm Action" }}
          </h3>
          <p class="text-xs text-slate-400 mt-1 leading-relaxed">
            {{ config?.message }}
          </p>
        </div>
      </div>

      <div
        class="flex items-center justify-end space-x-2.5 pt-2 border-t border-slate-800"
      >
        <button
          @click="emit('close')"
          class="px-3.5 py-2 rounded-xl text-xs font-semibold text-slate-400 hover:text-white hover:bg-slate-800 transition"
        >
          Cancel
        </button>
        <button
          @click="emit('confirm')"
          :class="config?.confirmColor || 'bg-cyan-500 text-slate-950'"
          class="px-4 py-2 rounded-xl text-xs font-bold transition shadow-sm"
        >
          {{ config?.confirmText || "Confirm" }}
        </button>
      </div>
    </div>
  </dialog>
</template>
