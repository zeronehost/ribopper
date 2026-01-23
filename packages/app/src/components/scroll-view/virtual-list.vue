<template>
  <s-scroll-view ref="rootEl" class="ribo-virtual-list" @scroll="scrollHandle">
    <div class="ribo-virtual-list__wrapper" ref="wrapperEl">
      <template v-for="item in visibleData">
        <slot :data="item.value" :id="item.index" :selected="item.index === selected" />
      </template>
    </div>
  </s-scroll-view>
</template>
<script setup lang="ts">
import type { Record } from '@ribo/api';
import { computed, nextTick, onMounted, ref, shallowRef, watch, type PropType } from 'vue';

defineOptions({
  name: "RiboVirtualList"
});

const props = defineProps({
  list: {
    type: Array as PropType<Array<Record>>,
    default: () => []
  },
  // 预估高度
  estimatedItemHeight: {
    type: [Number, String],
    required: true
  },
});

const rootEl = ref<HTMLElement>();
const startIndex = ref<number>(0);
const endIndex = ref<number>();
const height = ref<number>(0);
const offset = ref<number>(0);
const selected = ref<number>(-1);
const eItemHeight = computed<number>(() => {
  if (typeof props.estimatedItemHeight === "number") return props.estimatedItemHeight;
  if (typeof props.estimatedItemHeight === "string") {
    if (/\d+px$/.test(props.estimatedItemHeight)) return parseInt(props.estimatedItemHeight);
    if (/\d+rem$/.test(props.estimatedItemHeight)) {
      const rootFontSize = parseInt(window.getComputedStyle(document.documentElement).fontSize);
      return parseInt(props.estimatedItemHeight) * rootFontSize;
    }
  }
  return parseInt(props.estimatedItemHeight);
})
const listData = computed(() => props.list.map((item, index) => ({ index: index, value: item })));
const visibleCount = computed(() => Math.ceil(height.value / eItemHeight.value));
const visibleData = computed(() => listData.value.slice(startIndex.value, endIndex.value));
const positions = shallowRef<Position[]>([]);

// watch(props.list, (value, _, oncleanup) => {
//   let expired = false;
//   oncleanup(() => expired = true);
//   if (!expired) return;
//   nextTick(() => {
//     positions.value = value.map((_, index) => ({
//       index,
//       height: eItemHeight.value,
//       top: index * eItemHeight.value,
//       bottom: (index + 1) * eItemHeight.value
//     }));
//     updateItemSize();
//   });
// }, { flush: "post" });

const scrollHandle = () => {
  const scrollTop = rootEl.value?.scrollTop ?? 0;
  startIndex.value = getStartIndex(scrollTop) ?? 0;
  endIndex.value = startIndex.value + visibleCount.value;
  setOffset();
}

const getStartIndex = (scrollTop: number = 0) => {
  return binarySearch(positions.value, scrollTop);
}

const binarySearch = (list: Position[], scrollTop: number) => {
  let start = 0;
  let end = list.length - 1;
  let mid: number | null = null;

  while (start <= end) {
    let midIndex = parseInt(`${(start + end) / 2}`);
    let midValue = list[midIndex]?.bottom ?? 0;
    if (midValue === scrollTop) {
      return midIndex + 1;
    }
    if (midValue < scrollTop) {
      start = midIndex + 1;
    } else if (midValue > scrollTop) {
      if (mid === null || mid > midIndex) {
        mid = midIndex;
      }
      end = end - 1;
    }
  }
  return mid;
}

const setOffset = () => {
  offset.value = startIndex.value > 0 ? (positions.value[startIndex.value - 1]?.bottom ?? 0) : 0;
}

const next = () => {
  if (props.list.length === 0) return;
  selected.value = selected.value + 1;
  if (selected.value >= positions.value.length) {
    selected.value = 0;
  }
  scrollToIndex();
}

const prev = () => {
  if (props.list.length === 0) return;
  selected.value = selected.value - 1;
  if (selected.value < 0) {
    selected.value = positions.value.length - 1;
  }
  scrollToIndex();
}

const scrollToIndex = () => {
  const position = positions.value[selected.value > 0 ? selected.value - 1 : selected.value];
  rootEl.value?.scrollTo({
    top: position?.top,
    behavior: "auto"
  })
}

const wrapperEl = ref<HTMLElement>();
const isUpdatingPositions = ref(false);
const updateItemSize = () => {
  console.log("updateItemSize =>");
  if (isUpdatingPositions.value) return;
  isUpdatingPositions.value = true;

  const nodes = wrapperEl.value?.childNodes ?? [];
  let hasChanges = false;

  nodes.forEach(el => {
    if (el.nodeType !== 1) return;
    const { height } = (el as HTMLElement).getBoundingClientRect();
    const index = Number((el as HTMLElement).dataset.id);
    const oldHeight = positions.value[index]?.height ?? height;
    const val = oldHeight - height;
    if (val) {
      positions.value[index]!.bottom = positions.value[index]!.bottom - val;
      positions.value[index]!.height = height;
      for (let i = index + 1; i < positions.value.length; i++) {
        positions.value[i]!.top = positions.value[i - 1]!.bottom;
        positions.value[i]!.bottom = positions.value[i]!.bottom - val;
      }
      hasChanges = true;
    }
  });

  if (hasChanges) {
    totalHeight.value = positions.value[positions.value.length - 1]?.bottom ?? height.value;
  }

  isUpdatingPositions.value = false;
}

onMounted(() => {
  height.value = rootEl.value?.clientHeight ?? 0;
  startIndex.value = 0;
  endIndex.value = startIndex.value + visibleCount.value;

  // nextTick(() => {
  //   updateItemSize();
  // });
});

const totalHeight = ref(0);

// onUnmounted(() => {
// })

// style v-bind
const styleTotalHeight = computed(() => `${totalHeight.value}px`);
const styleOffset = computed(() => `${offset.value ?? 0}px`);

defineExpose({
  next,
  prev,
  selected
});
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
    height: v-bind(styleTotalHeight);
  }

  &__wrapper {
    position: absolute;
    left: 0;
    right: 0;
    top: 0;
    transform: translate3d(0, v-bind(styleOffset), 0);
    display: flow-root;
  }
}
</style>
