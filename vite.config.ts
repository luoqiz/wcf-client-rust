import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";
import VueRouter from "unplugin-vue-router/vite";
import { resolve } from "path";
import UnoCSS from "unocss/vite";
import Components from "unplugin-vue-components/vite";
import { PrimeVueResolver } from "unplugin-vue-components/resolvers";

const host = process.env.TAURI_DEV_HOST;

// https://vitejs.dev/config/
export default defineConfig(async () => ({
  resolve: {
    alias: {
      "~/": `${resolve(__dirname, "src")}/`,
      "#": `${resolve(__dirname, "types")}/`,
    },
  },
  plugins: [
    VueRouter({
      routesFolder: "src/pages",
      exclude: ["**/components/*.vue"],
      extensions: [".vue"],
      // dts: 'types/typed-router.d.ts'
    }),
    vue(),
    UnoCSS(),
    Components({
      resolvers: [PrimeVueResolver()],
    }),
  ],

  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  //
  // 1. prevent vite from obscuring rust errors
  clearScreen: false,
  // 2. tauri expects a fixed port, fail if that port is not available
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
}));
