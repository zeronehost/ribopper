<template>
  <main>
    <RiboVirtualList :list="d" />
  </main>
</template>
<script setup lang="ts">
import { RiboVirtualList } from "@/components/scroll-view";
let d = [];
for (let i = 0; i < 1000; i++) {
  d.push({ id: i, value: i });
}
</script>
<style lang="scss" scoped>
main {
  height: 100vh;
}
</style>