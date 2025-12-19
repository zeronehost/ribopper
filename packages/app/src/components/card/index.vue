<template>
  <s-card class="ribo-card" @click="copyHandle" clickable>
    <div class="content">
      <pre>{{ content }}</pre>
    </div>
    <div class="ribo-card__option">
      <!-- 执行 -->
      <s-icon-button v-if="false" class="btn" @click.prevent.stop="playHandle">
        <RiboIconPlay />
      </s-icon-button>
      <!-- 二维码 -->
      <s-icon-button class="btn" @click.prevent.stop="qrcodeHandle">
        <RiboIconQrcode />
      </s-icon-button>
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
import { computed, type PropType } from "vue";
import {
  RiboIconDelete,
  RiboIconEdit,
  RiboIconPlay,
  RiboIconQrcode,
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
  (e: "option", option: "delete" | "edit" | "exec" | "copy" | "qrcode", id: number): void;
}>();

const deleteHandle = () => {
  emit("option", "delete", id.value);
};

const playHandle = () => {
  emit("option", "exec", id.value);
};


const qrcodeHandle = () => {
  emit("option", "qrcode", id.value);
};

const editHandle = () => {
  emit("option", "edit", id.value);
};


const copyHandle = () => {
  emit("option", "copy", id.value);
};
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
  &:hover {
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