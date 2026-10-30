<script setup>
import { DialogTitle } from 'radix-vue';
import { cn } from '@/lib/utils';

const props = defineProps({
  asChild: { type: Boolean, default: false },
  as: { type: null, default: undefined },
  class: { type: [String, Object, Array], default: '' },
});
</script>

<template>
  <DialogTitle
    :as="as"
    :as-child="asChild"
    :class="cn('text-lg font-bold leading-none tracking-tight text-white', props.class)"
  >
    <slot />
  </DialogTitle>
</template>
