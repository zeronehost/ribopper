<template>
  <dialog class="ribo-dialog-option" ref="rootEl" @close="closeDialogHandle">
    <section class="ribo-dialog-option__container">
      <header class="ribo-dialog-option__header">
        <div class="ribo-dialog-option__title">指令配置</div>
        <s-icon name="close" class="close" @click="closeHandle"></s-icon>
      </header>
      <RiboField class="ribo-dialog-option__body">
        <RiboFieldItem title="名称" :error="errors?.name?.errors?.[0]">
          <s-text-field v-model.lazy="innerOption.name"></s-text-field>
        </RiboFieldItem>
        <RiboFieldItem title="指令" tip="指令中 <%s> 将被替换为完整剪贴板内容" :error="errors?.command?.errors?.[0]">
          <s-text-field v-model.lazy="innerOption.command"></s-text-field>
        </RiboFieldItem>
        <RiboFieldItem title="描述" :error="errors?.description?.errors?.[0]">
          <s-text-field v-model.lazy="innerOption.description"></s-text-field>
        </RiboFieldItem>
        <RiboFieldItem title="指令输出" :error="errors?.out?.errors?.[0]">
          <s-radio-button v-model.lazy="innerOption.out" type="radio" value="ingore">忽略</s-radio-button>
          <s-radio-button v-model.lazy="innerOption.out" type="radio" value="replace">替换当前剪贴板内容</s-radio-button>
          <s-radio-button v-model.lazy="innerOption.out" type="radio" value="append">添加到剪贴板</s-radio-button>
        </RiboFieldItem>
      </RiboField>
      <footer class="ribo-dialog-option__actions">
        <s-button type="filled" @click="confirmHandle">确定</s-button>
        <s-button type="elevated" @click="cancelHandle">取消</s-button>
      </footer>
    </section>
  </dialog>
</template>
<script setup lang="ts">
import { RiboField, RiboFieldItem } from '@/components/field';
import { NewOption, Option } from '@ribo/api';
import { ref, watch, type PropType } from 'vue';
import { treeifyError } from "zod";

defineOptions({
  name: "RiboDialogOption"
});

const props = defineProps({
  modelValue: {
    type: Boolean,
    default: false
  },
  data: {
    type: Object as PropType<Option>,
    default: () => ({})
  },
});

const rootEl = ref<HTMLDialogElement>();
const innerOption = ref<Partial<Option>>(Object.assign({}, {out: "ingore"}, props.data));

watch(() => props.modelValue, (val) => {
  if (val) {
    rootEl.value?.showModal();
    innerOption.value = Object.assign({}, {out: "ingore"}, props.data);
  }
});

const emit = defineEmits<{
  "update:modelValue": [val: boolean],
  confirm: [data: any],
  cancel: [],
}>();

const closeDialogHandle = () => {
  emit("update:modelValue", false);
}

const closeHandle = () => {
  rootEl.value?.close();
  innerOption.value = {};
  errors.value = undefined;
}
const errors = ref();
const confirmHandle = () => {
  if (props.data?.id) {
    // update
    const { success, error, data } = NewOption.safeParse(innerOption.value);
    if (!success) {
      errors.value = treeifyError(error).properties;
      return;
    }
    emit("confirm", data);
  } else {
    // create
    const { success, error, data } = NewOption.safeParse(innerOption.value);
    if (!success) {
      errors.value = treeifyError(error).properties;
      return;
    }
    emit("confirm", data);
  }
  closeHandle();
}
const cancelHandle = () => {
  closeHandle();
  emit("cancel");
}
</script>
<style lang="scss">
.ribo-dialog-option {
  border: none;
  padding: 1rem;
  border-radius: 5px;
  width: calc(100% - 6rem);
  height: calc(100% - 6rem);
  background-color: var(--s-color-surface-container);

  &:focus-visible {
    outline: none;
  }

  &__container {
    width: 100%;
    height: 100%;
    display: grid;
    grid-template-columns: auto;
    grid-template-rows: 2rem auto 2.5rem;
    gap: 0.5rem;
  }

  &__header {
    display: flex;
    align-items: center;
    justify-content: space-between;

    .close {
      cursor: pointer;
    }
  }

  &__title {
    color: var(--s-color-on-surface, #191C1E);
  }

  &__body {
    overflow: hidden;
    margin: 0;
    padding: 1rem;
    box-shadow: none;

    .ribo-field-item__content {
      width: 100%;
      flex-direction: column;
      align-items: start;
    }

    s-text-field {
      width: 100%;
    }
    s-radio-button {
      display: flex;
    }
  }

  &__actions {
    display: flex;
    align-items: center;
    justify-content: end;

    s-button {
      border-radius: 5px;

      &+s-button {
        margin-left: 1rem;
      }
    }
  }
}
</style>