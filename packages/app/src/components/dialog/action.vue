<template>
  <dialog class="ribo-dialog-action" ref="rootEl" @close="closeDialogHandle">
    <section class="ribo-dialog-action__container">
      <header class="ribo-dialog-action__header">
        <div class="ribo-dialog-action__title">操作配置</div>
        <s-icon name="close" class="close" @click="closeHandle"></s-icon>
      </header>
      <RiboField class="ribo-dialog-action__body">
        <RiboFieldItem class="action" title="操作名称" tip="将菜单中显示">
          <s-text-field v-model.lazy="innerAction.name"></s-text-field>
        </RiboFieldItem>
        <RiboFieldItem class="action" title="匹配模式" tip="匹配模式是一条正则表达式">
          <s-text-field v-model.lazy="innerAction.pattern"></s-text-field>
        </RiboFieldItem>
        <RiboFieldItem class="action" title="描述">
          <s-text-field v-model.lazy="innerAction.description"></s-text-field>
        </RiboFieldItem>
        <s-table>
          <s-thead>
            <s-tr>
              <s-th>名称</s-th>
              <s-th class="command">指令</s-th>
              <s-th>输出</s-th>
              <s-th>描述</s-th>
              <s-th class="option">操作</s-th>
            </s-tr>
          </s-thead>
          <s-tbody>
            <s-tr v-for="(option, index) in innerAction.options">
              <s-td>{{ option.name }}</s-td>
              <s-td class="command">{{ option.command }}</s-td>
              <s-td>
                <span v-if="option.out === 'append'">添加到剪贴板</span>
                <span v-if="option.out === 'replace'">替换当前剪贴板内容</span>
                <span v-if="option.out === 'ingore'">忽略</span>
              </s-td>
              <s-td>{{ option.description }}</s-td>
              <s-td class="option">
                <s-icon-button @click="editOption(option, index)">
                  <s-icon>
                    <RiboIconEdit />
                  </s-icon>
                </s-icon-button>
                <s-icon-button class="delete" @click="deleteOption(option.id, index)">
                  <s-icon>
                    <RiboIconDelete />
                  </s-icon>
                </s-icon-button>
              </s-td>
            </s-tr>
          </s-tbody>
        </s-table>
        <RiboFieldItem class="add">
          <s-button @click="addOption">
            <s-icon slot="start" name="add"></s-icon>
            添加指令
          </s-button>
        </RiboFieldItem>
      </RiboField>
      <footer class="ribo-dialog-action__actions">
        <s-button type="filled" @click="confirmHandle">确定</s-button>
        <s-button type="elevated" @click="cancelHandle">取消</s-button>
      </footer>
    </section>
    <RiboDialogDelete v-model="optionDeleteShow" @confirm="deleteOptionConfirm" />
    <RiboDialogOption v-model="addOptionShow" @confirm="addOptionConfirm" />
    <RiboDialogOption v-model="updateOptionShow" :data="isNew ? selectedOption : editOptionData" @confirm="updateOptionConfirm" />
  </dialog>
</template>
<script setup lang="ts">
import { RiboField, RiboFieldItem } from '@/components/field';
import { RiboIconDelete, RiboIconEdit } from '@/components/icons';
import type { Action, NewAction, NewOption, Option, UpdateAction, UpdateOption } from '@ribo/api';
import { computed, ref, watch, type PropType } from 'vue';
import RiboDialogOption from "./option.vue";
import RiboDialogDelete from './delete.vue';

defineOptions({
  name: "RiboDialogAction"
});

const props = defineProps({
  modelValue: {
    type: Boolean,
    default: false
  },
  action: {
    type: Object as PropType<Action>,
  }
});

const rootEl = ref<HTMLDialogElement>();
const innerAction = ref<Partial<Action>>(props.action ?? {
  options: [],
});
const isNew = computed(() => !Object.hasOwn(innerAction.value, "id"));

watch(() => props.modelValue, (val) => {
  if (val) {
    rootEl.value?.showModal();
    innerAction.value = props.action ?? {
      options: [],
    };
  }
});

