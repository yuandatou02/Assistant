import type {SummonerInfo} from "@/main/types/SummonerTypes";

/**
 * 比赛存储接口
 * 定义了比赛中召唤师信息的数据结构
 */
export interface MatchStore {
    summonerId: number,
    localSummonerId: number,
    summonerInfoStore: SummonerInfoStore | null,
}

interface SummonerInfoStore {
    info: SummonerInfo,
    rankList: string[],
}