import {defineConfig} from "vite";
import vue from "@vitejs/plugin-vue";
import path from "node:path";
import tailwindcss from "@tailwindcss/vite";

const host = process.env.TAURI_DEV_HOST;

// https://vite.dev/config/
export default defineConfig({
    plugins: [vue(), tailwindcss()],
    clearScreen: false,
    server: {
        port: 1420,
        strictPort: true,
        host: host || false,
        hmr: host
            ? {
                protocol: "ws",
                host,
                port: 1421,
            }
            : undefined,
        watch: {
            // 3. tell vite to ignore watching `src-tauri`
            ignored: ["**/src-tauri/**"],
        },
    },
    resolve: {
        alias: {
            "@": path.resolve("./src"), // @代替src
        },
    },
    envPrefix: ["VITE_", "TAURI_ENV_*"],
    build: {
        // Tauri 在 Windows 上使用 Chromium，在 macOS 和 Linux 上使用 WebKit
        target:
            process.env.TAURI_ENV_PLATFORM == "windows" ? "chrome105" : "safari13",
        // 在 debug 构建中不使用 minify
        minify: !process.env.TAURI_ENV_DEBUG ? "esbuild" : false,
        // 在 debug 构建中生成 sourcemap
        sourcemap: !!process.env.TAURI_ENV_DEBUG,
    },
});
