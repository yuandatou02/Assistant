import {createMainWindows} from "./utils/createWindow";
import {invoke} from "@tauri-apps/api/core";
import {listen} from "@tauri-apps/api/event";
import {configInit} from "@/background/utils/config.ts";

class Background {
    constructor() {
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
        console.log("initAssistant");
        const TIME_LIMIT = 3000;
        let elapsedTime = 0;
        const intervalTime = 3000;

        // const lcuSuccess = setInterval(async () => {
        // });
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
