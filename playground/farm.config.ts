import { defineConfig } from '@farmfe/core';
import react from '@farmfe/plugin-react';
import farmPlugin from 'farm-rust-plugin-auto-routes';

export default defineConfig({
  compilation: {
    input: {
      index: './index.html',
    },
    persistentCache: false,
    progress: false,
    // lazyCompilation: true,
  },
  plugins: [
    react({ runtime: 'automatic' }),
    farmPlugin({ mode: 'hash' }),
    // 'farm-rust-plugin-auto-routes',
  ],
});
