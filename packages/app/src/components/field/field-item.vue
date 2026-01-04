<template>
  <div class="ribo-field-item">
    <div clas="ribo-field-item__title">
      <slot name="title">{{ title }}</slot>
    </div>
    <div clas="ribo-field-item__content">
      <slot>{{ content }}</slot>
    </div>
  </div>
</template>
<script setup lang="ts">
defineOptions({
  name: "RiboFieldItem",
});

defineProps({
  title: String,
  content: String,
});
</script>
<style lang="scss">
  .ribo-field-item {
    display: flex;
    align-items: center;
    justify-content: space-between;
    &+& {
      border-top: solid 1px var(--s-color-outline-variant, #C0C8CC);
      margin-top: 0.5rem;
      padding-top: .5rem;
    }
    &__title {
      font-weight: 700;
      color: var(--s-color-on-surface-variant, #40484C);
    }
  }
</style>