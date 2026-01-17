<template>
  <s-dialog class="ribo-dialog-delete" :showed>
    <div slot="headline" class="headline">{{title}}</div>
    <div slot="text">{{content}}</div>
    <div slot="action" class="action">
      <s-button type="filled" @click="confirmHandle">确定</s-button>
      <s-button type="elevated" @click="cancelHandle">取消</s-button>
    </div>
  </s-dialog>
</template>
<script setup lang="ts">
import { watch, ref } from 'vue';

defineOptions({
  name: "RiboDialogDelete",
});

const props = defineProps({
  modelValue: {
    type: Boolean,
    default: false,
  },
  title: {
    type: String,
    default: "温馨提示",
  },
  content: {
    type: String,
    default: "确定要删除该操作吗？",
  }
});

const showed = ref(props.modelValue);
watch(() => props.modelValue, (val) => {
  showed.value = val;
});

const emit = defineEmits(["update:modelValue", "confirm", "cancel"]);

const confirmHandle = () => {
  emit("confirm");
  emit("update:modelValue", false);
};

const cancelHandle = () => {
  emit("cancel");
  emit("update:modelValue", false);
}
</script>
<style lang="scss">
.ribo-dialog-delete {
  &::part(container) {
    border-radius: 0.5rem;
  }

  .action {
    padding: 0;
  }

  s-button {
    border-radius: 5px;

    &+s-button {
      margin-left: 1rem;
    }
  }
}
</style>