<template>
  <div class="main bg-neutral-100 dark:bg-neutral-900">
    <div data-tauri-drag-region class="dragDiv"/>
    <query-header class="h-10 mb-2"/>
    <div class="flex">
      <info-view v-if="matchStore.summonerInfoStore" :info-store="matchStore.summonerInfoStore"/>
    </div>
  </div>
</template>

<script setup lang="ts">
import QueryHeader from "@/queryMatch/components/queryHeader.vue";
import useMatchStore from "@/queryMatch/store";
import {onBeforeMount} from "vue";
import InfoView from "@/queryMatch/components/infoView.vue";

const matchStore = useMatchStore();
onBeforeMount(async () => {
  // 判断是否从其它窗口启动的此窗口
  const isQueryRecord = localStorage.getItem("querySummonerMatch");
  if (isQueryRecord === null) {
    await matchStore.init();
  }
});
</script>