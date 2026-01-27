<template>
  <div class="flex flex-col w-63.5">
    <n-card size="small" class="shadow!" content-style="padding-bottom:0">
      <!--头像 昵称 等级-->
      <div class="h-14 flex gap-x-2">
        <!--头像-->
        <n-avatar class="avatarEffect" round :bordered="false" :size="56" :src="infoStore.info.imgUrl"
                  fallback-src="https://wegame.gtimg.com/g.26-r.c2d3c/helper/lol/assis/images/resources/usericon/4027.png"/>
        <n-space class="grow" :size="[0,0]" justify="space-between" vertical>
          <div class="flex justify-between">
            <!--昵称-->
            <n-tag class="justify-center w-41" :bordered="false" type="success" round>
              <n-ellipsis class="max-w-35!">
                {{ infoStore.info.name }}
              </n-ellipsis>
            </n-tag>
          </div>
          <!--等级-->
          <div class="flex justify-between gap-x-3">
            <div class="grow bg-[rgba(240,160,32,0.15)] px-1.75 py-0 text-[#f0a020] text-xs rounded-xl">
              <div class="flex justify-between items-center gap-x-2">
                <n-progress type="line" :show-indicator="false" :percentage="infoStore.info.xp" status="warning"
                            processing :height="10"/>
                <div class="pt-px">
                  <n-ellipsis class="max-w-10!">
                    {{ infoStore.info.lv }}
                  </n-ellipsis>
                </div>
              </div>
            </div>
          </div>
        </n-space>
      </div>
      <!--排位数据-->
      <n-list class="mt-5.25!">
        <n-list-item v-for="rank in rankRender">
          <div class="flex justify-between">
            <n-tag class="w-19 justify-center" type="success" :bordered="false" :round="false">
              {{ rank.title }}
            </n-tag>
            <n-tag class="w-19 justify-center" type="warning" :bordered="false" :round="false">
              {{ rank.value }}
            </n-tag>
          </div>
        </n-list-item>
      </n-list>
    </n-card>
    <!--排位数据-->
    <n-card size="small" class="mt-3 shadow h-84.25" content-style="padding-top:10px">
      <div class="pl-0.5" v-if="true">
        <n-steps size="small" vertical>
          <n-step style="margin: 4px 0" title="近期使用的英雄">
            <template #icon>
              <n-icon>
                <crown/>
              </n-icon>
            </template>
            <n-space justify="space-between">
              <n-space vertical :size="[0,2.5]" v-for="index in 3" :key="index">
                <n-skeleton height="45px" width="45px" :sharp="false"/>
                <n-tag :bordered="false" size="small" class="text-sm w-11.25 justify-center"/>
              </n-space>
            </n-space>
          </n-step>
          <n-step style="margin: 0" title="近期活跃度">
            <template #icon>
              <n-icon>
                <planet/>
              </n-icon>
            </template>
            <n-space class="pt-1" :size="[12,16]" justify="space-between">
              <n-space :size="[0,3]" vertical v-for="index in 6" :key="index">
                <n-skeleton height="45px" circle/>
                <n-tag :bordered="false" round class="w-11.25 h-5.5 px-3">
                  <div class="absolute top-1.75 right-1.25"/>
                </n-tag>
              </n-space>
            </n-space>
          </n-step>
        </n-steps>
      </div>

    </n-card>
  </div>
</template>

<script setup lang="ts">
import {
  NAvatar,
  NCard,
  NEllipsis,
  NIcon,
  NList,
  NListItem,
  NProgress,
  NSkeleton,
  NSpace,
  NStep,
  NSteps,
  NTag
} from "naive-ui";
import type {SummonerInfoStore} from "@/queryMatch/types/store";
import {Crown, Planet} from "@vicons/tabler";

const {infoStore} = defineProps<{ infoStore: SummonerInfoStore }>();
const rankRender = [
  {title: "单双排位", value: infoStore.rankList[0]},
  {title: "灵活排位", value: infoStore.rankList[1]},
  {title: "云顶排位", value: infoStore.rankList[2]}
];
</script>

<style scoped>

</style>