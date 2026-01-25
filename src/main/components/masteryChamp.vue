<template>
  <n-scrollbar v-if="existChampList.length===0" :style="styleSco">
    <n-space vertical :size="[0,15]" class="mt-0.75">
      <div class="flex" v-for="_ in 6">
        <n-skeleton height="50px" width="50px" :sharp="false" class="mr-2"/>
        <div class="grow flex flex-col justify-between">
          <n-skeleton height="22px" width="100%" :sharp="false"/>
          <n-skeleton height="22px" width="100%" :sharp="false"/>
        </div>
      </div>
    </n-space>
  </n-scrollbar>
  <n-scrollbar id="image-scroll-container" v-else-if="existChampList.length !== 0" :style="styleSco">
    <div>
      <n-space vertical :size="[0,15]" class="mt-0.75">
        <div class="flex h-12.5" v-for="champ in existChampList">
          <div class="w-12.5 mr-2">
            <div class="flex gap-x-3">
              <div class="flex w-12.5 h-12.5 items-center justify-center rounded bg-blue-100 dark:bg-[#70c0e850]">
                <n-avatar :si="42" :src="champ[0]" lazy :render-placeholder="()=>null" :intersection-observer-options="{
                  root: '#image-scroll-container',
                }"
                          fallback-src="https://wegame.gtimg.com/g.26-r.c2d3c/helper/lol/assis/images/resources/usericon/4027.png"
                          style="display: block;"/>
              </div>
            </div>
          </div>
          <div class="grow">
            <div class="flex flex-col justify-between h-full">
              <n-tag class="justify-center" size="small" :bordered="false" type="success">{{ champ[1] }}</n-tag>
              <n-tag class="justify-center" size="small" :bordered="false" type="info">{{ champ[2] }}</n-tag>
            </div>
          </div>
        </div>
      </n-space>
    </div>
  </n-scrollbar>
  <n-result v-else style="margin-top: 80px"
            status="418" title="数据获取失败" description="生活总归带点荒谬">
  </n-result>
</template>

<script setup lang="ts">
import {NAvatar, NResult, NScrollbar, NSkeleton, NSpace, NTag} from "naive-ui";

const {existChampList, maxH} = defineProps<
    { existChampList: string[][], maxH: number }
>();
const styleSco = `max-height:${maxH}px;padding-right: 13px`;
</script>

<style scoped>

</style>