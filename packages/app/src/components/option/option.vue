<template>
  <div class="ribo-tree-option">
    <div class="ribo-tree-option__body">
      <s-icon>
        <RiboIconTerminal />
      </s-icon>
      <div class="ribo-tree-option__content">
        <div>{{option.command}}</div>
        <div class="description">{{option.description}}</div>
      </div>
    </div>
  </div>
</template>
<script setup lang="ts">
import type { PropType } from "vue";
import { RiboIconTerminal } from "../icons";
import type { Option } from "@ribo/api";
defineOptions({
  name: "RiboOption"
});

defineProps({
  option: {
    type: Object as PropType<Option>,
    default: () => ({})
  }
})

</script>
<style lang="scss">
.ribo-tree-option {
  position: relative;
  padding-left: 1rem;

  &::before {
    content: "";
    position: absolute;
    bottom: 0;
    left: 1rem;
    top: 0;
    width: 1px;
    background-color: var(--s-color-outline);
    z-index: 0;
  }

  &:last-child {
    &::before {
      height: calc(0.75rem + 1px);
    }
  }

  &__body {
    position: relative;
    line-height: 1.5rem;
    padding-left: 0.25rem;
    display: flex;
    align-items: center;

    &::before {
      content: "";
      position: absolute;
      left: 0;
      top: 50%;
      width: 0.5rem;
      height: 1px;
      background-color: var(--s-color-outline);
      z-index: 0;
    }

    s-icon {
      width: 1rem;
      height: 1rem;
      margin: 0 0.25rem;
    }
  }

  &__content {
    flex: 1;
    display: flex;
    align-items: center;

    div {
      width: 50%;
      overflow: hidden; /* 隐藏超出部分 */
      white-space: nowrap; /* 禁止换行 */
      text-overflow: ellipsis; /* 显示省略号 */
    }

    .description {
      width: calc(50% + 24px + 2rem);
    }
  }
}
</style>