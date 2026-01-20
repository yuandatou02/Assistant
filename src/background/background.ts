import {createMainWindows} from "./utils/createWindow";
import {invoke} from "@tauri-apps/api/core";
import {listen} from "@tauri-apps/api/event";
import {configInit, getClientPath} from "@/background/utils/config.ts";
import {GameFlow} from "@/background/gameFlow.ts";

class Background {
    private gameFlow: GameFlow;

    constructor() {
        this.gameFlow = new GameFlow();
        createMainWindows();
        configInit();
        this.initializeListeners();
    }

    /**
     * 初始化事件监听器
     * 设置客户端状态监听，当收到客户端启动通知后开始监听客户端状态变化事件
     * @returns void
     */
    private initializeListeners() {
        invoke("listen_client_start").then(async () => {
            await listen<string>("client_status", (event) => {
                this.handleClientStatus(event.payload);
            });
        });
    };

    private initAssistant() {
        const TIME_LIMIT = 3000;
        let elapsedTime = 0;
        const intervalTime = 3000;

        const lcuSuccess = setInterval(async () => {
            const clientPath = await getClientPath();
            if (clientPath) {
                clearInterval(lcuSuccess);
                setTimeout(() => {
                    this.gameFlow.sendStartEvent();
                    invoke("start_listener");
                }, 500);
            }
            elapsedTime += intervalTime;
            if (elapsedTime >= TIME_LIMIT) {
                clearInterval(lcuSuccess);
                console.log("超时，客户端未启动");
            }
        }, intervalTime);
    }

    /**
     * 处理客户端状态变化
     * 根据不同的客户端状态执行相应的处理逻辑
     * @param status - 客户端状态字符串
     */
    private handleClientStatus(status: string) {
        switch (status) {
            case "ClientStarted":
                this.initAssistant();
                break;
        }
    }
}

new Background();
