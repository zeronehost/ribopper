<template>
  <s-scroll-view ref="rootEl">
    <slot />
  </s-scroll-view>
</template>
<script setup lang="ts">
import { onMounted, onUnmounted, ref, shallowRef } from 'vue';

defineOptions({
  name: "RiboScrollView",
});

const props = defineProps({
  delay: {
    type: Number,
    default: 200,
  },
  distance: {
    type: Number,
    default: 0,
  },
  disabled: {
    type: Boolean,
    default: false,
  },
});

const emits = defineEmits<{
  load: [];
}>();

const rootEl = shallowRef<HTMLElement>();
const observer = shallowRef<MutationObserver>();

const destroyObserver = () => {
  if (observer.value) {
    observer.value.disconnect();
    observer.value = undefined;
  }
}
const checkFull = () => {
  if (props.disabled || rootEl.value?.clientHeight === 0) return;
  
  if ((rootEl.value?.scrollHeight ?? 0) <= (rootEl.value?.clientHeight ?? 0)) {
    emits("load");
  } else {
    destroyObserver();
  }
}
const lastScrollTop = ref(0);
const onScroll = () => {
  const { clientHeight, scrollHeight, scrollTop } = rootEl.value!;
  const delta = scrollTop - lastScrollTop.value;
  lastScrollTop.value = scrollTop;

  if (observer.value || props.disabled || delta < 0) return;

  const shouldTrigger = scrollHeight - (clientHeight + scrollTop) <= props.distance;

  if (shouldTrigger) {
    emits("load");
  }
}

onMounted(() => {
  if (rootEl.value) {
    observer.value = new MutationObserver(checkFull);

    rootEl.value?.addEventListener("scroll", onScroll);
  }
});
onUnmounted(() => {
  rootEl.value?.removeEventListener("scroll", onScroll);
  destroyObserver();
});

</script>
<style lang="scss"></style>