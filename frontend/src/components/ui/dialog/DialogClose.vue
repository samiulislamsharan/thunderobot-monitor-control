<script setup>
import { DialogClose } from 'radix-vue';

const props = defineProps({
  asChild: { type: Boolean, default: false },
  as: { type: null, default: undefined },
});
</script>

<template>
  <DialogClose :as="as" :as-child="asChild">
    <slot />
  </DialogClose>
</template>
