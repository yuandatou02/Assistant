// 通过LCU API 获取召唤师信息
import {invoke} from "@tauri-apps/api/core";
import type {SummonerInfo} from "@/main/types/SummonerTypes";

export const querySummonerInfo = async (summonerId?: string) => {
    const endpoint = summonerId ? `/lol-summoner/v1/summoners/${summonerId}` : "/lol-summoner/v1/current-summoner";
    return await invoke<SummonerInfo | null>("get_summoner_info", {endpoint});
};