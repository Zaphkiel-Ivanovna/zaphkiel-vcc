import { defineConfig } from 'astro/config';

export default defineConfig({
  site: 'https://vcc.zaphkiel.dev',
  vite: {
    server: {
      proxy: { '/index.json': 'http://localhost:8080' },
    },
  },
});
