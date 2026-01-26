<template>
  <RiboOptionSection title="快捷键设置" class="hot-key">
    <s-table>
      <s-thead>
        <s-tr>
          <s-th class="index"></s-th>
          <s-th class="option">操作</s-th>
          <s-th class="key">快捷键</s-th>
          <s-th class="index"></s-th>
        </s-tr>
      </s-thead>
      <s-tbody>
        <RiboKey @edit="editHandle" :selected="currentEdit === 'pane'" desc="显示剪贴板" label="pane" :data="settingStore.config.hotkey?.pane">
        </RiboKey>
        <RiboKey @edit="editHandle" :selected="currentEdit === 'clear'" desc="清除历史记录" label="clear" :data="settingStore.config.hotkey?.clear"></RiboKey>
        <RiboKey v-if="editEnabled" @edit="editHandle" @delete="deleteHandle" :selected="currentEdit === 'edit'" desc="编辑内容" label="edit" :data="settingStore.config.hotkey?.edit"></RiboKey>
        <RiboKey v-if="prevEnabled" @edit="editHandle" @delete="deleteHandle" :selected="currentEdit === 'prev'" desc="上一条记录" label="prev" :data="settingStore.config.hotkey?.prev"></RiboKey>
        <RiboKey v-if="nextEnabled" @edit="editHandle" @delete="deleteHandle" :selected="currentEdit === 'next'" desc="下一条记录" label="next" :data="settingStore.config.hotkey?.next"></RiboKey>
        <RiboKey v-if="qrcodeEnabled" @edit="editHandle" @delete="deleteHandle" :selected="currentEdit === 'qrcode'" desc="显示二维码" label="qrcode" :data="settingStore.config.hotkey?.qrcode"></RiboKey>
        <RiboKey v-if="deleteEnabled" @edit="editHandle" @delete="deleteHandle" :selected="currentEdit === 'delete'" desc="删除" label="delete" :data="settingStore.config.hotkey?.delete"></RiboKey>
        <RiboKey v-if="copyEnabled" @edit="editHandle" @delete="deleteHandle" :selected="currentEdit === 'copy'" desc="复制" label="copy" :data="settingStore.config.hotkey?.copy"></RiboKey>
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

const editEnabled = import.meta.env.VITE_APP_ENABLE_HOTKEY === 'true' || import.meta.env.VITE_APP_ENABLE_EDIT_HOTKEY === 'true';
const prevEnabled = import.meta.env.VITE_APP_ENABLE_HOTKEY === 'true' || import.meta.env.VITE_APP_ENABLE_PREV_HOTKEY === 'true';
const nextEnabled = import.meta.env.VITE_APP_ENABLE_HOTKEY === 'true' || import.meta.env.VITE_APP_ENABLE_NEXT_HOTKEY === 'true';
const qrcodeEnabled = import.meta.env.VITE_APP_ENABLE_HOTKEY === 'true' || import.meta.env.VITE_APP_ENABLE_QRCODE_HOTKEY === 'true';
const deleteEnabled = import.meta.env.VITE_APP_ENABLE_HOTKEY === 'true' || import.meta.env.VITE_APP_ENABLE_DELETE_HOTKEY === 'true';
const copyEnabled = import.meta.env.VITE_APP_ENABLE_HOTKEY === 'true' || import.meta.env.VITE_APP_ENABLE_COPY_HOTKEY === 'true';

const settingStore = useSettingStore();

const showKeyF = ref(false);
const currentEdit = ref<keyof Hotkey>();
const editHandle = (label: keyof Hotkey) => {
  showKeyF.value = true;
  currentEdit.value = label;
}
const deleteHandle = (label: keyof Hotkey) => {
  settingStore.deleteHotkey(label);
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