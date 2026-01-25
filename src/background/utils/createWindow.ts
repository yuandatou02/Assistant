import {WebviewWindow} from "@tauri-apps/api/webviewWindow";

// 创建主窗口函数
export const createMainWindows = async () => {
    const webview = new WebviewWindow("mainWindow", {
        title: "Frank",
        url: "src/main/index.html",
        width: 320,
        height: 720,
        visible: false,
        resizable: false,
        decorations: false,
        center: true,
        transparent: true,
    });
    await webview.once("tauri://webview-created", async () => webview.show());
};

// 创建查询战绩窗口方法
export const createQueryMatchWindow = async () => {
    const webview = new WebviewWindow("queryMatchWindow", {
        title: "我的战绩",
        url: "src/queryMatch/index.html",
        width: 1174,
        height: 668,
        resizable: false,
        decorations: false,
        center: true,
        visible: false,
        transparent: true
    });
    await webview.once("tauri://created", async () => await webview.show());
};
