<template>
  <dialog class="ribo-dialog-update" ref="rootEl">
    <div class="ribo-dialog-update__container">
      <s-linear-progress :indeterminate :value animated></s-linear-progress>
      <div class="progress" v-if="!indeterminate">{{ downloaded }} / {{ total }}</div>
      <div class="progress" v-else>正在更新...</div>
    </div>
  </dialog>
</template>
<script setup lang="ts">
import { ref, watch } from 'vue';

defineOptions({
  name: "RiboDialogUpdate",
});

const props = defineProps({
  modelValue: {
    type: Boolean,
    default: false
  },
  value: {
    type: Number,
    default: 0
  },
  total: {
    type: Number,
    default: 0
  },
  downloaded: {
    type: Number,
    default: 0
  },
  indeterminate: {
    type: Boolean,
    default: false
  }
});
const rootEl = ref<HTMLDialogElement>();
watch(() => props.modelValue, (val) => {
  if (val) {
    rootEl.value?.show();
  } else {
    rootEl.value?.close();
  }
});
</script>
<style lang="scss">
.ribo-dialog-update {
  border: none;
  padding: 0;
  z-index: 100;
  background: transparent;
  position: fixed;
  &__container {
    position: fixed;
    top: 0;
    left: 0;
    width: 100vw;
    height: 100vh;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    background-color: rgba(var(--s-color-background), 0.1);
    backdrop-filter: blur(10px);
  }

  &:focus-visible {
    outline: none;
  }

  s-linear-progress {
    width: 80%;
    height: 0.5rem;
    border-radius: 1rem;
  }

  .progress {
    font-weight: bold;
    color: var(--s-color-primary);
    line-height: 2.5rem;
  }
}
</style>