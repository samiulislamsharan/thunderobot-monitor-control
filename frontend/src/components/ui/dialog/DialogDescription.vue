<script setup>
import { DialogDescription } from 'radix-vue';
import { cn } from '@/lib/utils';

const props = defineProps({
  asChild: { type: Boolean, default: false },
  as: { type: null, default: undefined },
  class: { type: [String, Object, Array], default: '' },
});
</script>

<template>
  <DialogDescription
    :as="as"
    :as-child="asChild"
    :class="cn('text-xs text-slate-400', props.class)"
  >
    <slot />
  </DialogDescription>
</template>
