<template>
  <s-card class="ribo-card" :class="{'is-edit': isEdit}" @click="copyHandle" clickable>
    <div class="content">
      <s-text-field v-if="isEdit && data.type === 'text'" type="multiline" v-model="newContent"
        @blur="updateHandle"></s-text-field>
      <pre v-else>{{ content }}</pre>
    </div>
    <div class="ribo-card__option">
      <!-- 执行 -->
      <!-- <s-icon-button class="btn" @click.prevent.stop="playHandle">
        <RiboIconPlay />
      </s-icon-button> -->
      <!-- 二维码 -->
      <!-- <s-icon-button class="btn" @click.prevent.stop="qrcodeHandle">
        <RiboIconQrcode />
      </s-icon-button> -->
      <!-- 编辑 -->
      <s-icon-button v-if="data.type === 'text'" class="btn" @click.prevent.stop="editHandle">
        <RiboIconEdit />
      </s-icon-button>
      <!-- 删除 -->
      <s-icon-button class="btn delete" @click.prevent.stop="deleteHandle">
        <RiboIconDelete />
      </s-icon-button>
      <!-- 收藏 -->
      <!-- <s-icon-button class="btn" @click.prevent.stop="favoritesHandle">
        <RiboIconStarActived v-if="data.favorites" />
        <RiboIconStar v-else />
      </s-icon-button> -->
    </div>

  </s-card>
</template>
<script lang="ts" setup>
import type { RiboRecordWithTargets } from "@ribo/api";
import { computed, type PropType, ref } from "vue";
import {
  RiboIconDelete,
  RiboIconEdit,
  // RiboIconPlay,
  // RiboIconQrcode,
  // RiboIconStar,
  // RiboIconStarActived,
} from "@/components/icons";

defineOptions({
  name: "RiboCard",
});
const props = defineProps({
  data: {
    type: Object as PropType<RiboRecordWithTargets>,
    required: true,
  },
  collectible: Boolean,
  deletable: Boolean,
  editable: Boolean,
  executable: Boolean,
  scannable: Boolean,
});

const content = computed(() => props.data.type === "text" ? props.data.text : "");
const id = computed(() => props.data.id);

const emit = defineEmits<{
  (e: "delete", id: number): void;
  (e: "edit", id: number, content: string): void;
  (e: "exec", id: number): void;
  (e: "qrcode", id: number): void;
  (e: "favorites", id: number): void;
  (e: "copy", id: number): void;
}>();

const deleteHandle = () => {
  emit("delete", id.value);
};

// const playHandle = () => {
//   emit("exec", id.value);
// };

const editHandle = () => {
  newContent.value = content.value;
  isEdit.value = true;
};

// const qrcodeHandle = () => {
//   emit("qrcode", id.value);
// };

// const favoritesHandle = () => {
//   emit("favorites", props.data.id);
// };
const updateHandle = () => {
  isEdit.value = false;
  emit("edit", id.value, newContent.value);
};

const copyHandle = () => {
  if (isEdit.value) {
    return;
  }
  emit("copy", id.value);
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

  .content {
    padding: .5rem;

    s-text-field {
      width: 100%;
    }

    pre {
      margin: 0;
      max-height: 5rem;
      overflow: hidden;
    }
  }

  .ribo-card {
    &__option {
      position: absolute;
      right: 0;
      bottom: 0;
      top: 0;
      align-items: center;
      justify-content: flex-end;
      padding: 0.5rem 1rem 0.5rem 1.5rem;
      background: linear-gradient(to right, #0000, var(--s-color-background) 10%, var(--s-color-background));
      display: none;
    }
  }
  &:not(.is-edit):hover {
    .ribo-card__option {
      display: flex;
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