<template>
  <li class="tree-node" :class="{ 'is-leaf': !hasChildren, 'is-last-child': isLastChild }">
    <div class="tree-node__wrapper">
      <!-- 父节点到当前节点的垂直线 -->
      <div
        v-if="level > 0"
        class="tree-node__vertical-line"
        :class="{ 'tree-node__vertical-line--last': isLastChild }"
        :style="{ left: `${(level - 1) * indent + 12}px` }"
      ></div>

      <!-- 当前节点的水平线 -->
      <div
        v-if="level > 0"
        class="tree-node__horizontal-line"
        :style="{ left: `${(level - 1) * indent + 12}px`, width: `${indent - 12}px` }"
      ></div>

      <div
        class="tree-node__content"
        :style="{ paddingLeft: `${level * indent}px` }"
        @click="handleClick"
      >
        <span
          class="tree-node__expand-icon"
          :class="{ 'is-expanded': expanded, 'is-disabled': disabled }"
          @click.stop="handleExpand"
        >
          <s-icon v-if="hasChildren">
            <svg viewBox="0 0 1024 1024" width="12" height="12">
              <path d="M765.312 499.2l-438.4-438.4c-12.8-12.8-32-12.8-44.8 0s-12.8 32 0 44.8l416 416-416 416c-12.8 12.8-12.8 32 0 44.8s32 12.8 44.8 0l438.4-438.4c6.4-6.4 9.6-14.72 9.6-22.4s-3.2-16-9.6-22.4z" fill="currentColor" transform="rotate(90 512 512)"></path>
            </svg>
          </s-icon>
        </span>

        <s-checkbox
          v-if="showCheckbox"
          class="tree-node__checkbox"
          :model-value="checked"
          :disabled="disabled"
          @change="handleCheck"
          @click.stop
        />

        <span
          class="tree-node__label"
          :class="{ 'is-selected': selected, 'is-disabled': disabled }"
        >
          {{ node.label }}
        </span>
      </div>
    </div>

    <transition name="tree-node-fade">
      <ul
        v-if="hasChildren && expanded"
        class="tree-node__children"
        :class="{ 'is-hidden': !expanded }"
      >
        <tree-node
          v-for="(child, index) in node.children"
          :key="child.id"
          :node="child"
          :level="level + 1"
          :indent="indent"
          :show-checkbox="showCheckbox"
          :is-last-child="index === node.children.length - 1"
          @node-click="handleChildClick"
          @node-expand="handleChildExpand"
          @node-check="handleChildCheck"
        />
      </ul>
    </transition>
  </li>
</template>

<script lang="ts" setup>
import { computed, ref, type PropType } from "vue";
import type { TreeNodeData } from "./index.vue";

defineOptions({
  name: "TreeNode",
});

const props = defineProps({
  node: {
    type: Object as PropType<TreeNodeData>,
    required: true,
  },
  level: {
    type: Number,
    default: 0,
  },
  indent: {
    type: Number,
    default: 16,
  },
  showCheckbox: {
    type: Boolean,
    default: false,
  },
  isLastChild: {
    type: Boolean,
    default: false,
  },
});

const emit = defineEmits<{
  (e: "node-click", data: TreeNodeData, node: any, event: Event): void;
  (e: "node-expand", data: TreeNodeData, node: any, expanded: boolean): void;
  (e: "node-check", data: TreeNodeData, checked: boolean): void;
}>();

const expanded = ref(props.node.expanded || false);
const checked = ref(props.node.checked || false);
const selected = ref(false);
const disabled = ref(props.node.disabled || false);

const hasChildren = computed(() => {
  return props.node.children && props.node.children.length > 0;
});

const handleClick = (event: Event) => {
  if (disabled.value) return;
  emit("node-click", props.node, { expanded, checked, selected }, event);
};

const handleExpand = () => {
  if (disabled.value || !hasChildren.value) return;
  expanded.value = !expanded.value;
  emit("node-expand", props.node, { expanded, checked, selected }, expanded.value);
};

const handleCheck = (val: boolean) => {
  if (disabled.value) return;
  checked.value = val;
  emit("node-check", props.node, val);
};

const handleChildClick = (data: TreeNodeData, node: any, event: Event) => {
  emit("node-click", data, node, event);
};

const handleChildExpand = (data: TreeNodeData, node: any, expanded: boolean) => {
  emit("node-expand", data, node, expanded);
};

const handleChildCheck = (data: TreeNodeData, checked: boolean) => {
  emit("node-check", data, checked);
};
</script>

<style lang="scss">
.tree-node {
  list-style: none;
  margin: 0;
  padding: 0;
  white-space: nowrap;
  outline: none;
  position: relative;

  &__wrapper {
    position: relative;
  }

  &__vertical-line {
    position: absolute;
    top: -13px; /* 从父节点中间开始 */
    bottom: 0;
    width: 1px;
    background-color: var(--s-color-outline);
    z-index: 0;
  }

  &__vertical-line--last {
    /* 最后一个子节点的垂直线只到节点中间位置 */
    bottom: 13px !important; /* 只到节点中间位置 */
  }

  &__horizontal-line {
    position: absolute;
    top: 13px; /* 节点中间位置 */
    height: 1px;
    background-color: var(--s-color-outline);
    z-index: 0;
  }

  &__content {
    display: flex;
    align-items: center;
    height: 26px;
    cursor: pointer;
    position: relative;
    z-index: 1;

    &:hover {
      background-color: var(--s-color-surface-hover);
    }
  }

  &__expand-icon {
    width: 24px;
    height: 24px;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    color: var(--s-color-on-surface-variant);
    transition: transform 0.2s ease-in-out;

    &.is-expanded {
      transform: rotate(90deg);
    }

    &.is-disabled {
      cursor: not-allowed;
    }
  }

  &__checkbox {
    margin-right: 8px;
  }

  &__label {
    font-size: 14px;
    flex: 1;
    overflow: hidden;
    text-overflow: ellipsis;

    &.is-selected {
      color: var(--s-color-primary);
      font-weight: 500;
    }

    &.is-disabled {
      color: var(--s-color-on-surface-variant);
      cursor: not-allowed;
    }
  }

  &__children {
    list-style: none;
    padding: 0;
    margin: 0;
    overflow: hidden;
    position: relative;
  }

  &.is-leaf > .tree-node__content > .tree-node__expand-icon {
    visibility: hidden;
  }
}

.tree-node-fade-enter-active,
.tree-node-fade-leave-active {
  transition: opacity 0.2s ease-in-out;
}

.tree-node-fade-enter-from,
.tree-node-fade-leave-to {
  opacity: 0;
}
</style>
