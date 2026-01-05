<template>
  <dialog class="ribo-dialog-add-action" ref="rootEl" @close="closeDialogHandle">
    <section class="ribo-dialog-add-action__container">
      <header class="ribo-dialog-add-action__header">
        <div class="ribo-dialog-add-action__title">操作配置</div>
        <s-icon name="close" class="close" @click="closeHandle"></s-icon>
      </header>
      <RiboField class="ribo-dialog-add-action__body">
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
            <s-tr v-for="(option, i) in innerAction.options">
              <s-td class="command">{{ option.command }}</s-td>
              <s-td></s-td>
              <s-td>{{ option.description }}</s-td>
              <s-td class="option">
                <s-icon-button @click="editOption(i)">
                  <s-icon>
                    <RiboIconEdit />
                  </s-icon>
                </s-icon-button>
                <s-icon-button class="delete" @click="deleteOption(i)">
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
      <footer class="ribo-dialog-add-action__actions">
        <s-button type="filled" @click="confirmHandle">确定</s-button>
        <s-button type="elevated" @click="cancelHandle">取消</s-button>
      </footer>
    </section>
    <RiboDialogDelete v-model="optionDeleteShow" @confirm="deleteOptionConfirm" />
    <RiboDialogOption v-model="addOptionShow" @confirm="addOptionConfirm" />
    <RiboDialogOption v-model="updateOptionShow" :data="selectedOption" @confirm="updateOptionConfirm" />
  </dialog>
</template>
<script setup lang="ts">
import { RiboField, RiboFieldItem } from '@/components/field';
import { RiboIconDelete, RiboIconEdit } from '@/components/icons';
import type { Action, Option } from '@ribo/api';
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
  data: {
    type: Object as PropType<Action>,
  }
});

const rootEl = ref<HTMLDialogElement>();
const innerAction = ref<Partial<Action>>(props.data ?? {
  options: [],
});

watch(() => props.modelValue, (val) => {
  if (val) {
    rootEl.value?.showModal();
    innerAction.value = props.data ?? {
      options: [],
    };
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
}

const confirmHandle = () => {
  closeHandle();
  emit("confirm", innerAction.value);
}
const cancelHandle = () => {
  closeHandle();
  emit("cancel");
}

//--------------------------------------
const addOptionShow = ref(false);
const updateOptionShow = ref(false);
const optionDeleteShow = ref(false);
const deletedIndex = ref<number>();
const editIndex = ref<number>();
const selectedOption = computed(() =>
  editIndex.value !== undefined ?
    innerAction.value.options?.[editIndex.value] :
    undefined
);
const addOption = () => {
  addOptionShow.value = true;
}
const editOption = (index: number) => {
  updateOptionShow.value = true;
  editIndex.value = index;
}
const deleteOption = (index: number) => {
  optionDeleteShow.value = true;
  deletedIndex.value = index;
}
const deleteOptionConfirm = () => {
  console.log(innerAction.value, deletedIndex.value);
  if (deletedIndex.value!==undefined) {
    innerAction.value.options?.splice(deletedIndex.value, 1);
  }
  deletedIndex.value = undefined;
}

const addOptionConfirm = (option: Option) => {
  if (innerAction.value.options) {
    option.actionId = 0;
    innerAction.value.options.push(option);
  } else {
    innerAction.value.options = [option];
  }
}
const updateOptionConfirm = (option: Option) => {
  if (editIndex.value !== undefined) {
    (innerAction.value.options as Option[])[editIndex.value] = option;
  }
  editIndex.value = undefined;
}
//--------------------------------------

</script>
<style lang="scss">
.ribo-dialog-add-action {
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