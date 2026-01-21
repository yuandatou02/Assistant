<template>
  <n-card :bordered="false" size="small" role="dialog" aria-modal="true">
    <n-alert title="本月多次使用英雄联盟助手完成对局" v-if="isCompleted" type="info">
      {{ sentence }}
      <br/>[此提示每月只会弹出一次]
    </n-alert>
    <n-alert v-else title="感谢您的支持❤️" type="success">
      {{ sentence }}
    </n-alert>
    <n-collapse class="mt-4">
      <n-collapse-item title="微信" name="1">
        <div class="flex flex-col items-center gap-1 justify-center">
          <div class="w-37.5 h-37.5">
            <img :src="waChat" class="w-full" alt="微信支付二维码">
          </div>
          <n-tag class="text-[13px]!" checkable disabled>若需要回复, 请在付款备注中, 留下您的邮箱</n-tag>
        </div>
      </n-collapse-item>
      <n-collapse-item title="支付宝" name="2">
        <div class="flex flex-col items-center gap-1 justify-center">
          <div class="w-37.5 h-37.5">
            <img :src="Alipay" class="w-full" alt="支付包二维码">
          </div>
          <n-tag class="text-[13px]!" checkable disabled>若需要回复, 请在付款备注中, 留下您的邮箱</n-tag>
        </div>
      </n-collapse-item>
    </n-collapse>
  </n-card>
</template>

<script setup lang="ts">
import {NAlert, NCard, NCollapse, NCollapseItem} from "naive-ui";

defineProps<{ isCompleted: boolean }>();

const sentence = "如果您觉得软件的内容有帮助，请考虑提供一点小小的赞助！您的支持，就是我追求完美代码的续航电源！！！ 😊";
const waChat = new URL("/src/assets/image/Wechat.JPEG", import.meta.url).href;
const Alipay = new URL("/src/assets/image/Alipay.JPEG", import.meta.url).href;
</script>

<style scoped>

</style>