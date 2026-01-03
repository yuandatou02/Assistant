/**
 * 配置设置类型接口
 * 定义了游戏自动化的各种配置选项，包括自动选择/禁用英雄、自动接受、主题设置等功能
 */
export interface ConfigSettingTypes {
    /**
     * 自动选择英雄配置
     * @property {string} championId - 英雄ID
     * @property {boolean} isAuto - 是否启用自动选择
     */
    autoPickChampion: { championId: string; isAuto: boolean };
    /**
     * 自动禁用英雄配置
     * @property {string} championId - 英雄ID
     * @property {boolean} isAuto - 是否启用自动禁用
     */
    autoBanChampion: { championId: string; isAuto: boolean };
    /**
     * 自动模式开关
     * 控制是否启用一键自动模式
     */
    autoIsOne: boolean;
    /**
     * 自动接受延迟时间
     * 设置自动接受游戏的延迟时间（毫秒）
     */
    autoAccept: number;
    /**
     * 主题设置
     * 应用程序的主题样式
     */
    theme: string;
    /**
     * 游戏窗口模式
     * 控制游戏是否在独立窗口中运行
     */
    isGameInWindow: boolean;
    /**
     * 游戏提示开关
     * 控制是否显示游戏相关提示信息
     */
    isGameInTips: boolean;
    /**
     * 自动填写屏蔽词
     * 控制是否自动填写屏蔽词功能
     */
    autoWriteBlock: boolean;
    /**
     * 窗口不透明度
     * 设置应用程序窗口的不透明度值
     */
    inWinOpacity: number;
    /**
     * 温馨提示配置
     * 包含各种提示功能的开关设置
     */
    warmTips: {
        /**
         * 自动符文提示
         * 控制是否显示自动符文推荐提示
         */
        autoRune: boolean,
        /**
         * 排位提示
         * 控制是否显示排位相关提示信息
         */
        rankTips: boolean,
        /**
         * 队伍提示
         * 控制是否显示队伍相关提示信息
         */
        teamTips: boolean,
    };
}


/**
 * 配置等级接口
 * 定义了等级配置的基本结构，包含段位、分路和是否为101模式等属性
 */
export interface ConfigRank {
    tier: number;
    lane: string;
    is101: boolean;
}
