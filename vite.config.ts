import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'

// https://vite.dev/config/
export default defineConfig({
  plugins: [vue()],

  // Prevent Vite from opening the browser on start
  server: {
    open: false,
    port: 5173,
    strictPort: false,
  },

  // Make Tauri env variables available
  envPrefix: ['VITE_', 'TAURI_'],

  // For production builds
  build: {
    // Tauri uses Chromium on Windows, so we target a modern version
    target: 'esnext',
    // Don't minify debug builds
    minify: process.env.TAURI_DEBUG ? false : 'esbuild',
    // Generate sourcemaps for debugging in production
    sourcemap: !!process.env.TAURI_DEBUG,
  },
})
