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
        <RiboKey @edit="editHandle" :selected="currentEdit === 'pane'" disabled desc="显示剪贴板" label="pane" :data="settingStore.config.hotkey?.pane">
        </RiboKey>
        <RiboKey @edit="editHandle" :selected="currentEdit === 'clear'" desc="清除历史记录" label="clear" :data="settingStore.config.hotkey?.clear"></RiboKey>
        <RiboKey @edit="editHandle" :selected="currentEdit === 'edit'" desc="编辑内容" label="edit" :data="settingStore.config.hotkey?.edit"></RiboKey>
        <RiboKey @edit="editHandle" :selected="currentEdit === 'prev'" desc="上一条记录" label="prev" :data="settingStore.config.hotkey?.prev"></RiboKey>
        <RiboKey @edit="editHandle" :selected="currentEdit === 'next'" desc="下一条记录" label="next" :data="settingStore.config.hotkey?.next"></RiboKey>
        <RiboKey @edit="editHandle" :selected="currentEdit === 'qrcode'" desc="显示二维码" label="qrcode" :data="settingStore.config.hotkey?.qrcode"></RiboKey>
        <RiboKey @edit="editHandle" :selected="currentEdit === 'delete'" desc="删除" label="delete" :data="settingStore.config.hotkey?.delete"></RiboKey>
        <RiboKey @edit="editHandle" :selected="currentEdit === 'copy'" desc="复制" label="copy" :data="settingStore.config.hotkey?.copy"></RiboKey>
      </s-tbody>
    </s-table>
    <RiboKeyDialog v-model="showKeyF" @confirm="confirmHandle" @close="closeHandle"></RiboKeyDialog>
  </RiboOptionSection>
</template>
<script setup lang="ts">
import { RiboOptionSection } from "@/components/section";
import { RiboKeyDialog, RiboKey } from "@/components/key";
import { ref } from "vue";
import { useSettingStore } from "@/stores/setting";
import { Hotkey, Key } from "@ribo/api";


const settingStore = useSettingStore();

const showKeyF = ref(false);
const currentEdit = ref<keyof Hotkey>();
const editHandle = (label: keyof Hotkey) => {
  showKeyF.value = true;
  currentEdit.value = label;
}

const confirmHandle = (data: Key) => {
  if (currentEdit.value) {
    settingStore.setHotkey(currentEdit.value, data);
  }
}

const closeHandle = () => {
  currentEdit.value = undefined;
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

    s-thead {
      .index {
        width: 3rem;
      }

      .option {
        width: 10rem;
      }
    }
  }
}
</style>