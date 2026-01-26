<script setup>
defineProps({
  msg: { type: String, default: "" },
});
</script>
<template>
  <footer
    class="fixed w-full h-[1.7rem] p-1 bottom-0 text-sm bg-zinc-100 border-t border-t-zinc-500"
  >
    {{ msg }}
  </footer>
</template>