const emit = defineEmits<{
  "update:modelValue": [val: boolean],
  deleteOption: [id: number],
  addOption: [option: NewOption],
  updateOption: [option: UpdateOption],
  confirm: [data: UpdateAction | NewAction],
  cancel: [],
  close: [],
}>();

const closeDialogHandle = () => {
  emit("update:modelValue", false);
}

const closeHandle = () => {
  rootEl.value?.close();
  emit("close");
}

const confirmHandle = () => {
  emit("confirm", isNew.value ? innerAction.value as NewAction : {
    id: props.action?.id,
    pattern: innerAction.value.pattern,
    description: innerAction.value.description,
  } as UpdateAction);
  closeHandle();
}
const cancelHandle = () => {
  emit("cancel");
  closeHandle();
}

//--------------------------------------
const addOptionShow = ref(false);
const updateOptionShow = ref(false);
const optionDeleteShow = ref(false);
const deletedId = ref<number>();
const deletedIndex = ref<number>();
const editIndex = ref<number>();
const editOptionData = ref<Option>();
const selectedOption = computed(() =>
  editIndex.value !== undefined ?
    innerAction.value.options?.[editIndex.value] :
    undefined
);
const addOption = () => {
  addOptionShow.value = true;
}
const editOption = (option: Option, index: number) => {
  updateOptionShow.value = true;
  editOptionData.value = option;
  editIndex.value = index;
}
const deleteOption = (optionId: number, index: number) => {
  optionDeleteShow.value = true;
  deletedId.value = optionId;
  deletedIndex.value = index;
}
const deleteOptionConfirm = () => {
  if (isNew.value) {
    if (deletedIndex.value!==undefined) {
      innerAction.value.options?.splice(deletedIndex.value, 1);
    }
  } else {
    if (deletedId.value !== undefined) {
      emit("deleteOption", deletedId.value);
    }
  }
  deletedIndex.value = undefined;
  deletedId.value = undefined;
}

const addOptionConfirm = (option: Option) => {
  if (isNew.value) {
    if (innerAction.value.options) {
      option.actionId = 0;
      innerAction.value.options.push(option);
    } else {
      innerAction.value.options = [option];
    }
  } else {
    emit("addOption", {
      command: option.command,
      description: option.description,
      out: option.out,
      actionId: props.action?.id,
    } as UpdateOption);

  }
}
const updateOptionConfirm = (option: Option) => {
  if (isNew.value) {
    if (editIndex.value !== undefined) {
      (innerAction.value.options as Option[])[editIndex.value] = option;
    }
    editIndex.value = undefined;
  } else {
    emit("updateOption", {
      id: option.id,
      command: option.command,
      description: option.description,
      out: option.out,
      actionId: props.action?.id,
    } as UpdateOption);
  }
}
//--------------------------------------
defineExpose({
  //刷新
  refresh(options: Option[]){
    innerAction.value.options = options;
  }
})
</script>
<style lang="scss">
.ribo-dialog-action {
  border: none;
  padding: 1rem;
  border-radius: 5px;
  width: calc(100% - 2rem);
  height: calc(100% - 2rem);
  background-color: var(--s-color-surface-container);
  overflow: hidden;

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

    .action {
      .ribo-field-item__content {
        width: 100%;
      }

      s-text-field {
        width: 100%;
      }
    }

    s-table {
      margin-top: 1rem;
      width: 100%;

      s-thead {
        z-index: 1;
      }

      s-td {
        overflow: hidden;
        /* 隐藏超出部分 */
        white-space: nowrap;
        /* 禁止换行 */
        text-overflow: ellipsis;
        /* 显示省略号 */
      }

      .option {
        width: 3rem;
      }

      .command {
        width: 15%;
      }

      .delete {
        s-icon {
          color: var(--s-color-error);
        }
      }
    }

    .add {
      margin-top: 1rem;

      s-button {
        border-radius: 5px;
      }
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