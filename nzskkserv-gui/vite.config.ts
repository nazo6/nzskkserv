import { defineConfig } from 'vite';

import solidPlugin from 'vite-plugin-solid';
import tsconfigPaths from 'vite-tsconfig-paths';

import Icons from 'unplugin-icons/vite';

export default defineConfig(() => {
  return {
    root: './src',
    plugins: [solidPlugin(), tsconfigPaths({ root: '../' }), Icons({ compiler: 'solid' })],
    build: {
      target: 'esnext',
      polyfillDynamicImport: false,
      outDir: '../dist/',
      emptyOutDir: true
    },
  };
});
