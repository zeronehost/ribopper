
<template>
  <RiboSection title="快捷键设置" class="hot-key">
    <s-table>
      <s-thead>
        <s-tr>
          <s-th>操作</s-th>
          <s-th>快捷键</s-th>
        </s-tr>
      </s-thead>
      <s-tbody>
        <s-tr v-for="i in 30" :key="i">
          <s-td>{{ i }}</s-td>
          <s-td></s-td>
        </s-tr>
      </s-tbody>
    </s-table>
  </RiboSection>  
</template>
<script setup lang="ts">
import { RiboSection } from "@/components/section";
</script>
<style lang="scss">
.hot-key {
  s-table {
    margin: .5rem;
    overflow: auto;

    .index {
      width: 3rem;
    }
  }
}
</style>