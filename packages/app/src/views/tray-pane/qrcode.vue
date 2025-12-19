<template>
  <section class="qrcode-pane">
    <s-appbar v-if="route.name !== 'trayPane'">
      <s-icon-button slot="navigation" @click="goback">
        <s-icon name="arrow_back"></s-icon>
      </s-icon-button>
    </s-appbar>
    <s-card>
      <img :src="src" />
    </s-card>
  </section>
</template>
<script setup lang="ts">
import { ref } from 'vue';
import { qrcodeRecord } from "@ribo/api";
import { useRouter, useRoute } from 'vue-router';

const router = useRouter();
const route = useRoute();

const goback = () => router.back();

const id = route.query.id;

const src = ref('');

if (id) {
  qrcodeRecord(Number(id)).then((res) => {
    // src.value = res;
    console.log(res);
    // const buf = Uint8Array.from(res);
    const reader = new FileReader();
    reader.readAsDataURL(res);
    reader.onload = (e) => {
      src.value = (e.target?.result as string) ?? "";
    }
  })
} else {
  goback();
}
</script>
<style lang="scss">
section.qrcode-pane {
  display: flex;
  flex-direction: column;
  height: 100vh;
  overflow: hidden;
  s-card {
    flex: 1;
    padding: 0.5rem;
    margin: 0.5rem;
    max-width: unset;

    img {
      width: 100%;
    }
  }
}
</style>