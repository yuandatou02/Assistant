import { WebviewWindow } from "@tauri-apps/api/webviewWindow";

// 创建主窗口函数
export const createMainWindows = () => {
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
  webview.once("tauri://webview-created", async () => webview.show());
};
