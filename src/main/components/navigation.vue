<template>
  <div class="navDiv bg-white shadow darkBoxBoard dark:bg-zinc-900">
    <div class="flex w-full justify-between mx-2">
      <n-icon class="transition cursor-pointer dark:text-slate-100" v-for="item in icons" :key="item.index" size="26">
        <component :is="item.icon"/>
      </n-icon>
    </div>
  </div>
</template>

<script setup lang="ts">
import {SmartHome, Ghost, List, Notebook, Stack2} from "@vicons/tabler";
import {NIcon} from "naive-ui";

const icons = [
  {icon: SmartHome, index: 0, route: "home"},
  {icon: List, index: 1, route: "rank"},
  {icon: Ghost, index: 2, route: "teammate"},
  {icon: Stack2, index: 3, route: "rune"},
  {icon: Notebook, index: 4, route: "record"},
];
</script>

<style scoped>

</style>