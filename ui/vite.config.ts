import path from "path";
import dotenv from "dotenv";
import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";

dotenv.config({ path: ".env.development" });

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react()],
  server: {
    port: 3220,
    allowedHosts: process.env.ALLOWED_HOST
      ? [process.env.ALLOWED_HOST]
      : undefined,
  },
  resolve: {
    alias: {
      "@": path.resolve(__dirname, "./src"),
    },
  },
});
