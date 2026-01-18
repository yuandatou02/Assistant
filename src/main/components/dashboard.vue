<template>
  <header class="flex justify-between items-center h-8 mb-2 relative">
    <div data-tauri-drag-region class="dragDiv"/>
    <div class="flex items-center">
      <img src="@/assets/icon/app-icon.png" alt="app" class="h-8" draggable="false">
      <img src="@/assets/icon/Frank.png" alt="frank" class="pl-1 h-6.25">
    </div>
    <div class="flex mt-0.5 gap-x-2">
      <!--提示按钮-->
      <n-button :focusable="false" text @click.prevent="showDialog" v-if="isShowNoticeIcon">
        <n-icon size="20" :color="'#f0a020'">
          <bulb/>
        </n-icon>
      </n-button>
      <!--最小化按钮-->
      <n-button :focusable="false" text @click.prevent="handleMinimize">
        <n-icon size="20">
          <circle-minus/>
        </n-icon>
      </n-button>
      <!--设置按钮-->
      <n-button :focusable="false" text circle @click.prevent="isShowDrawer=true">
        <n-icon size="20">
          <settings/>
        </n-icon>
      </n-button>
      <n-popconfirm @positive-click="handleExit">
        <template #trigger>
          <n-button text circle>
            <n-icon size="20">
              <circle-x/>
            </n-icon>
          </n-button>
        </template>
        是否退出英雄联盟助手?
      </n-popconfirm>
    </div>
  </header>
  <!--设置抽屉-->
  <n-drawer class="rounded-t-lg!" v-model:show="isShowDrawer" :placement="'bottom'" :auto-focus="false" height="544">
    <setting/>
  </n-drawer>
</template>

<script setup lang="ts">
import {NButton, NDrawer, NIcon, NPopconfirm} from "naive-ui";
import {Bulb, CircleMinus, Settings, CircleX} from "@vicons/tabler";
import {Notice} from "@/main/utils/notice.ts";
import {onMounted, ref} from "vue";
import {getCurrentWindow} from "@tauri-apps/api/window";
import Setting from "@/main/components/setting.vue";
import {exit} from "@tauri-apps/plugin-process";

const isShowNoticeIcon = ref(false);
const isShowDrawer = ref(false);
const notice = new Notice();
onMounted(() => {
  notice.init().then((v) => {
    if (v) {
      isShowNoticeIcon.value = true;
    }
  });
});
const showDialog = () => {
  notice.showDialog();
};
// 最小化方法
const handleMinimize = async () => {
  await getCurrentWindow().minimize();
};

// 退出方法
const handleExit = async () => {
  await exit();
};
</script>