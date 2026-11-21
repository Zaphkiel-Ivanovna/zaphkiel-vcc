import { defineConfig } from 'astro/config';

export default defineConfig({
  build: {
    inlineStylesheets: 'always',
  },
  i18n: {
    defaultLocale: 'en',
    locales: ['en', 'fr'],
  },
  site: 'https://vcc.invalid',
  vite: {
    server: {
      proxy: { '/index.json': 'http://localhost:8080' },
    },
  },
});
