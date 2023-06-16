import path from "path";
import { sveltekit } from "@sveltejs/kit/vite";
import { defineConfig } from "vitest/config";

export default defineConfig({
	plugins: [sveltekit()],
	test: {
		include: ["src/**/*.{test,spec}.{js,ts}"],
	},
	resolve: {
		alias: {
			"@api": path.join(process.cwd(), "src/api"),
			"@css": path.join(process.cwd(), "src/css"),
			"@components": path.join(process.cwd(), "src/components"),
			"@": path.join(process.cwd(), "src"),
		},
	},
});
