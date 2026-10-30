<script setup>
import { ref, watch } from "vue";
import { AlertTriangle, X } from "lucide-vue-next";
import { Button } from "@/components/ui/button";

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
    class="w-full max-w-sm p-0 m-auto overflow-hidden duration-150 border shadow-lg rounded-lg bg-card border-border text-card-foreground animate-in fade-in zoom-in-95"
  >
    <div class="p-6 space-y-4">
      <div class="flex items-start space-x-3">
        <div
          class="p-2 rounded-md bg-destructive/10 text-destructive shrink-0"
        >
          <AlertTriangle class="w-5 h-5" />
        </div>
        <div>
          <h3 class="text-base font-semibold text-foreground">
            {{ config?.title || "Confirm Action" }}
          </h3>
          <p class="mt-1 text-xs leading-relaxed text-muted-foreground">
            {{ config?.message }}
          </p>
        </div>
      </div>

      <div
        class="flex items-center justify-end space-x-2 pt-4 border-t border-border"
      >
        <Button
          variant="outline"
          size="sm"
          @click="emit('close')"
        >
          Cancel
        </Button>
        <Button
          variant="destructive"
          size="sm"
          @click="emit('confirm')"
        >
          {{ config?.confirmText || "Confirm" }}
        </Button>
      </div>
    </div>
  </dialog>
</template>
