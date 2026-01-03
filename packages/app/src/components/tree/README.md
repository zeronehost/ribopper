# RiboTree 树形控件

基于 Vue3 + TypeScript + SCSS 实现的树形控件组件。

## 功能特性

- 支持多级嵌套数据结构
- 支持展开/折叠节点
- 支持节点选择
- 支持复选框选择
- 支持自定义缩进
- 支持节点禁用状态
- 平滑的展开/折叠动画
- 节点间连接线，清晰展示层级关系

## 使用方法

```vue
<template>
  <div>
    <ribo-tree
      :data="treeData"
      :show-checkbox="true"
      :default-expand-all="false"
      @node-click="handleNodeClick"
      @node-expand="handleNodeExpand"
      @node-check="handleNodeCheck"
    />
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { RiboTree } from '@/components/tree';

const treeData = ref([
  {
    id: '1',
    label: '节点1',
    children: [
      {
        id: '1-1',
        label: '子节点1-1',
      },
      {
        id: '1-2',
        label: '子节点1-2',
        children: [
          {
            id: '1-2-1',
            label: '子节点1-2-1',
          }
        ]
      }
    ]
  },
  {
    id: '2',
    label: '节点2',
    disabled: true
  }
]);

const handleNodeClick = (data, node, event) => {
  console.log('节点点击', data, node);
};

const handleNodeExpand = (data, node, expanded) => {
  console.log('节点展开/折叠', data, expanded);
};

const handleNodeCheck = (data, checked) => {
  console.log('节点选中/取消', data, checked);
};
</script>
```

## API

### Props

| 属性名 | 类型 | 默认值 | 说明 |
| --- | --- | --- | --- |
| data | Array\<TreeNodeData\> | [] | 树形数据 |
| defaultExpandAll | Boolean | false | 是否默认展开所有节点 |
| showCheckbox | Boolean | false | 是否显示复选框 |
| accordion | Boolean | false | 是否手风琴模式 |
| indent | Number | 16 | 相邻级节点间的水平缩进，单位px |

### TreeNodeData

```typescript
interface TreeNodeData {
  id: string | number;
  label: string;
  children?: TreeNodeData[];
  expanded?: boolean;
  checked?: boolean;
  disabled?: boolean;
  [key: string]: any;
}
```

### Events

| 事件名 | 说明 | 参数 |
| --- | --- | --- |
| node-click | 节点点击事件 | (data: TreeNodeData, node: any, event: Event) |
| node-expand | 节点展开/折叠事件 | (data: TreeNodeData, node: any, expanded: boolean) |
| node-check | 节点选中/取消事件 | (data: TreeNodeData, checked: boolean) |
