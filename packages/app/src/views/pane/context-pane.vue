<template>
  <View :appbar="false" label="context_pane" />
</template>
<script setup lang="ts">
import { inject, nextTick, onMounted, onUnmounted } from "vue";
import View from "./view.vue";
import { rootContextKey } from "@/utils/types";
import { EVENT_LABEL_ALL, EVENT_LABEL_CONFIG, EVENT_LABEL_RECORD, EVENT_LABEL_TARGET, EVENT_TYPE_INIT, EVENT_TYPE_UPDATE, type RiboEvent } from "@ribo/api";
import { useRecordStore } from "@/stores/record";
import { useSettingStore } from "@/stores/setting";

const context = inject(rootContextKey);
const recordStore = useRecordStore();
const settingStore = useSettingStore();
const loadRecords = async (event: RiboEvent) => {
  if (
    (event.type === EVENT_TYPE_INIT || event.type === EVENT_TYPE_UPDATE)
    && (
      event.label === EVENT_LABEL_RECORD
      || event.label === EVENT_LABEL_TARGET
      || event.label === EVENT_LABEL_ALL
    )) {
    await nextTick();
    if (event.action === "DELETE" || event.action === "UPDATE") return;
    // await recordStore.getAllRecords();
    await recordStore.reset();
  }
  if (
    (event.type === EVENT_TYPE_INIT || event.type === EVENT_TYPE_UPDATE)) {
    if (
      event.label === EVENT_LABEL_CONFIG
      || event.label === EVENT_LABEL_ALL
    ) {
      await settingStore.loadConfig();
    }
  }
};

onMounted(() => {
  // recordStore.getAllRecords();
  settingStore.getAppInfo();
  context?.register(loadRecords);
});
onUnmounted(() => {
  context?.unregister(loadRecords);
});
</script>