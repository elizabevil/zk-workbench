//! 全局界面状态：缩放百分比等跨页面展示值。

import {defineStore} from "pinia";
import {ref} from "vue";

export const useUiStore = defineStore("ui", () => {
  const zoomPercent = ref(100);
  const zoomResetRevision = ref(0);
  const aboutOpen = ref(false);

  function setZoomPercent(value: number): void {
    zoomPercent.value = Math.round(value);
  }

  function resetZoom(): void {
    zoomResetRevision.value += 1;
  }

  function openAbout(): void {
    aboutOpen.value = true;
  }

  function closeAbout(): void {
    aboutOpen.value = false;
  }

  return {
    zoomPercent,
    zoomResetRevision,
    aboutOpen,
    setZoomPercent,
    resetZoom,
    openAbout,
    closeAbout,
  };
});
