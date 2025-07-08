<script setup>
import {useRoute} from "vue-router";
import {onMounted, reactive} from "vue";
import {invoke} from "@tauri-apps/api/core";
import {Uint8ArrayWriter, ZipReader} from "@zip.js/zip.js";
import {naturalSort, BlobReader} from '../utils';

const route = useRoute();
let zip_path = route?.query?.path;
const preview_list = reactive([]);

// 自然排序函数
onMounted(async () => {
  preview_list.length = 0;
  const zipReader = new ZipReader(new BlobReader(zip_path, await invoke('read_file_size', {path: zip_path})));
  const entries = (await zipReader.getEntries()).sort(naturalSort);
  for (const entry of entries) {
    if (!entry.directory) {
      const imageData = await entry.getData(new Uint8ArrayWriter());
      const imageBlob = new Blob([imageData]);
      let imageURL = URL.createObjectURL(imageBlob);
      preview_list.push({preview: imageURL, filename: entry.filename});
    }
  }
});
</script>

<template>
  <ul class="w-full">
    <li v-for="item in preview_list" :key="item.filename" class="w-full">
      <img class="w-full object-contain" :src="item.preview" :alt="item.filename">
    </li>
  </ul>
</template>

<style scoped>

</style>