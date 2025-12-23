<template>
  <dialog ref="rootEl" class="ribo-key" @close="closeHandle" @keydown.stop.prevent="keydownHandle">
    <header>添加组合键</header>
    <code>
      <kbd v-if="ctrlKey">Ctrl</kbd>
      <kbd v-if="shiftKey">Shift</kbd>
      <kbd v-if="altKey">Alt</kbd>
      <kbd v-if="metaKey">Win/Super</kbd>
      <kbd v-if="key">{{ key }}</kbd>
    </code>
    <footer>
      <s-button type="elevated" @click.stop="cancelHandle">取消</s-button>
      <s-button @click.stop="confirmHandle">确定</s-button>
    </footer>
  </dialog>
</template>
<script setup lang="ts">
defineOptions({
  name: 'RiboKeyDialog',
});
const props = defineProps({
  modelValue: {
    type: Boolean,
    default: false,
  },
});

const emits = defineEmits<{
  (e: 'update:modelValue', value: boolean): void,
  (e: "confirm", value: RiboKey): void,
  (e: "cancel"): void,
  (e: "close"): void,
  (e: "show"): void,
}>();

const rootEl = ref<HTMLDialogElement>();
watch(() => props.modelValue, (value) => {
  if (value) {
    rootEl.value?.showModal();
    emits('show');
    initHandle();
  } else {
    rootEl.value?.close();
  }
});

const closeHandle = () => {
  emits('update:modelValue', false);
  emits('close');
}

const cancelHandle = () => {
  emits("cancel");
  closeHandle();
}

const altKey = ref(false);
const ctrlKey = ref(false);
const metaKey = ref(false);
const shiftKey = ref(false);
const key = ref<string>();

  const initHandle = () => {
    altKey.value = false;
    ctrlKey.value = false;
    metaKey.value = false;
    shiftKey.value = false;
    key.value = undefined;
  }

const confirmHandle = () => {
  emits("confirm", {
    altKey: altKey.value,
    ctrlKey: ctrlKey.value,
    metaKey: metaKey.value,
    shiftKey: shiftKey.value,
    key: key.value
  });
  closeHandle();
}

const keydownHandle = (e: Event) => {
  initHandle();
  altKey.value = (e as KeyboardEvent).altKey;
  ctrlKey.value = (e as KeyboardEvent).ctrlKey;
  metaKey.value = (e as KeyboardEvent).metaKey;
  shiftKey.value = (e as KeyboardEvent).shiftKey;
  key.value = (e as KeyboardEvent).key;
}
</script>
<script lang="ts">
import type { RiboKey } from '@ribo/api';
import { ref, watch } from 'vue';

</script>
<style lang="scss">
dialog.ribo-key {
  border: none;
  padding: 1rem;
  border-radius: 5px;
  background-color: var(--s-color-surface-container-lowest);
  text-align: center;
  outline: none;

  header {
    margin-bottom: 1rem;
    font-size: .8em;
  }

  code {
    display: block;
    height: 1.5rem;
    border-radius: 5px;
    background-color: var(--s-color-surface-container-low);
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 5px;

    kbd {
      background-color: var(--s-color-surface-container-lowest);
      border-radius: 3px;
      font-size: .8em;
      padding: 3px 4px;
    }
  }

  footer {
    margin-top: 1rem;

    s-button {
      border-radius: 5px;

      &:not(:first-child) {
        margin-left: 1rem;
      }
    }
  }

  &:focus-visible {
    code {
      outline: 2px solid var(--s-color-primary);
    }
  }
}
</style>