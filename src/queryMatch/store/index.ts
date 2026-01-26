import {defineStore} from "pinia";
import type {MatchStore} from "@/queryMatch/types/store";
import {BaseMatch} from "@/queryMatch/utils/baseMatch.ts";

const baseMatch = new BaseMatch();

const useMatchStore = defineStore("useMatchStore", {
    state: (): MatchStore => ({
        summonerId: -1,
        localSummonerId: -1,
        summonerInfoStore: null
    }),
    actions: {
        async init(summonerId?: number, localSummonerId?: number) {
            const summonerInfoStore = await baseMatch.getSummonerInfoStore(summonerId);
            if (summonerInfoStore === null) return;
            if (summonerId === undefined && localSummonerId === undefined) {
                this.localSummonerId = summonerInfoStore.info.currentId;
            } else if (localSummonerId !== undefined) {
                this.localSummonerId = localSummonerId;
            }
            this.summonerInfoStore = summonerInfoStore;
        }
    }
});


export default useMatchStore;