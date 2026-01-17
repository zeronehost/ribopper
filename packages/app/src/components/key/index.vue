<template>
  <s-tr class="ribo-key" :class="{ disabled, selected }">
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
      <kbd v-if="data?.ctrl">Ctrl</kbd>
      <kbd v-if="data?.shift">Shift</kbd>
      <kbd v-if="data?.alt">Alt</kbd>
      <kbd v-if="data?.meta">Win/Super</kbd>
      <kbd v-if="data?.key">{{ data.key }}</kbd>
    </s-td>
  </s-tr>
</template>
<script setup lang="ts">
import { Hotkey, Key } from '@ribo/api';

defineOptions({
  name: "riboKey"
});
const props = defineProps<{
  desc: string,
  data?: Key,
  label: keyof Hotkey,
  disabled?: boolean,
  selected?: boolean
}>();
const emit = defineEmits<{
  (e: 'edit', label: keyof Hotkey): void,
}>();

const editHandle = () => {
  if (props.disabled) return;
  emit("edit", props.label);
}
</script>
<style scoped lang="scss">
.ribo-key {
  &.disabled {
    cursor: not-allowed;
    color: color-mix(in srgb, var(--s-color-on-surface, #191C1E) 38%, transparent);

    * {
      cursor: not-allowed;
      color: color-mix(in srgb, var(--s-color-on-surface, #191C1E) 38%, transparent);
    }
  }

  &:not(.disabled) {
    &:hover {
      s-icon {
        visibility: visible;
      }

      .key {
        cursor: text;

        * {
          cursor: text;
        }
      }
    }
    &.selected {
      s-icon {
        visibility: visible;
      }
    }
  }

  s-td {
    &:not(:last-child) {
      border-right: solid 1px var(--s-color-outline-variant, #C0C8CC);
    }

    s-icon {
      width: 1rem;
      height: 1rem;
      visibility: hidden;
    }

    kbd {
      background-color: var(--s-color-surface-container-low);
      padding: 3px 5px;
      border-radius: 4px;

      &:not(:last-child) {
        margin-right: 5px;
      }
    }

  }

  .index {
    width: 3rem;
  }

  .option {
    width: 10rem;
  }
}
</style>