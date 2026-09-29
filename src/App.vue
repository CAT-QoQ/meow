<script setup>
import TitleBar from "./components/TitleBar.vue";
</script>

<template>
  <div class="w-full h-screen flex flex-col">
    <title-bar/>
    <div class="size-full overflow-auto">
      <RouterView v-slot="{Component}">
        <KeepAlive include="IndexView">
          <component :is="Component"/>
        </KeepAlive>
      </RouterView>
    </div>
  </div>
</template>

<style scoped>
</style>
