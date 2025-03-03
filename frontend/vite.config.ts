import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'

// https://vite.dev/config/
export default defineConfig({
  plugins: [react()],
  server: {
    proxy: {
      // Redirect API and auth calls via reverse proxy to avoid CORS issues
      '^/api/.*': {
        target: 'http://0.0.0.0:3000',
        changeOrigin: true,
      },
      '^/auth/.*': {
        target: 'http://0.0.0.0:3000',
        changeOrigin: true,
      }
    },
  },
})
