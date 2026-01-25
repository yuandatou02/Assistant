<template>
  <div class="main bg-neutral-100 dark:bg-neutral-900">
    <dashboard/>
    <router-view v-slot="{Component}">
      <keep-alive>
        <Component :is="Component"/>
      </keep-alive>
    </router-view>
    <navigation/>
  </div>
</template>

<script setup lang="ts">
import dashboard from "@/main/components/dashboard.vue";
import Navigation from "@/main/components/navigation.vue";
</script>

<style scoped>

</style>