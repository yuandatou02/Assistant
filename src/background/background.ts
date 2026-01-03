import {createMainWindows} from "./utils/createWindows.ts";

class Background {
    constructor() {
    }

    async init() {
        await createMainWindows();
    }
}

await new Background().init();