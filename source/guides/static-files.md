# Static File Serving

Yeti includes a built-in static file server for hosting frontend applications alongside your API. This is useful for single-page applications (SPAs), documentation sites, dashboards, and any web content.

## Configuration

Add a `static_files` section to your `config.yaml`:

```yaml
static_files:
  path: web          # Directory containing static files (relative to app dir)
  route: "/"         # URL route prefix where files are served
  index: index.html  # Default file for directory requests
```

| Field | Description |
|-------|-------------|
| `path` | Directory containing your static files, relative to the application directory |
| `route` | URL prefix where files are served. Use `"/"` for the application root |
| `index` | The file served when a directory is requested (e.g., navigating to `/`) |

## Directory Structure

Place your static files in the configured directory:

```
~/yeti/applications/my-app/
  config.yaml
  web/
    index.html
    assets/
      main.js
      style.css
    images/
      logo.png
```

With `route: "/"`, these files are served at:

- `https://localhost:9996/my-app/` serves `web/index.html`
- `https://localhost:9996/my-app/assets/main.js` serves `web/assets/main.js`
- `https://localhost:9996/my-app/images/logo.png` serves `web/images/logo.png`

## React / Vite Integration

A common pattern is hosting a React or Vue application built with Vite.

### Project Setup

Place the frontend project in the `web/` directory:

```
~/yeti/applications/my-app/
  config.yaml
  schema.graphql
  web/
    index.html
    src/
      App.tsx
      main.tsx
    public/
      favicon.ico
    package.json
    vite.config.ts
    tsconfig.json
```

### Vite Configuration

Configure Vite to build into a directory that Yeti can serve. A typical `vite.config.ts`:

```typescript
import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'

export default defineConfig({
  plugins: [react()],
  base: '/my-app/',  // Must match your app_id for correct asset paths
  build: {
    outDir: 'dist',
  },
})
```

If you build to a `dist/` directory, update your config:

```yaml
static_files:
  path: web/dist
  route: "/"
  index: index.html
```

Alternatively, serve directly from the `web/` directory during development with the source files.

### Development Workflow

For local development, you can either:

1. **Serve from Yeti directly**: Place built files in the `web/` directory and access them via `https://localhost:9996/my-app/`

2. **Use Vite dev server with API proxy**: Run `vite dev` with a proxy to forward API calls to Yeti:

```typescript
// vite.config.ts
export default defineConfig({
  server: {
    proxy: {
      '/my-app/api': {
        target: 'https://localhost:9996',
        secure: false,  // Accept self-signed certs
      }
    }
  }
})
```

## SPA Routing

For single-page applications that use client-side routing (React Router, Vue Router), the `index` configuration ensures that all unmatched routes fall back to `index.html`. This allows deep links like `/my-app/dashboard/settings` to work correctly by serving the SPA entry point and letting the client-side router handle the path.

If your application also has custom resources that act as catch-all handlers (default resources), be aware that custom resources take priority over static file serving for matched routes.

## Content Types

Yeti automatically sets the correct `Content-Type` header based on file extensions:

| Extension | Content-Type |
|-----------|-------------|
| `.html` | `text/html; charset=utf-8` |
| `.css` | `text/css` |
| `.js` | `application/javascript` |
| `.json` | `application/json` |
| `.png` | `image/png` |
| `.jpg` | `image/jpeg` |
| `.svg` | `image/svg+xml` |
| `.woff2` | `font/woff2` |

## Example: graphql-explorer

The `graphql-explorer` application serves an Apollo-style GraphQL explorer UI:

```yaml
name: "GraphQL Explorer"
app_id: "graphql-explorer"
version: "1.0.0"
enabled: true
rest: true
graphql: true
schemas:
  - schema.graphql
dataLoader: data/*.json
static_files:
  path: web
  route: "/"
  index: index.html
```

The web UI is accessible at `https://localhost:9996/graphql-explorer/` and communicates with the GraphQL endpoint at `https://localhost:9996/graphql-explorer/graphql`.

## Combining Static Files with API Endpoints

Static files and API endpoints coexist naturally. Table endpoints and custom resources are matched first; static files serve as a fallback for unmatched paths. This means you can have:

- `GET /my-app/Product` -- REST API endpoint (from schema)
- `GET /my-app/` -- Static file (index.html)
- `GET /my-app/assets/main.js` -- Static file
