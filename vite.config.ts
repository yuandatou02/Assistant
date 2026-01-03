import {defineConfig} from "vite";
import vue from "@vitejs/plugin-vue";

// https://vite.dev/config/
export default defineConfig({
    plugins: [vue()],
    // 防止 Vite 清除 Rust 显示的错误
    clearScreen: false,

});
