<template>
  <section class="edit-pane">
    <s-appbar>
      <s-icon-button slot="navigation" @click="goback">
        <s-icon name="arrow_back"></s-icon>
      </s-icon-button>
    </s-appbar>
    <s-card>
      <textarea slot="text" v-model="value"></textarea>
    </s-card>
    <footer class="edit-footer">
      <s-button type="elevated" :disabled="value === originVal" @click="saveHandle">保存</s-button>
      <s-button type="elevated" @click="cancelHandle">取消</s-button>
    </footer>
  </section>
</template>
<script setup lang="ts">
import { getRecord, logger, updateRecord } from '@ribo/api';
import { Snackbar } from 'sober';
import { ref } from 'vue';
import { useRouter, useRoute } from 'vue-router';

const router = useRouter();
const route = useRoute();

const goback = () => router.back();

const value = ref('');
const originVal = ref('');

const id = ref(Number(route.query.id));

getRecord(id.value).then((res) => {
  if (res.type === "text") {
    value.value = res.text;
    originVal.value = res.text;
  }
});

const saveHandle = () => {
  updateRecord({
    id: id.value,
    type: "text",
    text: value.value
  }).then(() => {
    goback();
  }).catch((e) => {
    logger.error(e);
    Snackbar.builder({
      text: "修改失败",
      duration: 1000,
      type: "error",
    });
  })
}
const cancelHandle = () => {
  value.value = originVal.value;
  goback();
}
</script>
<style lang="scss">
section.edit-pane {
  display: flex;
  flex-direction: column;
  height: 100vh;
  overflow: hidden;

  s-card {
    flex: 1;
    padding: 0.5rem;
    margin: 0.5rem;
    max-width: unset;
  }

  textarea {
    margin: 0;
    width: 100%;
    height: 100%;
    resize: none;
    box-sizing: border-box;
    border-radius: 6px;
    caret-color: var(--s-color-primary, #006782);
    padding: 1rem;
    outline: none;

    &:focus,
    &:active {
      border-width: 2px;
      border-color: var(--s-color-primary, #006782);
    }
  }

  footer {
    padding: 0.5rem;
    display: flex;
    align-items: center;
    justify-content: end;
    gap: 0.5rem;

    s-button {
      border-radius: 6px;
    }
  }
}
</style>