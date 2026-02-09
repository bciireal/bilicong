<script setup>
import default_icon from "../assets/items.svg";

defineProps({
  icon: { type: String, default: default_icon },
  msg: { type: String, default: "" },
  spinIcon: { type: Boolean, default: false },
});
</script>

<template>
  <div class="flex flex-col gap-6 items-center pt-20">
    <img
      :src="icon"
      alt="Icon"
      class="w-20 opacity-75"
      :class="spinIcon ? 'animate-spin' : ''"
    />
    <p class="mx-10">{{ msg === "" ? "无信息" : msg }}</p>
  </div>
</template>

<style scoped>
.animate-spin {
  --animate-spin: spin 1s ease-in-out infinite;
}
</style>
