import type {ConfigRank, ConfigSettingTypes} from "@/background/types";

// 默认配置
const configSetting: ConfigSettingTypes = {
    autoPickChampion: {
        championId: 157,
        isAuto: false,
    },
    autoBanChampion: {
        championId: 101,
        isAuto: false,
    },
    autoIsOne: true,
    autoAccept: 50,
    theme: "light",
    isGameInWindow: true,
    isGameInTips: false,
    autoWriteBlock: true,
    inWinOpacity: 100,
    warmTips: {
        autoRune: false,
        rankTips: false,
        teamTips: false
    }
};
// 默认排名
const configRank: ConfigRank = {
    tier: 200,
    lane: "mid",
    is101: true
};

/**
 * 向本地存储中的配置对象添加新的配置项
 * 如果本地存储中已存在相同长度的配置对象，则不进行任何操作
 * 只有当配置对象中有本地存储中不存在的键时，才会将新键值对添加到本地存储中
 *
 * @param configName - 配置名称，用于在localStorage中标识配置对象
 * @param configObj - 要添加的配置对象，包含需要合并的配置项
 */
const addConfig = (configName: string, configObj: any) => {
    // 从本地存储中获取现有配置对象
    const localS = JSON.parse(localStorage.getItem(configName) as string);

    // 检查本地存储中的配置对象与传入配置对象的键数量是否相等
    if (Object.keys(localS).length === Object.keys(configObj).length) {
        return;
    }

    // 遍历传入的配置对象的所有键
    for (const configKey of Object.keys(configObj)) {
        // 如果本地存储中的配置对象不包含当前键，则添加该键值对
        if (!localS.hasOwnProperty(configKey)) {
            localS[configKey] = configObj[configKey];
            localStorage.setItem(configName, JSON.stringify(localS));
        }
    }
};


/**
 * 初始化配置数据
 * 检查本地存储中是否存在配置设置，如果不存在则创建默认配置，
 * 如果存在则将现有配置与默认配置进行合并
 *
 * @returns void
 */
export const configInit = () => {
    // 检查本地存储中是否已存在配置设置
    if (localStorage.getItem("configSetting") === null) {
        // 如果不存在配置，则保存默认配置到本地存储
        localStorage.setItem("configSetting", JSON.stringify(configSetting));
        localStorage.setItem("configRank", JSON.stringify(configRank));
    } else {
        // 如果已存在配置，则将现有配置与默认配置合并
        addConfig("configSetting", configSetting);
        addConfig("configRank", configRank);
    }
};
