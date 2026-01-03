<template>
  <n-config-provider :locale="zhCN" :theme="theme === 'dark' ? darkTheme : null "
                     :theme-overrides="themeOverrides">
    <n-dialog-provider>
      <n-message-provider :placement="'bottom'">
        <assistant/>
      </n-message-provider>
    </n-dialog-provider>
  </n-config-provider>
</template>

<script setup lang="ts">
import {darkTheme, NConfigProvider, NDialogProvider, NMessageProvider, zhCN} from "naive-ui";
import {themeOverrides} from "./utils/theme.ts";
import assistant from "./index.vue";

const theme = localStorage.getItem("theme") || "light";
</script>

<style scoped>

</style>