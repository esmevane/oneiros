// @ts-check
import { defineConfig } from 'astro/config';
import react from '@astrojs/react';

// https://astro.build/config
export default defineConfig({
	integrations: [react()],
	server: {
		port: 4321,
	},
	vite: {
		server: {
			proxy: {
				// Forward API calls to the local oneiros host during dev.
				'/v1': {
					target: 'http://127.0.0.1:2100',
					changeOrigin: true,
				},
			},
		},
	},
});
