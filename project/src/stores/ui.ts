//! 全局界面状态：缩放百分比等跨页面展示值。

import {defineStore} from "pinia";
import {ref} from "vue";

export const useUiStore = defineStore("ui", () => {
  const zoomPercent = ref(100);

  function setZoomPercent(value: number): void {
    zoomPercent.value = Math.round(value);
  }

  return {zoomPercent, setZoomPercent};
});
