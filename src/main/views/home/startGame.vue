<template>
  <n-card class="shadow!" size="small">
    <div class="float-left w-15 h-15 mr-3.75 mt-px">
      <div class="spinner"></div>
    </div>
    <n-space vertical>
      <n-space justify="space-between">
        <n-button type="success" secondary @click.prevent="openGuideSite">使用手册</n-button>
        <n-button type="success" @click.prevent="startGame">开始游戏</n-button>
      </n-space>
      <n-tag type="success" size="small" class="w-50.75! justify-center!" :bordered="false">
        <p>进入LOL大厅后•自动获取数据</p>
      </n-tag>
    </n-space>
  </n-card>
  <n-card class="shadow! mt-4!" size="small">
    <n-table :bordered="false" :single-line="false">
      <thead>
      <tr>
        <th>软件介绍</th>
        <th class="w-7">完全</th>
        <th class="w-7">免费</th>
      </tr>
      </thead>
      <tbody>
      <tr v-for="(row,index) in tableData" :key="index">
        <td>{{ row[0] }}</td>
        <td class="pl-4">{{ row[1] }}</td>
        <td class="pl-4">{{ row[2] }}</td>
      </tr>
      </tbody>
    </n-table>
    <n-space class="mt-5.25!" justify="space-between">
      <n-tag type="success" :bordered="false">软件部分功能介绍</n-tag>
      <n-button size="small" type="success" :bordered="false" @click.prevent="openInfo">全部功能</n-button>
    </n-space>
  </n-card>
</template>

<script setup lang="ts">
import {NButton, NCard, NSpace, NTable, NTag, useMessage} from "naive-ui";
import {open} from "@tauri-apps/plugin-shell";
import {invoke} from "@tauri-apps/api/core";

const message = useMessage();
const tableData = [
  ["排位笔记", "✅", "✅"],
  ["自动接收对局", "✅", "✅"],
  ["英雄符文展示", "✅", "✅"],
  ["一键配置符文", "✅", "✅"],
  ["自动配置符文", "✅", "✅"],
  ["对局结束后评分", "✅", "✅"],
  ["秒选英雄 / 秒禁英雄", "✅", "✅"],
  ["国服 / 韩服 英雄数据", "✅", "✅"],
];
// 跳转使用手册
const openGuideSite = () => {
  open("https://www.yuque.com/java-s/frank");
};

// 开始游戏
const startGame = async () => {
  // 获取英雄联盟路径
  const path = localStorage.getItem("clientPath");
  if (path !== null) {
    await invoke("start_game", {path});
    message.loading("英雄联盟客户端启动中...");
  } else {
    message.error("路径不存在，首次启动请先打开客户端", {duration: 4000});
  }
};
const openInfo = () => {
  open("https://www.yuque.com/java-s/frank/introduction");
};
</script>

<style scoped>
.spinner {
  --size: 30px;
  --first-block-clr: #FF8282;
  --second-block-clr: #FFDC80;
  --clr: #111;
  width: 60px;
  height: 60px;
  position: relative;
}

.spinner::after,
.spinner::before {
  box-sizing: border-box;
  position: absolute;
  content: "";
  width: var(--size);
  height: var(--size);
  top: 50%;
  animation: up 2.4s cubic-bezier(0, 0, 0.24, 1.21) infinite;
  left: 50%;
  background: var(--first-block-clr);
  border-radius: 50px;
}

.spinner::after {
  background: var(--second-block-clr);
  top: calc(50% - var(--size));
  left: calc(50% - var(--size));
  animation: down 2.4s cubic-bezier(0, 0, 0.24, 1.21) infinite;
}

@keyframes down {

  0%,
  100% {
    transform: none;
  }

  25% {
    transform: translateX(100%);
  }

  50% {
    transform: translateX(100%) translateY(100%);
  }

  75% {
    transform: translateY(100%);
  }
}

@keyframes up {

  0%,
  100% {
    transform: none;
  }

  25% {
    transform: translateX(-100%);
  }

  50% {
    transform: translateX(-100%) translateY(-100%);
  }

  75% {
    transform: translateY(-100%);
  }
}
</style>