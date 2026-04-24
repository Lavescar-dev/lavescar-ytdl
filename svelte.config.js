import adapter from '@sveltejs/adapter-static';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';

/** @type {import('@sveltejs/kit').Config} */
export default {
  preprocess: vitePreprocess(),

  kit: {
    // SPA mode: SvelteKit produces static assets; Tauri's webview serves them.
    // `fallback: 'index.html'` enables client-side routing for any path.
    adapter: adapter({
      pages: 'build',
      assets: 'build',
      fallback: 'index.html',
      precompress: false,
      strict: true
    }),

    // Disable prerender because everything is client-rendered.
    prerender: { entries: [] }
  }
};
