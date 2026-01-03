import {createMainWindows} from "./utils/createWindows.ts";
import {configInit} from "@/background/utils/config.ts";

class Background {
    constructor() {
        configInit();
    }

    async init() {
        await createMainWindows();
    }
}

await new Background().init();