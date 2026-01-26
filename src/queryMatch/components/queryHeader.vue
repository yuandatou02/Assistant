<template>
  <header class="flex">
    <div class="flex gap-x-2 items-center mr-3">
      <img src="@/assets/icon/app-icon.png" alt="app-icon" class="h-10" draggable="false">
      <img src="@/assets/icon/Frank.png" alt="Frank" draggable="false">
      <n-button v-if="false" size="small" style="margin-left: 30px; color: #666666; width: 90.41px" secondary
                type="tertiary">
        lolfrank.cn
      </n-button>
      <n-button v-else size="small" style="margin-left: 30px; width: 90.41px" secondary type="info">
        Back Self
      </n-button>
    </div>
    <div class="grow flex items-center gap-x-3">
      <n-button size="small" secondary type="tertiary" :bordered="false" class="w-35.25 text-[#666666] text-[13.5px]">
        仅显示玩家战绩数据
      </n-button>
      <n-button size="small" :bordered="false" type="success" class="w-11.5 py-0 px-2.25">
        刷新
      </n-button>
      <n-select size="small" v-model:value="selectValue" :options="options" class="w-25! ml-7!"/>
      <n-pagination v-model:page="pageValue" :page-slot="10" :page-count="10"/>
    </div>
    <n-space class="pt-2.5!" :size="[8,0]">
      <n-button text @click.prevent="handleMinimize">
        <n-icon size="20">
          <circle-minus/>
        </n-icon>
      </n-button>
      <n-button text circle @click.prevent="handleSettings">
        <n-icon size="20">
          <settings/>
        </n-icon>
      </n-button>
      <n-button text circle @click.prevent="handleClose">
        <n-icon size="20">
          <circle-x/>
        </n-icon>
      </n-button>
    </n-space>
  </header>
</template>

<script setup lang="ts">
import {NButton, NIcon, NPagination, NSelect, NSpace, useMessage} from "naive-ui";
import {CircleMinus, CircleX, Settings} from "@vicons/tabler";
import {ref} from "vue";
import {getCurrentWindow} from "@tauri-apps/api/window";

const selectValue = ref(0);
const pageValue = ref(1);
const message = useMessage();
const options = [
  {
    label: "全部模式",
    value: 0,
  },
  {
    label: "单双排位",
    value: 420,
  },
  {
    label: "灵活排位",
    value: 440,
  },
  {
    label: "匹配模式",
    value: 430,
  },
  {
    label: "极地乱斗",
    value: 450,
  },
  {
    label: "斗魂竞技",
    value: 1700,
  },
];


const handleMinimize = async () => {
  await getCurrentWindow().minimize();
};
const handleClose = async () => {
  await getCurrentWindow().close();
};
const handleSettings = () => {
  message.info("功能开发中...");
};
</script>

<style scoped>

</style>