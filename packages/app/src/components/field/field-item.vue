<template>
  <div class="ribo-field-item">
    <div class="ribo-field-item__title">
      <slot name="title">{{ title }}</slot>
    </div>
    <div class="ribo-field-item__wrapper">
      <div class="ribo-field-item__content">
        <slot>{{ content }}</slot>
      </div>
      <div class="ribo-field-item__tip" v-if="tip || $slots.tip">
        <slot name="tip">{{ tip }}</slot>
      </div>
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
  tip: String,
});
</script>
<style lang="scss">
.ribo-field-item {
  display: flex;
  align-items: center;
  justify-content: space-between;

  &+& {
    margin-top: 0.5rem;
  }

  &__title {
    font-weight: 700;
    width: 10rem;
    color: var(--s-color-on-surface-variant, #40484C);
    align-self: start;
    line-height: 2rem;
  }

  &__wrapper {
    flex: 1;
    display: flex;
    flex-direction: column;
  }
  &__content {
    align-self: flex-end;
  }
  &__tip {
    align-self: flex-start;
    width: 100%;
    color: var(--s-color-tertiary);
  }
}
</style>