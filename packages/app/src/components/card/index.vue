<template>
  <s-card class="ribo-card">
    <div class="content">
      <s-text-field v-if="isEdit" type="multiline" v-model="newContent" @blur="updateHandle"></s-text-field>
      <pre v-else>{{ content }}</pre>
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
    <s-icon-button v-if="deletable" class="btn" slot="action" @click="deleteHandle">
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
import { ref, watch } from 'vue';
import { RiboIconPlay, RiboIconDelete, RiboIconEdit, RiboIconQrcode, RiboIconStar, RiboIconStarActived } from "@/components/icons"

defineOptions({
  name: 'RiboCard',
});
const props = defineProps({
  content: {
    type: String,
    required: true
  },
  id: {
    type: Number,
    required: true
  },
  isStar: Boolean,
  deletable: Boolean,
  editable: Boolean,
  executable: Boolean,
  scannable: Boolean,
  starable: Boolean,
});

const emit = defineEmits<{
  (e: 'delete', id: number): void,
  (e: 'edit', id: number, content: string): void,
  (e: 'play', id: number): void,
  (e: 'qrcode', id: number): void,
  (e: 'star', id: number): void,
}>();

const deleteHandle = () => {
  emit('delete', props.id);
};

const playHandle = () => {
  emit('play', props.id);
}

const editHandle = () => {
  newContent.value = props.content;
  isEdit.value = true;
}

const qrcodeHandle = () => {
  emit('qrcode', props.id);
}

const starHandle = () => {
  emit('star', props.id);
}
const updateHandle = () => {
  isEdit.value = false;
  emit('edit', props.id, newContent.value);
}

const isEdit = ref(false);
const newContent = ref('');
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
  }

  s-icon-button.btn {
    width: 1.2rem;
    height: 1.2rem;

    svg {
      width: .8rem;
      height: .8rem;
    }
  }
}
</style>