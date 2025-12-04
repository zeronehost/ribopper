<template>
  <section class="ribo-section">
    <s-appbar>
      <span slot="headline">{{title}}</span>
    </s-appbar>
    <s-scroll-view>
      <slot />
    </s-scroll-view>
  </section>
</template>
<script setup lang="ts">
defineProps<{
  title: string,
}>();
defineOptions({
  name: 'RiboSection'
})
</script>
<style lang="scss">
section {
  &.ribo-section {
    s-scroll-view {
      display: flex;
      flex-direction: column;
    }
  }
}
</style>