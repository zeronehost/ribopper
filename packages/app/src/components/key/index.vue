<template>
  <s-tr>
    <s-td class="index" @click="editHandle">
      <s-icon>
        <svg viewBox="0 -960 960 960">
          <path
            d="M200-200h57l391-391-57-57-391 391v57Zm-80 80v-170l528-527q12-11 26.5-17t30.5-6q16 0 31 6t26 18l55 56q12 11 17.5 26t5.5 30q0 16-5.5 30.5T817-647L290-120H120Zm640-584-56-56 56 56Zm-141 85-28-29 57 57-29-28Z">
          </path>
        </svg>
      </s-icon>
    </s-td>
    <s-td class="option">{{ desc }}</s-td>
    <s-td class="key">
      <kbd v-if="data?.ctrlKey">Ctrl</kbd>
      <kbd v-if="data?.shiftKey">Shift</kbd>
      <kbd v-if="data?.altKey">Alt</kbd>
      <kbd v-if="data?.metaKey">Win/Super</kbd>
      <kbd v-if="data?.key">{{ data.key }}</kbd>
    </s-td>
  </s-tr>
</template>
<script setup lang="ts">
import type { RiboHotkey, RiboKey } from '@ribo/api';

  defineOptions({
    name: "riboKey"
  });
  const props = defineProps<{
    desc: string,
    data?: RiboKey,
    label: keyof RiboHotkey,
  }>();
  const emit = defineEmits<{
    (e: 'edit', label: keyof RiboHotkey): void,
  }>();

  const editHandle = () => {
    emit("edit", props.label);
  }
</script>