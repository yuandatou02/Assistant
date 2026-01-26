import {queryRankPoint, querySummonerInfo} from "@/main/api/aboutSummoner.ts";
import type {SummonerInfoStore} from "@/queryMatch/types/store";

export class BaseMatch {
    /**
     * 获取召唤师信息存储对象
     * @param summonerId - 召唤师ID，可选参数，默认为undefined
     * @returns 返回包含召唤师基本信息和排名信息的SummonerInfoStore对象，如果查询失败则返回null
     */
    public getSummonerInfoStore = async (summonerId?: number): Promise<SummonerInfoStore | null> => {
        // 查询召唤师基础信息
        const summonerInfo = await querySummonerInfo(summonerId);
        if (summonerInfo !== null) {
            // 根据puuid查询排名积分信息
            const rankList = await queryRankPoint(summonerInfo.puuid);
            return {
                info: summonerInfo,
                rankList: rankList
            };
        }
        return null;
    };
}