<template>
  <n-config-provider :locale="zhCN" :theme-overrides="themeOverrides" :theme="theme === 'dark'?darkTheme:null">
    <n-message-provider>
      <query-match/>
    </n-message-provider>
  </n-config-provider>
</template>

<script setup lang="ts">
import {darkTheme, NConfigProvider, NMessageProvider, zhCN} from "naive-ui";
import {themeOverrides} from "@/main/utils/theme.ts";
import QueryMatch from "@/queryMatch/index.vue";

const theme = localStorage.getItem("theme") || "light";
if (theme === "dark") {
  document.documentElement.classList.add("dark");
}
</script>

<style scoped>

</style>