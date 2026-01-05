<template>
  <dialog class="ribo-dialog-action" ref="rootEl" @close="closeDialogHandle">
    <section class="ribo-dialog-action__container">
      <header class="ribo-dialog-action__header">
        <div class="ribo-dialog-action__title">操作配置</div>
        <s-icon name="close" class="close" @click="closeHandle"></s-icon>
      </header>
      <RiboField class="ribo-dialog-action__body">
        <RiboFieldItem class="action" title="匹配模式" tip="匹配模式是一条正则表达式">
          <s-text-field v-model.lazy="innerAction.pattern"></s-text-field>
        </RiboFieldItem>
        <RiboFieldItem class="action" title="描述">
          <s-text-field v-model.lazy="innerAction.description"></s-text-field>
        </RiboFieldItem>
        <s-table>
          <s-thead>
            <s-tr>
              <s-th class="command">指令</s-th>
              <s-th>输出</s-th>
              <s-th>描述</s-th>
              <s-th class="option">操作</s-th>
            </s-tr>
          </s-thead>
          <s-tbody>
            <s-tr v-for="option in innerAction.options">
              <s-td class="command">{{ option.command }}</s-td>
              <s-td></s-td>
              <s-td>{{ option.description }}</s-td>
              <s-td class="option">
                <s-icon-button @click="editOption(option)">
                  <s-icon>
                    <RiboIconEdit />
                  </s-icon>
                </s-icon-button>
                <s-icon-button class="delete" @click="deleteOption(option.id)">
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
    <RiboDialogOption v-model="updateOptionShow" :data="editOptionData" @confirm="updateOptionConfirm" />
  </dialog>
</template>
<script setup lang="ts">
import { RiboField, RiboFieldItem } from '@/components/field';
import { RiboIconDelete, RiboIconEdit } from '@/components/icons';
import type { Action, NewOption, Option, UpdateAction, UpdateOption } from '@ribo/api';
import { ref, watch, type PropType } from 'vue';
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
  confirm: [data: UpdateAction],
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
  closeHandle();
  emit("confirm", {
    id: props.action?.id,
    pattern: innerAction.value.pattern,
    description: innerAction.value.description,
  } as UpdateAction);
}
const cancelHandle = () => {
  closeHandle();
  emit("cancel");
}

//--------------------------------------
const addOptionShow = ref(false);
const updateOptionShow = ref(false);
const optionDeleteShow = ref(false);
const deleted = ref<number>();
const editOptionData = ref<Option>();
const addOption = () => {
  addOptionShow.value = true;
}
const editOption = (option: Option) => {
  updateOptionShow.value = true;
  editOptionData.value = option;
}
const deleteOption = (optionId: number) => {
  optionDeleteShow.value = true;
  deleted.value = optionId;
}
const deleteOptionConfirm = () => {
  if (deleted.value !== undefined) {
    emit("deleteOption", deleted.value);
  }
  deleted.value = undefined;
}

const addOptionConfirm = (option: Option) => {
  emit("addOption", {
    command: option.command,
    description: option.description,
    out: option.out,
    actionId: props.action?.id,
  } as UpdateOption);
}
const updateOptionConfirm = (option: Option) => {
  emit("updateOption", {
    id: option.id,
    command: option.command,
    description: option.description,
    out: option.out,
    actionId: props.action?.id,
  } as UpdateOption);
}
//--------------------------------------

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
    grid-template-rows: 2rem auto 3rem;
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
    margin: 1rem 0;
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