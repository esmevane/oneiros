// @ts-check
import { defineConfig } from 'astro/config';
import react from '@astrojs/react';
import starlight from '@astrojs/starlight';

// https://astro.build/config
export default defineConfig({
	integrations: [
		react(),
		starlight({
			title: 'Oneiros',
			description: 'Persistent continuity control for your AI agents.',
			customCss: ['./src/styles/oneiros.css'],
			components: {
				// Hide the theme selector — we only ship dark-mode tokens for now.
				ThemeSelect: './src/components/null.astro',
				// Replace the default footer with our own minimal one.
				Footer: './src/components/Footer.astro',
				// Render the sidebar through our design system primitives.
				Sidebar: './src/components/Sidebar.astro',
			},
			head: [
				{ tag: 'link', attrs: { rel: 'preconnect', href: 'https://fonts.googleapis.com' } },
				{ tag: 'link', attrs: { rel: 'preconnect', href: 'https://fonts.gstatic.com', crossorigin: true } },
				{
					tag: 'link',
					attrs: {
						rel: 'stylesheet',
						href: 'https://fonts.googleapis.com/css2?family=Syne+Mono&family=Syne:wght@400;500;600;700;800&family=Spectral:ital,wght@0,300;0,400;0,600;0,700;1,300;1,400&display=swap',
					},
				},
			],
			social: [
				{ icon: 'github', label: 'GitHub', href: 'https://github.com/esmevane/oneiros' },
			],
			sidebar: [
				{
					label: 'Concepts',
					items: [
						{ label: 'Continuity', slug: 'concepts/continuity' },
					],
				},
				{
					label: 'Reference',
					items: [{ autogenerate: { directory: 'reference' } }],
				},
			],
		}),
	],
});
