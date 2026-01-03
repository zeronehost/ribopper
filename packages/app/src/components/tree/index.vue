<template>
  <div class="ribo-tree">
    <ul class="ribo-tree__list">
      <tree-node
        v-for="node in data"
        :key="node.id"
        :node="node"
        :level="0"
        @node-click="handleNodeClick"
        @node-expand="handleNodeExpand"
        @node-check="handleNodeCheck"
      />
    </ul>
  </div>
</template>

<script lang="ts" setup>
import { ref, type PropType } from "vue";
import TreeNode from "./tree-node.vue";

defineOptions({
  name: "RiboTree",
});

export interface TreeNodeData {
  id: string | number;
  label: string;
  children?: TreeNodeData[];
  expanded?: boolean;
  checked?: boolean;
  disabled?: boolean;
  [key: string]: any;
}

const props = defineProps({
  data: {
    type: Array as PropType<TreeNodeData[]>,
    default: () => [],
  },
  defaultExpandAll: {
    type: Boolean,
    default: false,
  },
  showCheckbox: {
    type: Boolean,
    default: false,
  },
  accordion: {
    type: Boolean,
    default: false,
  },
  indent: {
    type: Number,
    default: 16,
  },
});

const emit = defineEmits<{
  (e: "node-click", data: TreeNodeData, node: any, event: Event): void;
  (e: "node-expand", data: TreeNodeData, node: any, expanded: boolean): void;
  (e: "node-check", data: TreeNodeData, checked: boolean): void;
}>();

const handleNodeClick = (data: TreeNodeData, node: any, event: Event) => {
  emit("node-click", data, node, event);
};

const handleNodeExpand = (data: TreeNodeData, node: any, expanded: boolean) => {
  emit("node-expand", data, node, expanded);
};

const handleNodeCheck = (data: TreeNodeData, checked: boolean) => {
  emit("node-check", data, checked);
};
</script>

<style lang="scss">
.ribo-tree {
  &__list {
    list-style: none;
    padding: 0;
    margin: 0;
  }
}
</style>
