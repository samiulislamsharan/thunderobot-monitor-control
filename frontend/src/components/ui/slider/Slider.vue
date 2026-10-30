<script setup>
import { SliderRoot, SliderTrack, SliderRange, SliderThumb } from 'radix-vue';
import { cn } from '@/lib/utils';

const props = defineProps({
  modelValue: {
    type: Array,
    default: () => [0],
  },
  min: {
    type: Number,
    default: 0,
  },
  max: {
    type: Number,
    default: 100,
  },
  step: {
    type: Number,
    default: 1,
  },
  class: {
    type: [String, Object, Array],
    default: '',
  },
  variant: {
    type: String,
    default: 'default',
  },
});

const emit = defineEmits(['update:modelValue']);
</script>

<template>
  <SliderRoot
    :model-value="modelValue"
    :min="min"
    :max="max"
    :step="step"
    @update:model-value="emit('update:modelValue', $event)"
    :class="cn('relative flex w-full touch-none select-none items-center py-2 cursor-pointer', props.class)"
  >
    <SliderTrack class="relative h-1.5 w-full grow overflow-hidden rounded-full bg-secondary">
      <SliderRange
        :class="
          cn('absolute h-full', {
            'bg-primary': variant === 'default',
            'bg-red-500': variant === 'red',
            'bg-emerald-500': variant === 'green',
            'bg-blue-500': variant === 'blue',
          })
        "
      />
    </SliderTrack>
    <SliderThumb
      v-for="(_, index) in modelValue"
      :key="index"
      :class="
        cn(
          'block h-4 w-4 rounded-full border border-primary/50 bg-background shadow transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-50 cursor-grab active:cursor-grabbing hover:scale-110',
          {
            'border-primary': variant === 'default',
            'border-red-500 bg-red-500': variant === 'red',
            'border-emerald-500 bg-emerald-500': variant === 'green',
            'border-blue-500 bg-blue-500': variant === 'blue',
          }
        )
      "
    />
  </SliderRoot>
</template>
