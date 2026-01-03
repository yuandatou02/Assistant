import type {ConfigRank, ConfigSettingTypes} from "@/background/types";

const configSetting: ConfigSettingTypes = {
    autoPickChampion: {
        championId: "157",
        isAuto: false,
    },
    autoBanChampion: {
        championId: "101",
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
        teamTips: false,
    },
};


const configRank: ConfigRank = {
    tier: 200,
    lane: "mid",
    is101: true,
};

const addConfig = (configName: string, configObj: any) => {
    const localS = JSON.parse(localStorage.getItem(configName) || "{}");
    if (Object.keys(localS).length === Object.keys(configObj).length) {
        return;
    }

    for (const configKey of Object.keys(configObj)) {
        if (!localS.hasOwnProperty(configKey)) {
            // @ts-ignore
            localS[configKey] = configObj[configKey];
            localStorage.setItem(configName, JSON.stringify(localS));
        }
    }
};


export const configInit = () => {
    if (localStorage.getItem("configSetting") === null) {
        localStorage.setItem("configSetting", JSON.stringify(configSetting));
        localStorage.setItem("configRank", JSON.stringify(configRank));
    } else {
        addConfig("configSetting", configSetting);
        addConfig("configRank", configRank);
    }
};