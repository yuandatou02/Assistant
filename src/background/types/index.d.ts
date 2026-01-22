/**
 * 自动选择英雄接口
 * 定义了英雄ID和是否自动选择的状态信息
 */
interface AutoChampion {
    championId: string;
    isAuto: boolean;
}


/**
 * 配置设置类型接口
 * 定义了应用程序的各种配置选项，包括自动选择/禁用英雄、游戏窗口设置、主题等
 */
export interface ConfigSettingTypes {
    /**
     * 自动选择英雄配置
     */
    autoPickChampion: AutoChampion;
    /**
     * 自动禁用英雄配置
     */
    autoBanChampion: AutoChampion;
    /**
     * 是否启用一键模式
     */
    autoIsOne: boolean;
    /**
     * 是否自动接受对局
     */
    autoAccept: number;
    /**
     * 应用主题设置
     */
    theme: string;
    /**
     * 游戏是否以窗口模式运行
     */
    isGameInWindow: boolean;
    /**
     * 是否显示游戏提示信息
     */
    isGameInTips: boolean;
    autoWriteBlock: boolean;
    /**
     * 窗口透明度设置（仅在窗口模式下有效）
     */
    inWinOpacity: number;
    /**
     * 温馨提示功能配置
     * 包含符文推荐、排位提示、队伍信息等功能的开关设置
     */
    warmTips: {
        /**
         * 是否启用自动符文推荐
         */
        autoRune: boolean,
        /**
         * 是否启用排位赛提示
         */
        rankTips: boolean,
        /**
         * 是否启用队伍信息提示
         */
        teamTips: boolean,
    };
}

/**
 * 配置排名信息接口
 * 定义了玩家在游戏中的段位、分路和是否为101模式的信息结构
 */
export interface ConfigRank {
    /**
     * 段位等级
     * 表示玩家当前的段位级别数值
     */
    tier: number;

    /**
     * 分路信息
     * 表示玩家在游戏中的位置或路线选择
     */
    lane: string;

    /**
     * 101模式标识
     * 布尔值，表示该配置是否适用于101特殊模式
     */
    is101: boolean;
}

