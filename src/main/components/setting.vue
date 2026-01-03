<template>
  <n-drawer-content
      body-style="padding:20px 22px"
      body-content-style="padding:0px"
  >
    <n-list>
      <n-list-item style="padding-top: 0">
        <div class="gap-x-5 flex justify-between items-center">
          <n-tag :bordered="false">鼓励开发</n-tag>
          <n-button
              @click.prevent="showModal = true"
              style="width: 186px"
              size="small"
              secondary
              :bordered="false"
              type="warning"
          >
            赞助 Frank 英雄联盟助手
          </n-button>
        </div>
      </n-list-item>
      <!--        切换主题-->
      <n-list-item>
        <div class="flex gap-x-5 justify-between items-center">
          <n-tag :bordered="false">主题样式</n-tag>
          <div class="flex grow justify-between">
            <n-radio
                :checked="theme === 'light'"
                value="light"
                name="basic-demo"
                @click="handleThemeChange"
            >
              白羽清风
            </n-radio>
            <n-radio
                :checked="theme === 'dark'"
                value="dark"
                name="basic-demo"
                @click="handleThemeChange"
            >
              幽黑星空
            </n-radio>
          </div>
        </div>
      </n-list-item>
      <!--        秒选英雄-->
      <n-list-item>
        <div class="gap-x-5 flex justify-between">
          <n-tag :bordered="false">秒选英雄</n-tag>
          <div class="flex grow items-center justify-between">
            <n-select
                v-model:value="config.autoPickChampion.championId"
                filterable
                spellcheck="false"
                size="small"
                :filter="searchChamp"
                :options="optionsChampion"
                :disabled="!config.autoPickChampion.isAuto"
                @update:value="saveConfig"
                placeholder="选择英雄"
                style="width: 126px"
            />
            <n-switch
                v-model:value="config.autoPickChampion.isAuto"
                @click="saveConfig"
            />
          </div>
        </div>
      </n-list-item>
      <!--        秒禁英雄-->
      <n-list-item>
        <div class="flex gap-x-5 justify-between">
          <n-tag :bordered="false">秒禁英雄</n-tag>
          <div class="flex grow items-center justify-between">
            <n-select
                v-model:value="config.autoBanChampion.championId"
                filterable
                spellcheck="false"
                size="small"
                :filter="searchChamp"
                :options="optionsChampion"
                :disabled="!config.autoBanChampion.isAuto"
                @update:value="saveConfig"
                placeholder="选择英雄"
                style="width: 126px"
            />
            <n-switch
                v-model:value="config.autoBanChampion.isAuto"
                @click="saveConfig"
            />
          </div>
        </div>
      </n-list-item>
      <!--        秒选/秒禁英雄 是否使用一次关闭-->
      <n-list-item>
        <div class="gap-x-5 flex justify-between">
          <n-tag :bordered="false">昙花一现</n-tag>
          <div class="flex grow items-center justify-between">
            <n-tag
                :disabled="!config.autoIsOne"
                :type="config.autoIsOne ? 'success' : 'default'"
            >
              使用一次后会禁用
            </n-tag
            >
            <n-switch
                v-model:value="config.autoIsOne"
                @click="saveConfig"
                style="margin-top: 0"
            />
          </div>
        </div>
        <n-tag
            class="mt-1.5 w-full justify-center"
            :disabled="true"
            :bordered="false"
            size="small"
        >
          秒选/秒禁英雄 功能使用一次后关闭
        </n-tag
        >
      </n-list-item>
      <!--        游戏窗口-->
      <n-list-item>
        <div class="gap-x-5 flex justify-between">
          <n-tag :bordered="false">游戏窗口</n-tag>
          <div class="flex grow items-center justify-between">
            <n-tag
                :type="
                                config.isGameInWindow ? 'success' : 'default'
                            "
                :disabled="!config.isGameInWindow"
            >
              自动打开游戏窗口
            </n-tag
            >
            <n-switch
                v-model:value="config.isGameInWindow"
                @click="saveConfig"
            />
          </div>
        </div>
        <n-tag
            class="mt-1.5 w-full justify-center"
            :disabled="true"
            :bordered="false"
            size="small"
        >
          游戏内显示战绩窗口，显示|隐藏 SHIFT+TAB
        </n-tag
        >
        <n-tag
            class="mt-1.5 w-full justify-center"
            :disabled="config.isGameInWindow"
            :bordered="false"
            size="small"
        >
          关闭自动打开后，进入游戏需点击右下角图标
        </n-tag
        >
      </n-list-item>
      <!--        秒接对局-->
      <n-list-item>
        <div class="gap-x-5 flex justify-between items-center">
          <n-tag :bordered="false">秒接对局</n-tag>
          <n-slider
              v-model:value="config.autoAccept"
              :step="10"
              @update:value="saveConfig"
          />
        </div>
        <n-tag
            class="mt-1.5 w-full justify-center"
            :disabled="true"
            :bordered="false"
            size="small"
        >数值: [ {{ "<" }}50 关闭 ] [ =50 开启 ] [ {{ "=" }}60
          延迟两秒 ]
        </n-tag
        >
      </n-list-item>
      <!--        秒接对局-->
      <n-list-item style="padding-bottom: 0">
        <div class="flex justify-between items-center">
          <n-button
              size="small"
              secondary
              type="tertiary"
              @click="openWeb(false)"
          >
            版本 {{ version }}
          </n-button>
          <n-button
              size="small"
              secondary
              type="tertiary"
              @click="openWeb(true)"
          >
            By Java_S
          </n-button>
          <n-button
              size="small"
              secondary
              type="tertiary"
              @click="restart"
          >
            重启
          </n-button>
        </div>
      </n-list-item>
    </n-list>
    <n-modal style="margin: 8px; max-width: 334px" v-model:show="showModal">
      <Sponsor :is-completed="false"/>
    </n-modal>
  </n-drawer-content>
