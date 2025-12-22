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
        <template v-for="(item, i) in hotkeys" :key="i">
          <s-tr v-if="item.enabled">
            <s-td class="index" @click="editHandle(i)">
              <s-icon>
                <svg viewBox="0 -960 960 960">
                  <path
                    d="M200-200h57l391-391-57-57-391 391v57Zm-80 80v-170l528-527q12-11 26.5-17t30.5-6q16 0 31 6t26 18l55 56q12 11 17.5 26t5.5 30q0 16-5.5 30.5T817-647L290-120H120Zm640-584-56-56 56 56Zm-141 85-28-29 57 57-29-28Z">
                  </path>
                </svg>
              </s-icon>
            </s-td>
            <s-td class="option">{{ item.desc }}</s-td>
            <s-td class="key">
              <kbd v-for="key in item.keys" :key="key">{{ key }}</kbd>
            </s-td>
          </s-tr>
        </template>
      </s-tbody>
    </s-table>
    <dialog ref="dialogEl">
      <header>添加组合键</header>
      <s-text-field></s-text-field>
      <footer>
        <s-button type="elevated">取消</s-button>
        <s-button>确定</s-button>
      </footer>
    </dialog>
  </RiboOptionSection>
</template>
<script setup lang="ts">
import { RiboOptionSection } from "@/components/section";
import { ref } from "vue";

const hotkeys = ref([
  {
    desc: "编辑内容",
    id: "edit",
    keys: ["ctrl", "e"],
    enabled: true,
  },
  {
    desc: "清除历史记录",
    id: "clear",
    keys: [],
    enabled: true,
  },
  {
    desc: "上一条记录",
    id: "prev",
    keys: [],
    enabled: true,
  },
  {
    desc: "下一条记录",
    id: "next",
    keys: [],
    enabled: true,
  },
  {
    desc: "显示二维码",
    id: "qrcode",
    keys: [],
    enabled: true,
  },
  {
    desc: "显示剪贴板", // 当前鼠标位置
    id: "clipboard",
    keys: [],
    enabled: true,
  },
]);

const dialogEl = ref<HTMLDialogElement>();
const editHandle = (index: number) => {
  dialogEl.value?.showModal()
}
const blurHandle = () => {
  dialogEl.value?.close()
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
  dialog {
    border: none;
    padding: 1rem;
    border-radius: 5px;
    background-color: var(--s-color-surface-container-lowest);
    text-align: center;

    header {
      margin-bottom: 1rem;
      font-size: .8em;
    }
    footer {
      margin-top: 1rem;
      s-button {
        border-radius: 5px;
        &:not(:first-child) {
          margin-left: 1rem;
        }
      }
    }
  }
}
</style>