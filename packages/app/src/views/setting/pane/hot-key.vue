<template>
  <RiboOptionSection title="快捷键设置" class="hot-key">
    <s-table>
      <s-thead>
        <s-tr>
          <s-th class="index"></s-th>
          <s-th class="option">操作</s-th>
          <s-th class="key">快捷键</s-th>
        </s-tr>
      </s-thead>
      <s-tbody>
        <RiboKey @edit="editHandle" desc="显示剪贴板" label="pane" :data="settingStore.config.hotkey?.pane"></RiboKey>
        <RiboKey @edit="editHandle" desc="清除历史记录" label="clear" :data="settingStore.config.hotkey?.clear"></RiboKey>
        <RiboKey @edit="editHandle" desc="编辑内容" label="edit" :data="settingStore.config.hotkey?.edit"></RiboKey>
        <RiboKey @edit="editHandle" desc="上一条记录" label="prev" :data="settingStore.config.hotkey?.prev"></RiboKey>
        <RiboKey @edit="editHandle" desc="下一条记录" label="next" :data="settingStore.config.hotkey?.next"></RiboKey>
        <RiboKey @edit="editHandle" desc="显示二维码" label="qrcode" :data="settingStore.config.hotkey?.qrcode"></RiboKey>
        <RiboKey @edit="editHandle" desc="删除" label="delete" :data="settingStore.config.hotkey?.delete"></RiboKey>
        <RiboKey @edit="editHandle" desc="复制" label="copy" :data="settingStore.config.hotkey?.copy"></RiboKey>
      </s-tbody>
    </s-table>
    <RiboKeyDialog v-model="showKeyF" @confirm="confirmHandle"></RiboKeyDialog>
  </RiboOptionSection>
</template>
<script setup lang="ts">
import { RiboOptionSection } from "@/components/section";
import { RiboKeyDialog, RiboKey } from "@/components/key";
import { ref } from "vue";
import { useSettingStore } from "@/stores/setting";
import { type RiboHotkey, type RiboKey as IRiboKey, logger } from "@ribo/api";


const settingStore = useSettingStore();

const showKeyF = ref(false);
const currentEdit = ref<keyof RiboHotkey>();
const editHandle = (label: keyof RiboHotkey) => {
  showKeyF.value = true;
  logger.debug(label);
  currentEdit.value = label;
}

const confirmHandle = (data: IRiboKey) => {
  if (currentEdit.value) {
    settingStore.setHotkey(currentEdit.value, data);
  }
}

</script>
<style lang="scss">
.hot-key {
  &.ribo-option-section {
    s-scroll-view {
      display: flex;
      flex-direction: column;
      overflow: hidden;
    }
  }

  s-table {
    margin: .5rem;
    overflow: auto;
    display: block;

    * {
      cursor: default;
    }

    .index {
      width: 3rem;
    }

    .option {
      width: 10rem;
    }

    s-td {
      &:not(:last-child) {
        border-right: solid 1px var(--s-color-outline-variant, #C0C8CC);
      }

      s-icon {
        width: 1rem;
        height: 1rem;
        visibility: hidden;
      }

      kbd {
        background-color: var(--s-color-surface-container-low);
        padding: 3px 5px;
        border-radius: 4px;

        &:not(:last-child) {
          margin-right: 5px;
        }
      }

    }

    s-tr {
      &:hover {
        s-icon {
          visibility: visible;
        }

        .key {
          cursor: text;

          * {
            cursor: text;
          }
        }
      }
    }
  }
}
</style>