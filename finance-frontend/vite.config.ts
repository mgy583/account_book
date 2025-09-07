import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";

export default defineConfig({
  plugins: [react()],
  server: {
    port: 5173,
    proxy: {
  "/accounts": "http://localhost:3000",
  "/categories": "http://localhost:3000",
  "/assets": "http://localhost:3000",
  "/transactions": "http://localhost:3000",
  "/budgets": "http://localhost:3000",
  "/register": "http://localhost:3000",
  "/login": "http://localhost:3000",
    },
  },
});
