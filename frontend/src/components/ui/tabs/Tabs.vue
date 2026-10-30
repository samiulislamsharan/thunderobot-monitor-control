<script setup>
import { TabsRoot } from 'radix-vue';
import { cn } from '@/lib/utils';

const props = defineProps({
  defaultValue: {
    type: String,
    default: '',
  },
  modelValue: {
    type: String,
    default: undefined,
  },
  class: {
    type: [String, Object, Array],
    default: '',
  },
});

const emit = defineEmits(['update:modelValue']);
</script>

<template>
  <TabsRoot
    :default-value="defaultValue"
    :model-value="modelValue"
    @update:model-value="emit('update:modelValue', $event)"
    :class="cn('w-full', props.class)"
  >
    <slot />
  </TabsRoot>
</template>
