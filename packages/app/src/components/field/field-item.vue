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
    name: "RiboFieldItem"
  });

  defineProps({
    title: String,
    content: String,
  })
</script>
<style lang="scss">
  .ribo-field-item {
    display: flex;
    align-items: center;
    justify-content: space-between;
    &__title {
      font-weight: 700;
    }

    &+& {
      margin-top: 1rem;
    }
  }
</style>