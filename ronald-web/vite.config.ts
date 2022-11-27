import { defineConfig } from 'vite';
import solidPlugin from 'vite-plugin-solid';
import wasmPack from 'vite-plugin-wasm-pack';

export default defineConfig({
  plugins: [
    solidPlugin(),
    wasmPack('./ronald-wasm'),
  ],
  server: {
    port: 3000,
  },
  build: {
    target: 'esnext',
  },
});
