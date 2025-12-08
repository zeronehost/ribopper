<template>
  <s-card class="ribo-field" :class="{'has-title': !!title}">
    <div class="ribo-field__title" v-if="title">{{ title }}</div>
    <slot />
  </s-card>
</template>
<script setup lang="ts">
defineOptions({
  name: "RiboField",
});
defineProps({
  title: String,
});
</script>
<style lang="scss">
  .ribo-field {
    display: flex;
    flex-direction: column;

    &__title {
      font-weight: 700;
      font-size: 18px;
    }
    &.has-title {
      .ribo-field-item {
        margin-left: 2rem;
      }
    }
  }
</style>