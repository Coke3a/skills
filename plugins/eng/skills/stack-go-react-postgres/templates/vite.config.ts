// <project>/frontend/vite.config.ts
//
// ⚠ Everything below is under `server`, which `vite build` NEVER reads. It looks like dev
//   cruft; removing it breaks the container while the build keeps passing.

import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'

export default defineConfig({
  plugins: [react()],
  server: {
    host: true,          // bind 0.0.0.0 — or the server is unreachable from the host
    port: 5173,
    strictPort: true,    // ⚠ not optional: Vite otherwise drifts to the next free port and
                         //   the 5173:5173 mapping then points at nothing, silently
    proxy: {
      // ⚠ this is what makes the browser see ONE origin in development.
      //   The API base in application code stays the relative '/api/v1' everywhere.
      '/api/v1': { target: process.env.DEV_PROXY_TARGET ?? 'http://localhost:8080' },
    },
    watch: { usePolling: !!process.env.CHOKIDAR_USEPOLLING },
    // ws: { clientPort: 5173 },  // ⚠ Vite 8 moved this from server.hmr.* — only needed when
                                  //   the published host port differs from the container port
  },
})
