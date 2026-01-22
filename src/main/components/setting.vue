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
      <!--   切换主题   -->
      <n-list-item>
        <div class="flex gap-x-5 justify-between items-center">
          <n-tag :bordered="false">主题样式</n-tag>
          <div class="flex grow justify-between">
            <n-radio :checked="theme ==='light'" value="light" name="basic-demo"
                     @click.prevent="handleThemeChange('light')">白羽清风
            </n-radio>
            <n-radio :checked="theme ==='dark'" value="dark" name="basic-demo"
                     @click.prevent="handleThemeChange('dark')">幽黑星空
            </n-radio>
          </div>
        </div>
      </n-list-item>
      <!--  秒选英雄  -->
      <n-list-item>
        <div class="gap-x-5 flex justify-between">
          <n-tag :bordered="false">秒选英雄</n-tag>
          <div class="flex grow items-center justify-between">
            <n-select filterable spellcheck="false" size="small" placeholder="选择英雄" class="w-31.5!"
                      v-model:value="config.autoPickChampion.championId"
                      :options="optionsChampion"
                      :disabled="!config.autoPickChampion.isAuto"
                      :filter="searchChampion"
                      @update:value="updateConfig"/>
            <n-switch v-model:value="config.autoPickChampion.isAuto" @click.prevent="updateConfig"/>
          </div>
        </div>
      </n-list-item>
      <!--    秒禁英雄  -->
      <n-list-item>
        <div class="flex gap-x-5 justify-between">
          <n-tag :bordered="false">秒禁英雄</n-tag>
          <div class="flex grow items-center justify-between">
            <n-select v-model:value="config.autoBanChampion.championId" filterable
                      spellcheck="false" size="small" :filter="searchChampion" :options="optionsChampion"
                      :disabled="!config.autoBanChampion.isAuto"
                      @update:value="updateConfig" placeholder="选择英雄" class="w-31.5!"/>
            <n-switch v-model:value="config.autoBanChampion.isAuto" @click.prevent="updateConfig"/>
          </div>
        </div>
      </n-list-item>
      <!--     秒选/秒禁英雄 是否使用一次关闭  -->
      <n-list-item>
        <div class="gap-x-5 flex justify-between">
          <n-tag :bordered="false">昙花一现</n-tag>
          <div class="flex grow items-center justify-between">
            <n-tag :disabled="!config.autoIsOne" :type="config.autoIsOne ? 'success' : 'default'">
              使用一次后会禁用
            </n-tag>
            <n-switch v-model:value="config.autoIsOne" @click.prevent="updateConfig" class="mt-0!"/>
          </div>
        </div>
        <n-tag class="mt-1.5 w-full justify-center" :disabled="true" :bordered="false" size="small">
          秒选/秒禁英雄 功能使用一次后关闭
        </n-tag>
      </n-list-item>
      <!-- 游戏窗口-->
      <n-list-item>
        <div class="gap-x-5 flex justify-between">
          <n-tag :bordered="false">游戏窗口</n-tag>
          <div class="flex grow items-center justify-between">
            <n-tag :type="config.isGameInWindow ? 'success' : 'default'" :disabled="!config.isGameInWindow">
              自动打开游戏窗口
            </n-tag>
            <n-switch v-model:value="config.isGameInWindow" @click.prevent="updateConfig"/>
          </div>
        </div>
        <n-tag class="mt-1.5 w-full justify-center" :disabled="true" :bordered="false" size="small">
          游戏内显示战绩窗口，显示|隐藏 SHIFT+TAB
        </n-tag>
        <n-tag class="mt-1.5 w-full justify-center" :disabled="!config.isGameInWindow" :bordered="false"
               size="small">
          关闭自动打开后，进入游戏需点击右下角图标
        </n-tag>
      </n-list-item>
      <!-- 秒接对局-->
      <n-list-item>
        <div class="gap-x-5 flex justify-between items-center">
          <n-tag :bordered="false">秒接对局</n-tag>
          <n-slider v-model:value="config.autoAccept" :step="10" @update:value="updateConfig"/>
        </div>
        <n-tag class="mt-1.5 w-full justify-center" :disabled="true" :bordered="false" size="small">
          数值: [ {{ "<" }}50 关闭 ] [ =50 开启 ] [ {{ "=" }}60 延迟两秒 ]
        </n-tag>
      </n-list-item>
      <!--      软件信息   -->
      <n-list-item class="pb-0!">
        <div class="flex justify-between items-center">
          <n-button size="small" secondary type="tertiary" @click.prevent="openWeb(false)">
            版本 {{ version }}
          </n-button>
          <n-button size="small" secondary type="tertiary" @click.prevent="openWeb(true)">
            By Java_S
          </n-button>
          <n-button size="small" secondary type="tertiary" @click.prevent="restart">
            重启
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
import {
  NButton,
  NDrawerContent,
  NList,
  NListItem,
  NModal,
  NRadio,
  NSelect,
  NSlider,
  NSwitch,
  NTag,
  useDialog
} from "naive-ui";
import Sponsor from "@/main/components/sponsor.vue";
import {type Ref, ref} from "vue";
import {relaunch} from "@tauri-apps/plugin-process";
import type {ConfigSettingTypes} from "@/background/types";
import {keywordsList, optionsChampion} from "@/resources/champList.ts";

declare const __APP_VERSION__: string;
const version = __APP_VERSION__;
const dialog = useDialog();
const sponsor = ref(false);
const theme = localStorage.getItem("theme") || "light";
const config: Ref<ConfigSettingTypes> = ref(JSON.parse(localStorage.getItem("configSetting") as string));
// 切换主题方法
const handleThemeChange = (value: string) => {
  if (theme !== value) {
    dialog.warning({
      title: "提示",
      content: "主题切换将重启应用, 是否执行操作o.O?",
      showIcon: true,
      positiveText: "确定",
      negativeText: "取消",
      maskClosable: true,
      closable: false,
      autoFocus: false,
      style: "margin:8px;max-width:334px",
      onPositiveClick: async () => {
        theme === "light" ? localStorage.setItem("theme", "dark") : localStorage.setItem("theme", "light");
        await relaunch();
      },
    });
  }

};

// 搜索英雄
const searchChampion = (pattern: string, options: object): boolean => {
  if (pattern === "" || pattern === null) return false;
  const keyword = pattern.toLowerCase();
  const renderList = keywordsList.filter(item => item.keywords.toLowerCase().includes(keyword));
  if (renderList.length > 5 || renderList.length === 0) return false;
  for (const renderListItem of renderList) {
    if (renderListItem.name === options.label) {
      return true;
    }
  }
  return false;
};

// 更新配置
const updateConfig = () => {
  localStorage.setItem("configSetting", JSON.stringify(config.value));
};

const openWeb = (isSYJ: boolean) => {
  if (isSYJ) {
    open("https://syjun.vip");
  } else {
    open("https://www.yuque.com/java-s/frank/introduction");
  }
};

const restart = async () => {
  await relaunch();
};
</script>

<style scoped>

</style>