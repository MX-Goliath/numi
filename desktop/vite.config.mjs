import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import path from "path";

export default defineConfig({
	plugins: [react()],
	base: "./",
	root: path.join(import.meta.dirname, "src"),
	build: {
		outDir: path.join(import.meta.dirname, "dist"),
		emptyOutDir: true,
	},
	server: {
		host: "127.0.0.1",
        port: 3000,
        strictPort: true,
	},
});
