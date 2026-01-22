<template>
  <section ref="rootEl" class="ribo-virtual-list" @scroll="scrollHandle">
    <div class="ribo-virtual-list__wrapper" ref="wrapperEl">
      <div class="ribo-virtual-list__item" v-for="item in visibleData">
        <slot :data="item" />
      </div>
    </div>
  </section>
</template>
<script setup lang="ts">
import { computed, onMounted, ref, shallowRef, type PropType } from 'vue';

defineOptions({
  name: "RiboVirtualList"
});

const props = defineProps({
  list: {
    type: Array as PropType<Array<{ value: number }>>,
    default: () => []
  },
  // 预估高度
  estimatedItemHeight: {
    type: Number,
    required: true
  },
  height: {
    type: [Number, String],
    default: "100%"
  }
});

const rootEl = ref<HTMLElement>();
const startIndex = ref<number>(0);
const endIndex = ref<number>();
const height = ref<number>(0);
const offset = ref<number>(0);
const listData = computed(() => props.list.map((item, index) => ({index: index, value: item})));
const visibleCount = computed(() => Math.ceil(height.value / props.estimatedItemHeight));
const visibleData = computed(() => listData.value.slice(startIndex.value, endIndex.value));
const positions = shallowRef<Position[]>([]);

const scrollHandle = () => {
  const scrollTop = rootEl.value?.scrollTop ?? 0;
  startIndex.value = Math.floor(scrollTop / props.itemSize);
  endIndex.value = startIndex.value + visibleCount.value;
  offset.value = scrollTop - (scrollTop % props.itemSize);
}

const init = () => {
  positions.value = listData.value.map((d, index) => ({
    index,
    height: props.estimatedItemHeight,
    top: index * props.estimatedItemHeight,
    bottom: (index + 1) * props.estimatedItemHeight
  }));
}

const getStartIndex = (scrollTop: number = 0) => {
  return binarySearch(positions.value, scrollTop);
}

const binarySearch = (list: Position[], scrollTop: number) => {
  let start = 0;
  let end = list.length - 1;
  let mid: number | null = null;

  while (start <= end) {
    let midIndex = (start + end) / 2;
    let midValue = list[midIndex]?.bottom??0;
    if (midValue === scrollTop) {
      return midIndex + 1;
    }
    if (midValue < scrollTop) {
      start = midIndex + 1;
    } else if (midValue > scrollTop){
      if (mid === null || mid > midIndex) {
        mid = midIndex;
      }
      end = midIndex - 1;
    }
  }
  return mid;
}

const wrapperEl = ref<HTMLElement>();
const observer = new MutationObserver((mutations, observer) => {
  for (const mutation of mutations) {
    if (mutation.type === "childList") {
      
    }
  }
});

observer.observe(wrapperEl.value!, { subtree: true, childList: true})
const updateItemSize = () => {
  let nodes = 
}

onMounted(() => {
  height.value = rootEl.value?.clientHeight ?? 0;
  startIndex.value = 0;
  endIndex.value = startIndex.value + visibleCount.value;
});

// style v-bind
const totalHeight = computed(() => `${props.list.length * props.itemSize}px`);
const styleOffset = computed(() => `${offset.value ?? 0}px`);
const sryleItemHeight = computed(() => `${props.itemSize}px`);
</script>
<script lang="ts">
  export interface Position {
    index: number,
    height: number,
    top: number,
    bottom: number
  }
</script>
<style lang="scss">
.ribo-virtual-list {
  height: 100%;
  overflow: auto;
  position: relative;
  -webkit-overflow-scrolling: touch;

  &::after {
    content: "";
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    z-index: -1;
    min-height: 100%;
    height: v-bind(totalHeight);
  }

  &__wrapper {
    position: absolute;
    left: 0;
    right: 0;
    top: 0;
    transform: translate3d(0, v-bind(styleOffset), 0);
  }

  &__item {
    height: v-bind(sryleItemHeight);
    line-height: v-bind(sryleItemHeight);
  }
}
</style>
