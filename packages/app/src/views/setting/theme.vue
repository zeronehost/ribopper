<template>
  <RiboSection class="theme" title="主题设置">
    <s-card>
      <div slot="headline">内置主题</div>
      <s-radio-button type="radio" value="light" v-model.lazy="currentTheme">
        <RiboIconBrightnessLight slot="unchecked" />
        <RiboIconBrightnessLightActived slot="checked" />
        <span>浅色</span>
      </s-radio-button>
      <s-radio-button type="radio" value="dark" v-model.lazy="currentTheme">
        <RiboIconBrightnessDark slot="unchecked" />
        <RiboIconBrightnessDarkActived slot="checked" />
        <span>深色</span>
      </s-radio-button>
      <s-radio-button type="radio" value="auto" v-model.lazy="currentTheme">
        <RiboIconBrightnessAuto slot="unchecked" />
        <RiboIconBrightnessAutoActived slot="checked" />
        <span>给随系统</span>
      </s-radio-button>
    </s-card>
  </RiboSection>
</template>
<script setup lang="ts">
import { RiboIconBrightnessDark, RiboIconBrightnessLight, RiboIconBrightnessAuto, RiboIconBrightnessLightActived, RiboIconBrightnessDarkActived, RiboIconBrightnessAutoActived } from '@/components/icons';
import { computed } from 'vue';
import { useSettingStore } from "@/stores/setting";
import { RiboSection } from '@/components/section';

const store = useSettingStore();

const currentTheme = computed({
  get() {
    return store.theme
  },
  set(value: "light" | "dark" | "auto") {
    store.toggleTheme(value)
  }
})

</script>
<style lang="scss">
.theme {

  s-card {
    // width: 100%;
    max-width: unset;
    margin: 1rem;
    padding: .5rem;
  }

  s-radio-button {
    &[checked=true] {
      [slot="unchecked"] {
        display: none;
      }
    }

    &[checked=false] {
      [slot="checked"] {
        display: none;
      }
    }
  }
}
</style>