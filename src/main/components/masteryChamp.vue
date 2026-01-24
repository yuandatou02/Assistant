<template>
  <n-scrollbar></n-scrollbar>
</template>

<script setup lang="ts">
import {NScrollbar} from "naive-ui";

const {puuid, existChampList, maxH} = defineProps<
    { puuid: string, existChampList: string[][] | undefined, maxH: number }
>();
</script>

<style scoped>

</style>