/**
 * 召唤师信息接口
 * 定义了召唤师的基本信息结构，包括名称、隐私设置、头像地址、等级、经验值等属性
 */
export interface SummonerInfo {
    /** 召唤师名称 */
    name: string,
    /** 隐私设置 */
    privacy: string,
    /** 头像图片地址 */
    imgUrl: string
    /** 等级（字符串或数字类型） */
    lv: string | number,
    /** 经验值 */
    xp: number,
    /** PUUID标识符 */
    puuid: string,
    /** 当前ID */
    currentId: string,
    /** 标签行（可选） */
    tagLine: string | undefined
}


/**
 * 召唤师数据接口
 * 定义了召唤师信息的数据结构，包含完整的召唤师相关信息
 */
export interface SummonerData {
    summonerInfo: SummonerInfo | null;
    rankList: string[] | null;
    champLevel: string[][] | null;
}
