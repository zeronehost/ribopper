<template>
  <s-scroll-view class="ribo-virtual-list" ref="containerRef">
    <section v-if="data.length" class="ribo-virtual-list__wrapper" ref="wrapperRef">
      <div
        v-for="data in renderedItems"
        :key="data.index"
        :data-index="data.index"
        :ref="(el) => setItemRef(el, data.index)"
        class="ribo-virtual-list__item"
      >
        <slot :data="data.item" :index="data.index" />
      </div>
    </section>
    <s-empty v-else>{{ emptyTip }}</s-empty>
  </s-scroll-view>
</template>
<script setup lang="ts">
import { useVirtualScroll, type VirtualScrollProps } from "@pdanpdan/virtual-scroll";
import { Record } from "@ribo/api";
import { computed, nextTick, ref, watch, type PropType } from "vue";

defineOptions({
  name: "RiboVirtualList"
})

const props = defineProps({
  data: {
    type: Array as PropType<Record[]>,
    default: () => []
  },
  emptyTip: {
    type: String,
    default: "暂时没有内容"
  },
  loadDistance: {
    type: Number,
    default: 100,
  },
  gap: {
    type: Number,
    default: 16,
  },
  loading: Boolean,
  finished: Boolean,
  current: {
    type: Object,
    default: () => ({id: -1, index: -1}),
  }
});

const containerRef = ref();
const wrapperRef = ref();
const virtualScrollProps = computed<VirtualScrollProps<Record>>(() => {
  return {
    items: props.data,
    direction: "vertical",
    hostElement: wrapperRef.value,
    loading: props.loading,
    loadDistance: props.loadDistance,
    container: containerRef.value,
    gap: props.gap,
  }
})
const {
  renderedItems,
  scrollDetails,

  updateItemSizes,
  scrollToOffset,
  refresh,
  stopProgrammaticScroll,
} = useVirtualScroll<Record>(virtualScrollProps);

const current = ref(props.current);

watch(() => props.current, (value) => {
  current.value = value;
}, { deep: true });

defineExpose({
  async prev() {
    let index = current.value.index;
    if (current.value.index >= 0) {
      index = current.value.index === 0 ? props.data.length - 1 : current.value.index - 1;
    } else {
      index = scrollDetails.value.currentIndex;
    }
    const item = renderedItems.value.find((item) => item.index === index);
    if (item) {
      stopProgrammaticScroll();
      await nextTick();
      scrollToOffset(item.originalX, item.originalY);
      await nextTick();
      const id = props.data[index]?.id ?? -1;
      emit("update:current", { id, index });
    }
  },
  async next() {
    let index = current.value.index;
    if (current.value.index >= 0) {
      index = current.value.index === props.data.length - 1 ? 0 : current.value.index + 1;
    } else {
      index = scrollDetails.value.currentIndex;
    }
    const item = renderedItems.value.find((item) => item.index === index);
    if (item) {
      stopProgrammaticScroll();
      await nextTick();
      scrollToOffset(item.originalX, item.originalY);
      await nextTick();
      const id = props.data[index]?.id ?? -1;
      emit("update:current", { id, index });
    }
  },
  refresh,
});

const emit = defineEmits<{
  load: [],
  "update:current": [{id:number, index: number}]
}>();

watch(scrollDetails, (details) => {
  // emit("scroll", details);
  // if (
  //   !oldDetails
  // ) {}
  // if (!details.isProgrammaticScroll) {
  //   emit("update:current", { id: -1, index: -1 });
  // }
  if (props.loading || props.finished) return;

  const remaining = details.totalSize.height - (details.scrollOffset.y + details.viewportSize.height);
  if (remaining <= props.loadDistance) {
    emit("load");
  }
});

const itemRefs = new Map<number, HTMLElement>();
const itemResizeObserver = new ResizeObserver((entries) => {
  const updates = [];
  for (const entry of entries) {
    const target = entry.target as HTMLElement;
    const index = Number(target.dataset.index);
    let inlineSize = entry.contentRect.width;
    let blockSize = entry.contentRect.height;
    if (entry.borderBoxSize && entry.borderBoxSize.length > 0) {
      inlineSize = entry.borderBoxSize[0]?.inlineSize ?? 0;
      blockSize = entry.borderBoxSize[0]?.blockSize ?? 0;
    } else {
      inlineSize = target.offsetWidth;
      blockSize = target.offsetHeight;
    }
    if (!Number.isNaN(index)) {
      updates.push({
        index, inlineSize, blockSize, element: target
      })
    }
  }
  if (updates.length) {
    updateItemSizes(updates);
  }
})
const setItemRef = (el: any, index: number) => {
  if (el) {
    itemRefs.set(index, el);
    itemResizeObserver.observe(el);
  } else {
    const oldEl = itemRefs.get(index);
    if (oldEl) {
      itemResizeObserver.unobserve(oldEl);
      itemRefs.delete(index);
    }
  }
}

const styleTotalHeight = computed(() => {
  return `${scrollDetails.value.totalSize.height}px`;
});
const styleOffset = computed(() => {
  return `${scrollDetails.value.scrollOffset.y}px`;
});
const styleGap = computed(() => {
  return `${props.gap}px`;
})
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
    pointer-events: none;
  }

  &__wrapper {
    position: absolute;
    left: 0;
    right: 0;
    top: 0;
    transform: translate3d(0, v-bind(styleOffset), 0);
    display: flex;
    flex-direction: column;
    gap: v-bind(styleGap);
    padding: v-bind(styleGap) 0;
  }

  &__item {
    flex: 1;
    padding: 0 0.5rem;
    position: relative;
  }
}
</style>