<template>
  <n-drawer-content body-style="padding:20px 22px" body-content-style="padding:0px">
    <n-list>
      <n-list-item style="padding-top: 0">
        <div class="gap-x-5 flex justify-between items-center">
          <!--      赞助区    -->
          <n-tag :bordered="false">鼓励开发</n-tag>
          <n-button class="w-46.5!" size="small" secondary :bordered="false" type="warning"
                    @click.prevent="sponsor=true">
            赞助英雄联盟助手
          </n-button>
        </div>
      </n-list-item>
    </n-list>
    <!--赞助弹窗-->
    <n-modal class="max-w-83.5! m-2!" v-model:show="sponsor">
      <sponsor :is-completed="false"/>
    </n-modal>
  </n-drawer-content>
</template>

<script setup lang="ts">
import {NButton, NDrawerContent, NList, NListItem, NModal, NTag} from "naive-ui";
import Sponsor from "@/main/components/sponsor.vue";
import {ref} from "vue";

const sponsor = ref(false);
</script>

<style scoped>

</style>