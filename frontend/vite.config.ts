import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vitest/config';
import { loadEnv } from 'vite';

export default defineConfig(({ mode }) => {
    // Vite charge .env dans import.meta.env, pas dans process.env — or le code
    // serveur (lib/server, hooks) lit process.env (parité avec adapter-node en prod).
    // Sans ça, `npm run dev` tourne sur les fallbacks (DB postgres:postgres, admin 503…).
    Object.assign(process.env, loadEnv(mode, process.cwd(), ''));

    return {
        plugins: [sveltekit()],
        test: {
            include: ['src/**/*.{test,spec}.{js,ts}'],
            environment: 'node',
            globals: true,
        },
    };
});
