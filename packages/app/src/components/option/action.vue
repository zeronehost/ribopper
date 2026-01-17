<template>
  <div class="ribo-tree-action">
    <div class="ribo-tree-action__body" :class="{selected}">
      <s-icon :name="folded ? 'chevron_right' : 'chevron_down'" @click="expandHandle"></s-icon>
      <div class="ribo-tree-action__content" @click.stop="selectHandle">
        <div>{{data.pattern}}</div>
        <div class="description">{{data.description}}</div>
      </div>
    </div>
    <s-fold :folded="folded" class="ribo-tree-action__children">
      <RiboOption v-for="(option, key) in data.options" :key :option />
    </s-fold>
  </div>
</template>
<script setup lang="ts">
import { inject, ref, type PropType } from 'vue';
import RiboOption from "./option.vue";
import type { Action } from '@ribo/api';
import { optionContainerKey, type OptionContainer } from './utils';

defineOptions({
  name: "RiboAction"
});

const props = defineProps({
  data: {
    type: Object as PropType<Action>,
    default: () => ({})
  },
  selected: {
    type: Boolean,
    default: false
  }
});

const container = inject<OptionContainer>(optionContainerKey);

const folded = ref(true);

const expandHandle = () => {
  folded.value = !folded.value;
}

const selectHandle = () => {
  container?.selected(props.data);
}
</script>
<style lang="scss">
.ribo-tree-action {
  position: relative;
  // padding-left: 1rem;

  &::before {
    content: "";
    position: absolute;
    bottom: 0;
    left: 1rem;
    top: 0;
    width: 1px;
    background-color: var(--s-color-outline);
    z-index: 1;
  }

  &:last-child {
    &::before {
      height: calc(0.75rem + 1px);
    }
  }

  &__body {
    position: relative;
    line-height: 1.5rem;
    padding-left: 1.25rem;
    display: flex;
    cursor: pointer;

    &::before {
      content: "";
      position: absolute;
      left: 1rem;
      top: 50%;
      width: 0.5rem;
      height: 1px;
      background-color: var(--s-color-outline);
      z-index: 1;
    }
    &:hover {
      background-color: var(--s-color-secondary-container);
    }
    &.selected {
      background-color: var(--s-color-primary-container);
    }
  }

  &__content {
    flex: 1;
    display: flex;
    align-items: center;

    div {
      width: 50%;
    }

    .description {
      width: calc(50% + 24px + 1rem);
    }
  }

  &__children {
    padding-left: 1rem;
  }
}
</style>