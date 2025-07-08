<script setup>
import XMarkSvg from "./svg/x-mark-svg.vue";
import {onMounted, reactive} from "vue";
import {invoke} from "@tauri-apps/api/core";

const emit = defineEmits(['confirm', 'cancel']);

const accessPaths = reactive([]);
const directoryList = reactive([]);

onMounted(async () => {
  await queryPathList(accessPaths.join("/"));
});

async function queryPathList(path) {
  let dir_list = await invoke("read_dir_list", {path});
  directoryList.length = 0;
  for (let i = 0; i < dir_list?.length; i++) {
    if (dir_list[i]?.["is_dir"]) {
      directoryList.push(dir_list[i].name);
    }
  }
}


</script>

<template>
  <div class="fixed top-0 left-0 w-screen h-screen box-border backdrop-blur">
    <div
        class="relative top-0 left-0 size-full bg-[--light-background] dark:bg-[--dark-background] opacity-60 dark:opacity-85"></div>
    <div class="pt-12 absolute top-0 left-0 size-full flex justify-center items-center flex-col box-border">
      <div
          class="w-1/2 min-w-80 h-2/3 flex flex-col bg-[--light-foreground] dark:bg-[--dark-foreground] rounded-lg overflow-hidden">
        <div
            class="px-2 py-1.5 w-full flex justify-between items-center shadow-sm dark:shadow">
          <div class="text-[--light-text-weak-color] dark:text-[--dark-text-weak-color]">选择访问的路径</div>
          <x-mark-svg
              class="size-6 text-[--light-text-weak-color] dark:text-[--dark-text-weak-color] hover:text-[--light-accent-colour] dark:hover:text-[--dark-accent-colour] cursor-pointer"
              @click="emit('cancel')"/>
        </div>
        <ul class="px-2 py-2 size-full overflow-auto flex flex-col gap-2">
          <li v-if="accessPaths.length > 0"
              class="px-2 py-1.5 w-full text-[--light-text-weak-color] dark:text-[--dark-text-weak-color] break-all bg-[--light-embossment] dark:bg-[--dark-embossment] hover:text-[--light-accent-colour] dark:hover:text-[--dark-accent-colour] rounded-lg cursor-pointer"
              @click="accessPaths.pop();queryPathList(accessPaths.join('/'))">
            . . . 返回上一级
          </li>
          <li v-for="item in directoryList" :key="item"
              class="px-2 py-1.5 w-full text-[--light-text-weak-color] dark:text-[--dark-text-weak-color] break-all bg-[--light-embossment] dark:bg-[--dark-embossment] hover:text-[--light-accent-colour] dark:hover:text-[--dark-accent-colour] rounded-lg cursor-pointer"
              @click="accessPaths.push(item);queryPathList(accessPaths.join('/'))">
            {{ item }}
          </li>
        </ul>
      </div>
      <div
          class="mt-5 py-1 w-1/4 min-w-36 font-bold text-lg text-center text-[--light-background] dark:text-[--dark-background] bg-[--light-accent-colour] dark:bg-[--dark-accent-colour] rounded-lg cursor-pointer select-none"
          @click="emit('confirm',{path:accessPaths.join('/')})">
        确认
      </div>
    </div>
  </div>
</template>

<style scoped>

</style>