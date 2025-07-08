<script setup>
import {ref, onMounted} from "vue";
import {type as os_type} from '@tauri-apps/plugin-os';
import {getCurrentWindow} from "@tauri-apps/api/window";
import {invoke} from "@tauri-apps/api/core";
import CatLogoSvg from "./svg/cat-logo-svg.vue";
import MinimizeSvg from "./svg/minimize-svg.vue";
import ArrowsPointingInSvg from "./svg/arrows-pointing-in-svg.vue";
import ArrowsPointingOutSvg from "./svg/arrows-pointing-out-svg.vue";
import XMarkSvg from "./svg/x-mark-svg.vue";
import SunSvg from "./svg/sun-svg.vue";
import MoonSvg from "./svg/moon-svg.vue";

let appWindow = window?.["__TAURI_INTERNALS__"] && getCurrentWindow();
const isDesktopApp = ref(appWindow != null && os_type() !== "android" && os_type() !== "ios");
const isDarkState = ref(true);
const isMaximized = ref(false);
const isMouseDown = ref(false);
let startX = 0;
let startY = 0;
let threshold = 5;

onMounted(async () => {
  if (isDesktopApp.value) {
    isMaximized.value = await appWindow?.isMaximized();
  }
  isDarkState.value = localStorage.getItem('theme') !== 'light';
  document.documentElement.classList.toggle('dark', isDarkState.value);
  await invoke("update_theme_state", {isDark: isDarkState.value});
});

// 目前通过使用自定义事件解决窗口标题栏双击取消缩放时出现startDragging和toggleMaximize的冲突问题,导致无法缩放.
// 出现事件无效时,查看控制台错误信息,进行"src-tauri/capabilities/default.json"的permissions对象添加对于的权限申请.
const onMouseDown = (e) => {
  if (isDesktopApp.value && e.button === 0) {
    isMouseDown.value = true;
    startX = e.clientX;
    startY = e.clientY;
  }
};

const onMouseMove = (e) => {
  if (isDesktopApp.value && isMouseDown.value) {
    const dx = Math.abs(e.clientX - startX);
    const dy = Math.abs(e.clientY - startY);
    if (dx + dy > threshold) {
      isMouseDown.value = false;
      appWindow?.startDragging();
    }
  }
};

async function toggleMaximize() {
  if (isDesktopApp.value) {
    isMaximized.value = !isMaximized.value;
    await appWindow?.toggleMaximize();
  }
}

async function toggleTheme() {
  isDarkState.value = !isDarkState.value;
  document.documentElement.classList.toggle('dark', isDarkState.value);
  localStorage.setItem('theme', isDarkState.value ? 'dark' : 'light');
  await invoke("update_theme_state", {isDark: isDarkState.value});
}

function showHelp() {
  alert('QQ群:1025084354,管理CAT的免费程序,请勿售卖');
}

// (async () => {
//   //实现窗口事件穿透
//   //需要在src-tauri/capabilities/default.json中的permissions添加"core:window:allow-set-ignore-cursor-events"权限。
//   console.log(await getCurrentWindow().setIgnoreCursorEvents(true));
// })();
</script>

<template>
  <div
      :class="!isDesktopApp?'pt-6 h-[4.5rem]':''"
      class="shrink-0 sticky top-0 px-2 w-full h-12 bg-[--light-foreground] dark:bg-[--dark-foreground] flex justify-between items-center gap-2 select-none z-[9999]"
      @mousedown="onMouseDown" @mousemove="onMouseMove" @dblclick="toggleMaximize();" @mouseout="isMouseDown=false">
    <div
        class="flex shrink-0 items-center text-base text-[--light-text-color] dark:text-[--dark-text-weak-color] select-none"
        @click="showHelp($event)">
      <cat-logo-svg class="size-8"/>
      <span>Meow</span>
    </div>
    <div class="size-full overflow-hidden">
      <slot></slot>
    </div>
    <ul class="flex shrink-0 justify-center items-center text-[--light-text-color] dark:text-[--dark-text-weak-color]">
      <li class="flex justify-center items-center size-9 rounded-md hover:bg-[--light-embossment] dark:hover:bg-[--dark-embossment]"
          @click="toggleTheme();">
        <sun-svg class="size-5" v-if="isDarkState"/>
        <moon-svg class="size-5" v-else/>
      </li>
      <li v-if="isDesktopApp"
          class="flex justify-center items-center size-9 rounded-md hover:bg-[--light-embossment] dark:hover:bg-[--dark-embossment]"
          @click="appWindow?.minimize();">
        <minimize-svg class="size-5"/>
      </li>
      <li v-if="isDesktopApp"
          class="flex justify-center items-center size-9 rounded-md hover:bg-[--light-embossment] dark:hover:bg-[--dark-embossment]"
          @click="toggleMaximize();">
        <arrows-pointing-in-svg class="size-5" v-if="isMaximized"/>
        <arrows-pointing-out-svg class="size-5" v-else/>
      </li>
      <li v-if="isDesktopApp"
          class="flex justify-center items-center size-9 rounded-md hover:bg-[--light-embossment] dark:hover:bg-[--dark-embossment]"
          @click="appWindow?.close();">
        <x-mark-svg class="size-5"/>
      </li>
    </ul>
  </div>
</template>

<style scoped>

</style>