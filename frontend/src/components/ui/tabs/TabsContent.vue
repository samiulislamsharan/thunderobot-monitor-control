<script setup>
import { TabsContent } from 'radix-vue';
import { cn } from '@/lib/utils';

const props = defineProps({
  value: {
    type: String,
    required: true,
  },
  class: {
    type: [String, Object, Array],
    default: '',
  },
});
</script>

<template>
  <TabsContent
    :value="value"
    :class="
      cn(
        'mt-4 ring-offset-background focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-cyan-400 focus-visible:ring-offset-2',
        props.class
      )
    "
  >
    <slot />
  </TabsContent>
</template>
