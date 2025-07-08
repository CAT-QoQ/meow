<script setup>
import {onMounted, reactive, ref} from "vue";
import {nanoid} from 'nanoid/non-secure';
import {invoke} from "@tauri-apps/api/core";
import {type as os_type} from '@tauri-apps/plugin-os';
import {WebviewWindow} from '@tauri-apps/api/webviewWindow'
import MagnifyingGlassSvg from "../components/svg/magnifying-glass-svg.vue";
import distance from "jaro-winkler";
import SelectPathDialog from "../components/SelectPathDialog.vue";
import ExclamationTriangleSvg from "../components/svg/exclamation-triangle-svg.vue";
import {useRouter} from "vue-router";
import {BlobWriter, ZipReader} from "@zip.js/zip.js";
import {BlobReader, naturalSort} from '../utils';

const router = useRouter();
const isMobileApp = ref(os_type() === "android" || os_type() === "ios");
const selectPathState = ref(false);
const target_path = ref("");
const zip_list = reactive([]);
const search_text = ref("");
const render_list = reactive([]);

onMounted(async function () {
  try {
    target_path.value = localStorage.getItem('target_path') ?? "";
    await load_comic();
  } catch (e) {
    console.error(e);
  }
});

async function extractFirstImage(file) {
  let imageSrc = null;
  const zipReader = new ZipReader(new BlobReader(file, await invoke('read_file_size', {path: file})));
  const entries = (await zipReader.getEntries()).sort(naturalSort);
  for (let entry of entries) {
    if (entry.filename.match(/\.(jpg|jpeg|png|gif|bmp|svg|webp)$/i)) {
      imageSrc = URL.createObjectURL(await entry.getData(new BlobWriter()));
      break;
    }
  }
  await zipReader.close();
  return {preview: imageSrc, count: entries.length};
}

async function load_comic() {
  try {
    let dir_list = await invoke("read_dir_list", {path: target_path.value});
    render_list.length = 0;
    for (let i = 0; i < dir_list.length; i++) {
      const file = dir_list[i];
      if (file.name.split('.').pop().toLowerCase() === "zip") {
        render_list.push({...await extractFirstImage([target_path.value, file.name].join("/")), name: file.name});
      }
    }
  } catch (e) {
    console.error(e);
  }
}

function openWindows(target_path) {
  if (isMobileApp.value) {
    router.push({path: `/preview`, query: {path: target_path}});
    return;
  }
  new WebviewWindow(nanoid(), {
    url: `/preview?path=${target_path}`,
    width: 700,
    height: 1000,
    decorations: false,
    transparent: false,
    shadow: true,
    center: true
  });
}

function sortBySimilarity(target, data) {
  return [...data].sort((a, b) => {
    const distanceA = distance(target, a.name);
    const distanceB = distance(target, b.name);
    return distanceB - distanceA; // 编辑距离越小越相似
  });
}

function onSearchChange() {
  render_list.length = 0;
  render_list.push(...sortBySimilarity(search_text.value, zip_list));
}

async function saveTargetPath($event) {
  target_path.value = $event?.path?.replaceAll("\\", "") ?? "";
  localStorage.setItem('target_path', target_path.value);
  await load_comic();
}
</script>

<template>
  <div class="relative w-full">
    <div class="pt-2 px-2 w-full flex gap-2 justify-between flex-wrap">
      <div
          class="px-2 py-1.5 flex gap-1 items-center flex-wrap bg-[--light-foreground] dark:bg-[--dark-foreground] rounded-lg">
        <div class="text-sm text-[--light-text-weak-color] dark:text-[--dark-text-weak-color]">
          Access:
        </div>
        <div
            class="py-1 px-1.5 text-sm bg-[--light-embossment] dark:bg-[--dark-embossment] rounded-lg text-[--light-text-color] dark:text-[--dark-text-color] break-all cursor-pointer"
            @click="selectPathState = true">
          {{ target_path === "" ? "请点击选择路径" : target_path }}
        </div>
      </div>
      <div
          class="px-2 py-1.5 flex gap-1 items-center flex-wrap bg-[--light-foreground] dark:bg-[--dark-foreground] rounded-lg overflow-hidden hidden">
        <div
            class="w-80 h-full relative bg-[--light-embossment] dark:bg-[--dark-embossment] rounded-lg text-[--light-text-color] dark:text-[--dark-text-color]">
          <input class="pl-7 pr-1.5 size-full bg-transparent rounded-lg" placeholder="输入字符相似度排序"
                 v-model="search_text" @input="onSearchChange($event)">
          <magnifying-glass-svg
              class="absolute top-1/2 left-1.5 translate-y-[-50%] size-5 text-[--light-text-weak-color] dark:text-[--dark-text-weak-color]"/>
        </div>

      </div>
    </div>
    <div v-if="render_list.length === 0"
         class="mt-10 py-10 mx-auto flex justify-center items-center flex-col text-[--light-text-weak-color] dark:text-[--dark-text-weak-color]">
      <exclamation-triangle-svg class="size-1/4"/>
      暂无阅读文件
    </div>
    <ul :class="isMobileApp?'grid-cols-2':''"
        class="w-full p-2 grid gap-2 min-[460px]:grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 xl:grid-cols-6 2xl:grid-cols-8">
      <li v-for="item in render_list" :key="item.name"
          class=" p-2 w-full bg-[--light-foreground] dark:bg-[--dark-foreground] rounded-lg"
          @click="openWindows(target_path+'/'+item.name)">
        <div class="relative w-full rounded-lg overflow-hidden">
          <img class="w-full object-cover aspect-[7/10]" :src="item.preview" :alt="item.name">
          <div
              class="absolute top-2 right-2 rounded px-1 py-0.5 text-[--light-text-color] dark:text-[--dark-text-color] bg-[--light-foreground] dark:bg-[--dark-foreground]">
            {{ item.count }}P
          </div>
        </div>
        <div
            class="mt-2 text-base h-12 text-[--light-text-color] dark:text-[--dark-text-color] break-all line-clamp-2">
          {{ item.name }}
        </div>
      </li>
    </ul>
    <select-path-dialog v-if="selectPathState"
                        @confirm="saveTargetPath($event);selectPathState = false;"
                        @cancel="selectPathState = false">
    </select-path-dialog>
  </div>
</template>

<style scoped>

</style>