</template>

<script setup lang="ts">
import {
  NDrawerContent,
  NModal,
  NTag,
  NButton,
  NSelect,
  NSwitch,
  NSlider,
  NRadio,
  NList,
  NListItem,
  useDialog, type SelectFilter, type SelectOption,
} from "naive-ui";
import {type Ref, ref} from "vue";
import {relaunch} from "@tauri-apps/plugin-process";
import Sponsor from "@/main/components/sponsor.vue";
import type {ConfigSettingTypes} from "@/background/types";
import {keywordsList, optionsChampion} from "@/resources/champList.ts";

const showModal = ref(false);
const theme = localStorage.getItem("theme") || "light";
const dialog = useDialog();
const config: Ref<ConfigSettingTypes> = ref(
    JSON.parse(localStorage.getItem("configSetting") as string),
);
declare const __APP_VERSION__: string;
const version = __APP_VERSION__;
const saveConfig = () => {
  localStorage.setItem("configSetting", JSON.stringify(config.value));
};
// 切换主题
const handleThemeChange = () => {
  dialog.warning({
    title: "Tips",
    content: "主题切换将重启Frank, 是否执行操作o.O?",
    showIcon: true,
    positiveText: "确认",
    negativeText: "取消",
    maskClosable: true,
    closable: false,
    autoFocus: false,
    style: "margin:8px;max-width:334px",
    onPositiveClick: async () => {
      if (theme !== "dark") {
        localStorage.setItem("theme", "dark");
      } else {
        localStorage.setItem("theme", "light");
      }
      await relaunch();
    },
  });
};
// 搜索英雄
const searchChamp: SelectFilter = (pattern: string, option: SelectOption): boolean => {
  if (!pattern) return false;               // 空串或 null 直接返回
  const keyword = pattern.toLowerCase();

  // 如果想用 value 也可以，这里只比 label 当示例
  return keywordsList.some(
      (it) =>
          it.keywords.toLowerCase().includes(keyword) &&
          it.name === option.label,
  );
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
