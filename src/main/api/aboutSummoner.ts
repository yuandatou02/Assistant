// 通过LCU API 获取召唤师信息
import {invoke} from "@tauri-apps/api/core";
import type {SummonerInfo} from "@/main/types/SummonerTypes";

export const querySummonerInfo = async (summonerId?: string) => {
    const endpoint = summonerId ? `/lol-summoner/v1/summoners/${summonerId}` : "/lol-summoner/v1/current-summoner";
    return await invoke<SummonerInfo>("get_summoner_info", {endpoint});
};

export const queryRankPoint = async (puuid?: string) => {
    // 根据是否提供 puuid 调用不同的接口
    const endpoint = puuid ? `/lol-ranked/v1/ranked-stats/${puuid}` : "/lol-ranked/v1/current-ranked-stats";
    return await invoke<string[]>("get_rank_point", {endpoint});
};

export const querySummonerHonor = async () => {
    return await invoke<string>("get_summoner_honor");
};

export const queryMasteryChampList = async (summonerPuuid?: string) => {
    const endpoint = summonerPuuid ? "/lol-champion-mastery/v1/local-player/champion-mastery" : `/lol-champion-mastery/v1/${summonerPuuid}/champion-mastery`;
    return await invoke<string[][]>("get_mastery_champ_list", {endpoint});
};