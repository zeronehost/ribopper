<template>
  <s-card class="ribo-card" @click="copyHandle" clickable>
    <div class="content" @click.prevent.stop>
      <s-text-field v-if="isEdit" type="multiline" v-model="newContent" @blur="updateHandle"></s-text-field>
      <pre v-else>{{ data.content }}</pre>
    </div>
    <!-- 执行 -->
    <s-icon-button v-if="executable" class="btn" slot="action" @click="playHandle">
      <RiboIconPlay />
    </s-icon-button>
    <!-- 二维码 -->
    <s-icon-button v-if="scannable" class="btn" slot="action" @click="qrcodeHandle">
      <RiboIconQrcode />
    </s-icon-button>
    <!-- 编辑 -->
    <s-icon-button v-if="editable" class="btn" slot="action" @click="editHandle">
      <RiboIconEdit />
    </s-icon-button>
    <!-- 删除 -->
    <s-icon-button v-if="deletable" class="btn delete" slot="action" @click="deleteHandle">
      <RiboIconDelete />
    </s-icon-button>
    <!-- 收藏 -->
    <s-icon-button v-if="starable" class="btn" slot="action" @click="starHandle">
      <RiboIconStarActived v-if="isStar" />
      <RiboIconStar v-else />
    </s-icon-button>
  </s-card>
</template>
<script lang="ts" setup>
import type { History } from "@ribo/api";
import { type PropType, ref } from "vue";
import {
  RiboIconDelete,
  RiboIconEdit,
  RiboIconPlay,
  RiboIconQrcode,
  RiboIconStar,
  RiboIconStarActived,
} from "@/components/icons";

defineOptions({
  name: "RiboCard",
});
const props = defineProps({
  data: {
    type: Object as PropType<History>,
    required: true,
  },
  isStar: Boolean,
  deletable: Boolean,
  editable: Boolean,
  executable: Boolean,
  scannable: Boolean,
  starable: Boolean,
});

const emit = defineEmits<{
  (e: "delete", id: number): void;
  (e: "edit", id: number, content: string): void;
  (e: "play", id: number): void;
  (e: "qrcode", id: number): void;
  (e: "star", id: number): void;
  (e: "copy", id: number): void;
}>();

const deleteHandle = () => {
  emit("delete", props.data.id);
};

const playHandle = () => {
  emit("play", props.data.id);
};

const editHandle = () => {
  newContent.value = props.data.content;
  isEdit.value = true;
};

const qrcodeHandle = () => {
  emit("qrcode", props.data.id);
};

const starHandle = () => {
  emit("star", props.data.id);
};
const updateHandle = () => {
  isEdit.value = false;
  emit("edit", props.data.id, newContent.value);
};

const copyHandle = () => {
  emit("copy", props.data.id);
};

const isEdit = ref(false);
const newContent = ref("");
</script>
<script lang="ts"></script>
<style lang="scss">
s-card.ribo-card {
  display: block;
  max-width: unset;
  margin: 1rem;
  position: relative;
  padding-bottom: .5rem;
  .content {
    padding: .5rem;
    s-text-field {
      width: 100%;
    }
    pre {
      margin: 0;
    }
  }

  s-icon-button.btn {
    width: 1.2rem;
    height: 1.2rem;
    color: var(--s-color-primary);

    svg {
      width: .8rem;
      height: .8rem;
    }
    &.delete {
      color: var(--s-color-error);
    }
  }
}
</style>