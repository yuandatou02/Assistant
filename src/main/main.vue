<template>
  <n-config-provider :locale="zhCN" :theme-overrides="themeOverrides" :theme="theme === 'dark'? darkTheme:null">
    <n-dialog-provider>
      <n-message-provider :placement="'bottom'">
        <assistant/>
      </n-message-provider>
    </n-dialog-provider>
  </n-config-provider>
</template>

<script lang="ts" setup>
import {darkTheme, NConfigProvider, NDialogProvider, NMessageProvider, zhCN} from "naive-ui";
import {themeOverrides} from "@/main/utils/theme.ts";
import assistant from "@/main/index.vue";

const theme = localStorage.getItem("theme") || "light";
if (theme === "dark") {
  document.documentElement.classList.add("dark");
}
</script>
