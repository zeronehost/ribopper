<template>
  <dialog class="ribo-add-edit-dialog" ref="rootEl" @close="closeDialogHandle">
    <section class="ribo-add-edit-dialog__container">
      <header class="ribo-add-edit-dialog__header">
        <div class="ribo-add-edit-dialog__title">指令配置</div>
        <s-icon name="close" class="close" @click="closeHandle"></s-icon>
      </header>
      <RiboField class="ribo-add-edit-dialog__body">
        <RiboFieldItem title="匹配模式">
          <s-text-field v-model.lazy="innerAction.pattern"></s-text-field>
        </RiboFieldItem>
        <RiboFieldItem title="描述">
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
                <s-icon-button>
                  <s-icon>
                    <RiboIconEdit />
                  </s-icon>
                </s-icon-button>
                <s-icon-button class="delete">
                  <s-icon>
                    <RiboIconDelete />
                  </s-icon>
                </s-icon-button>
              </s-td>
            </s-tr>
          </s-tbody>
        </s-table>
        <RiboFieldItem class="add">
          <s-button>
            <s-icon slot="start" name="add"></s-icon>
            添加指令
          </s-button>
        </RiboFieldItem>
      </RiboField>
      <footer class="ribo-add-edit-dialog__actions">
        <s-button type="filled" @click="confirmHandle">确认</s-button>
        <s-button type="elevated" @click="cancelHandle">取消</s-button>
      </footer>
    </section>
  </dialog>
</template>
<script setup lang="ts">
import { RiboField, RiboFieldItem } from '@/components/field';
import { RiboIconDelete, RiboIconEdit } from '@/components/icons';
import type { Action } from '@ribo/api';
import { ref, watch, type PropType } from 'vue';

defineOptions({
  name: "RiboAddAndEditDialog"
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
const innerAction = ref<Partial<Action>>(props.data??{
  options: [],
});

watch(() => props.modelValue, (val) => {
  if (val) {
    rootEl.value?.showModal();
  }
});

const emit = defineEmits<{
  "update:modelValue": [val: boolean],
  confirm: [data: any],
  cancel: []
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
</script>
<style lang="scss">
.ribo-add-edit-dialog {
  border: none;
  padding: 1rem;
  border-radius: 5px;
  width: calc(100% - 2rem);
  height: calc(100% - 2rem);
  background-color: var(--s-color-surface-container);

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

  &__body {
    overflow: hidden;
    margin: 1rem 0;
    padding: 1rem;
    box-shadow: none;

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