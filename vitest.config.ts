import { defineConfig } from "vitest/config"
import vue from "@vitejs/plugin-vue"
import { resolve } from "path"

export default defineConfig({
	plugins: [vue()],
	test: {
		environment: "jsdom",
		globals: true,
		pool: "forks",
		poolOptions: {
			forks: {
				singleFork: true,
			},
		},
		include: ["src/**/*.{test,spec}.{js,ts}"],
		coverage: {
			provider: "v8",
			reporter: ["text", "json", "html"],
			reportsDirectory: "./coverage",
			include: ["src/**/*.{js,ts,vue}"],
			exclude: [
				"src/**/*.d.ts",
				"src/**/*.spec.{js,ts}",
				"src/**/*.test.{js,ts}",
				"src/main.ts",
				"src/App.vue",
				"src/assets/**",
				"src/components/**/index.ts",
			],
		},
	},
	resolve: {
		alias: {
			"@": resolve(__dirname, "src"),
		},
	},
})
