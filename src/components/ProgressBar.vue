<script setup>
import { computed } from "vue";

const props = defineProps({
  value: { type: Number, required: true },
  total: { type: Number, required: true },
});

const progress_raw = computed(() => {
  if (props.total === 0) {
    return 0;
  }

  let v = props.value / props.total;
  if (v <= 0) {
    v = 0;
  }
  if (v >= 1) {
    v = 1;
  }
  return v;
});

const progress_floor = computed(() => {
  return Math.floor(progress_raw.value * 100);
});

const progress_style = computed(() => {
  return `width: ${(0.98 * progress_raw.value + 0.02) * 100}%;`;
});
</script>

<template>
  <div class="flex items-center gap-5">
    <div class="progress-outer">
      <div class="progress-inner" :style="progress_style"></div>
    </div>
    <div>共 {{ total }}, 已完成 {{ value }}, 占 {{ progress_floor }} %</div>
  </div>
</template>

<style scoped>
@reference "../style.css";

@utility progress-shape {
  @apply w-[50vw] h-3 shadow-md rounded-full transition-[width];
}

.progress-outer {
  @apply progress-shape bg-zinc-300;
}

.progress-inner {
  @apply progress-shape bg-cyan-500;
}
</style>
