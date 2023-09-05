// @ts-check
import path from "path";

// 1. Import the Skeleton plugin
import { skeleton } from '@skeletonlabs/tw-plugin';
import plugin from "tailwindcss/plugin.js";

/** @type {import('tailwindcss').Config} */
module.exports = {
	// 2. Opt for dark mode to be handled via the class method
	darkMode: 'class',
	content: [
		'./src/**/*.{html,js,svelte,ts}',
		// 3. Append the path to the Skeleton package
		path.join(require.resolve(
				'@skeletonlabs/skeleton'),
			'../**/*.{html,js,svelte,ts}'
		)
	],
	theme: {
		extend: {},
	},
	plugins: [
		// 4. Append the Skeleton plugin (after other plugins)
		plugin(function({ addUtilities, addComponents, e, config }) {
			const utilities = {};
			const colors = ['surface','primary','secondary','tertiary','success','warning','error'];
			const numbers = [50, 100, 200, 300, 400, 500, 600, 700, 800, 900];
			for (const color of colors){
				utilities[`.neon-${color}`] = {
					boxShadow: `0 0 5px rgb(var(--color-${color}-500) / 1), 0 0 20px rgb(var(--color-${color}-700) / 1)`,
				};
				utilities[`.neon-text-${color}`] = {
					textShadow: `0 0 5px rgb(var(--color-${color}-500) / 1), 0 0 20px rgb(var(--color-${color}-700) / 1)`,
				};
			}
			addUtilities(utilities);
		}),
		skeleton({
			themes: { preset: [ "skeleton", "rocket", "modern" ] }
		})
	]
}