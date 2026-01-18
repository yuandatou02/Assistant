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
    </div>
  </header>
</template>

<script setup lang="ts">
import {NButton, NIcon} from "naive-ui";
import {Bulb} from "@vicons/tabler";
import {Notice} from "@/main/utils/notice.ts";
import {onMounted, ref} from "vue";

const isShowNoticeIcon = ref(false);
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
</script>