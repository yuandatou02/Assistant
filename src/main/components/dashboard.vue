<template>
  <header class="flex justify-between items-center h-8 mb-2 relative">
    <div data-tauri-drag-region class="dragDiv"/>
    <div class="flex items-center">
      <img src="@/assets/icon/app-icon.png" class="h-8" alt="app-icon">
      <img src="@/assets/icon/Frank.png" draggable="false" alt="frank" class="pl-1 h-6.25">
    </div>
    <div class="flex mt-0.5 gap-x-2">
      <!--提示图标-->
      <n-button v-if="isShowNoticeIcon" :focusable="false" text>
        <n-icon size="20" :color="'#f0a020'">
          <bulb/>
        </n-icon>
      </n-button>
      <!--   最小化   -->
      <n-button :focusable="false" @click.prevent="handleMin" text>
        <n-icon size="20">
          <circle-minus/>
        </n-icon>
      </n-button>
      <!--   设置   -->
      <n-button :focusable="false" text circle @click.prevent="isShowDrawer=true">
        <n-icon size="20">
          <settings/>
        </n-icon>
      </n-button>
      <!--   退出   -->
      <n-popconfirm
          @positive-click="handleClose" :show-icon="true">
        <template #trigger>
          <n-button text circle>
            <n-icon size="20">
              <circle-x/>
            </n-icon>
          </n-button>
        </template>
        是否退出 Frank?
      </n-popconfirm>
    </div>
  </header>
  <!--设置抽屉-->
  <n-drawer
      style="border-top-left-radius: 0.5rem;border-top-right-radius: 0.5rem"
      v-model:show="isShowDrawer"
      :placement="'bottom'"
      :auto-focus="false"
      height="544"
  >
    <setting/>
  </n-drawer>
</template>

<script setup lang="ts">
import {NButton, NDrawer, NIcon, NPopconfirm} from "naive-ui";
import {Bulb, CircleMinus, Settings, CircleX} from "@vicons/tabler";
import {getCurrentWindow} from "@tauri-apps/api/window";
import {ref} from "vue";
import {exit} from "@tauri-apps/plugin-process";
import Setting from "@/main/components/setting.vue";

const isShowNoticeIcon = ref(false);
const isShowDrawer = ref(false);
// 最小化窗口
const handleMin = async () => {
  await getCurrentWindow().minimize();
};
// 关闭窗口
const handleClose = async () => {
  await exit();
};
</script>