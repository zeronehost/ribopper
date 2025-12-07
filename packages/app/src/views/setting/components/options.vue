
<template>
  <RiboOptionSection title="操作设置">
    <RiboField title="文本">
      <RiboFieldItem title="可执行"><s-switch></s-switch></RiboFieldItem>
      <RiboFieldItem title="可生成二维码"><s-switch></s-switch></RiboFieldItem>
      <RiboFieldItem title="可编辑"><s-switch></s-switch></RiboFieldItem>
      <RiboFieldItem title="可修改"><s-switch></s-switch></RiboFieldItem>
    </RiboField>
  </RiboOptionSection>  
</template>
<script setup lang="ts">
import { RiboOptionSection } from "@/components/section";
import { RiboField, RiboFieldItem } from "@/components/field";
</script>
