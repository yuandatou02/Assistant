<template>
  <div class="flex flex-col w-63.5">
    <n-card size="small" class="shadow!" content-style="padding-bottom:0">
      <!--头像 昵称 等级-->
      <div class="h-14 flex gap-x-2">
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
    </n-card>
  </div>
</template>

<script setup lang="ts">
import {NAvatar, NCard, NEllipsis, NProgress, NSpace, NTag} from "naive-ui";
import type {SummonerInfoStore} from "@/queryMatch/types/store";

const {infoStore} = defineProps<{ infoStore: SummonerInfoStore }>();
</script>

<style scoped>

</style>