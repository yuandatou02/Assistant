<template>
  <div class="mainContent" v-if="summonerData.summonerInfo!==null">
    <n-card size="small" class="shadow!" content-style="padding-bottom: 0;">
      <!--头像、昵称、等级-->
      <div class="h-14 flex gap-x-2">
        <n-avatar class="avatarEffect" round :bordered="false" :size="56"
                  :src="summonerData.summonerInfo.imgUrl"
                  fallback-src="https://wegame.gtimg.com/g.26-r.c2d3c/helper/lol/assis/images/resources/usericon/4027.png"/>
        <n-space class="grow" :size="[0,0]" justify="space-between" vertical>
          <div class="flex justify-between">
            <!--昵称-->
            <n-tag type="success" class="w-32.5! justify-center!" :bordered="false" round>
              <n-ellipsis class="max-w-27.5!" :tooltip="false">
                {{ summonerData.summonerInfo.name }}
              </n-ellipsis>
            </n-tag>
            <n-button class="px-2!" :bordered="false" @click.prevent="true" type="success" size="small" round>
              我的战绩
            </n-button>
          </div>
          <div class="flex justify-between gap-x-3">
            <n-tag type="warning" size="small" round :bordered="false">
              {{ summonerData.summonerInfo.lv }}
            </n-tag>
            <div class="grow bg-orange-400/15 px-1.5 text-orange-500 text-xs rounded-full">
              <div class="flex justify-between items-center">
                <n-progress type="line" :show-indicator="false" :percentage="summonerData.summonerInfo.xp"
                            status="warning" processing class="w-25! mt-[1.2px]!" :height="10"/>
                <div class="pt-0.5">{{ summonerData.summonerInfo.xp }} %</div>
              </div>
            </div>
          </div>
        </n-space>
      </div>
      <!--分界线-->
      <n-divider dashed style="margin: 14px 0 2px 0"/>
      <!--段位 荣誉等级-->
    </n-card>
  </div>
  <div class="mainContent" v-else>
    <start-game/>
  </div>
</template>

<script setup lang="ts">
import StartGame from "@/main/views/home/startGame.vue";
import {onMounted, reactive} from "vue";
import type {SummonerData} from "@/main/types/SummonerTypes";
import {querySummonerInfo} from "@/main/api/aboutSummoner.ts";
import {invoke} from "@tauri-apps/api/core";
import {NAvatar, NButton, NCard, NDivider, NEllipsis, NProgress, NSpace, NTag} from "naive-ui";

const summonerData = reactive<SummonerData>({
  summonerInfo: null,
});

const init = async () => {
  const summonerAllInfo = await getCurrentSummonerInfo();
  summonerData.summonerInfo = summonerAllInfo.summonerInfo;
};

const getCurrentSummonerInfo = async () => {
  const summonerInfo = await querySummonerInfo();
  return {summonerInfo};
};

onMounted(() => {
  invoke<boolean>("start_client").then(async (value: boolean) => {
    if (value) {
      await init();
    }
  });
});
console.log(summonerData);
</script>

<style scoped>

</style>