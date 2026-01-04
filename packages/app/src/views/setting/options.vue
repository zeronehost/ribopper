<template>
  <RiboOptionSection title="操作设置" class="options-view">
    <s-alert>当前匹配模式与剪贴板内容匹配时，可以执行它弹出菜单中的命令。</s-alert>
    <s-card>
      <section class="ribo-cmd">
        <header class="ribo-cmd__row ribo-cmd__header">
          <div class="ribo-cmd__col pattern">匹配模式和指令</div>
          <div class="ribo-cmd__col description">描述</div>
        </header>
        <s-scroll-view class="ribo-cmd__body">
          <RiboOption :options="options" @selected="selectedHandle" />
        </s-scroll-view>
      </section>
      <footer>
        <div>
          <s-button type="elevated" @click="addHandle">
            <s-icon slot="start" name="add"></s-icon>
            <span>添加操作</span>
          </s-button>
          <s-button type="elevated" :disabled="!selected" @click="editHandle">
            <s-icon slot="start">
              <RiboIconEdit />
            </s-icon>
            <span>编辑操作</span>
          </s-button>
        </div>
        <s-button type="elevated" :disabled="!selected" @click="deleteDialogShow = true">
          <s-icon slot="start">
            <RiboIconDelete />
          </s-icon>
          <span>删除操作</span>
        </s-button>
      </footer>
    </s-card>
  </RiboOptionSection>

  <RiboDialogDelete v-model="deleteDialogShow" @confirm="deleteConfirmHandle" />
  <RiboDialogAction v-model="actionShow" @confirm="actionConfirmHandle" />
</template>
<script setup lang="ts">
import { RiboIconEdit, RiboIconDelete } from "@/components/icons";
import { RiboOptionSection } from "@/components/section";
import { ref } from 'vue';
import { RiboOption } from "@/components/option";
import { RiboDialogDelete, RiboDialogAction } from "@/components/dialog";
import { createActionOption, deleteAction, logger, updateAction, type Action, type NewAction, type UpdateAction } from "@ribo/api";


const options = ref<Action[]>([]);
const selected = ref<Action>();

const selectedHandle = (data: Action) => {
  selected.value = data;
}

const actionShow = ref(false);
const addHandle = () => {
  selected.value = undefined;
  actionShow.value = true;
}
const editHandle = () => {
  actionShow.value = true;
}
const actionConfirmHandle = async (action: NewAction | UpdateAction) => {
  actionShow.value = false;
  if (Object.hasOwn(action, "id")) {
    await updateAction(action as UpdateAction).catch(e => {
      logger.error(e);
    });
  } {
    await createActionOption(action as NewAction).catch(e => {
      logger.error(e);
    });
  }
}

const deleteDialogShow = ref(false);

const deleteConfirmHandle = async () => {
  deleteDialogShow.value = false;
  if (selected.value) {
    await deleteAction(selected.value?.id as number).catch(e => {
      logger.error(e);
    });
  }
}

</script>
<style lang="scss">
.options-view {
  &.ribo-option-section {
    >s-scroll-view {
      display: flex;
      flex-direction: column;
      gap: 0.5rem;
      overflow: hidden;
    }

    s-alert {
      width: 100%;
    }

    s-card {
      flex: 1;
      margin: 0 0.5rem 0.5rem;
      max-width: unset;
      display: flex;
      flex-direction: column;
      overflow: hidden;
      height: 100%;
    }

    .ribo-cmd {
      flex: 1;
      background-color: var(--s-color-surface-container-lowest);
      overflow: hidden;
      display: flex;
      flex-direction: column;

      &__row {
        display: flex;
      }

      &__col {
        flex: 1;
        padding: 0 0.5rem;
      }

      &__header {
        border-bottom: 1px solid var(--s-color-outline);
        line-height: 2rem;

        .pattern {
          border-right: 1px solid var(--s-color-outline);
        }
      }

      &__body {
        overflow: auto;
      }
    }

    footer {
      display: flex;
      padding: 0.5rem 1rem;
      align-items: center;
      background-color: var(--s-color-surface-container-high);

      s-button {
        border-radius: 6px;
      }

      div {
        flex: 1;

        s-button+s-button {
          margin-left: 0.5rem;
        }
      }
    }
  }
}
</style>