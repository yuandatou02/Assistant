import {WebviewWindow} from "@tauri-apps/api/webviewWindow";

export const createMainWindows = async () => {
    const webview = new WebviewWindow("mainWindow", {
        title: "assistant",
        url: "src/main/index.html",
        width: 320,
        height: 720,
        visible: false,
        resizable: false,
        decorations: false,
        center: true,
        transparent: true,
    });

    // 1. 等事件触发，拿到 unlisten
    await webview.once("tauri://webview-created", async () => {
        // 2. 再显示，并显式接住错误
        await webview.show();
    });
};