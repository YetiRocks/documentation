# Applications

A Yeti **application** is a self-contained unit of functionality deployed to the platform. Each application bundles its configuration, schema definitions, custom logic, seed data, and optional static files into a single directory. Applications are isolated from each other -- each gets its own database namespace, URL prefix, and route space.

## Directory Structure

Every application lives under `~/yeti/applications/{app-id}/` and follows this layout:

```
~/yeti/applications/my-app/
  config.yaml          # Application configuration (required)
  schema.graphql       # Table definitions using GraphQL SDL
  resources/           # Custom Rust resource handlers
    greeting.rs
    dashboard.rs
  data/                # Seed data loaded on startup
    users.json
    products.json
  web/                 # Static files (HTML, CSS, JS)
    index.html
    style.css
```

Only `config.yaml` is strictly required. All other files are optional depending on what the application needs.

## Configuration Anatomy

The `config.yaml` file defines everything about the application. Here is a complete example:

```yaml
# Metadata
name: "My Application"
app_id: "my-app"
version: "1.0.0"
description: "A sample Yeti application"

# Enable/disable the application
enabled: true

# Interface toggles (which protocols to expose)
rest: true
graphql: true
ws: true
sse: false

# Schema files that define tables
schemas:
  - schema.graphql

# Custom resource handlers (glob patterns supported)
resources:
  - resources/*.rs

# Static file serving
static_files:
  path: web
  route: "/"
  index: index.html

# Seed data loaded on first startup
dataLoader: data/*.json

# Extensions this app uses (with optional inline config)
extensions:
  - yeti-auth:
      oauth:
        rules:
          - strategy: provider
            pattern: "github"
            role: standard

# Rust crate dependencies for custom resources
dependencies:
  serde_yaml: "0.9"
```

Key fields:

| Field | Purpose |
|-------|---------|
| `app_id` | URL prefix and database namespace identifier |
| `enabled` | Toggle the application on/off without removing it |
| `rest` / `graphql` / `ws` / `sse` | Enable specific protocol interfaces |
| `schemas` | List of GraphQL SDL files that define tables |
| `resources` | Glob patterns for custom Rust resource files |
| `static_files` | Serve a directory of static files at a route |
| `dataLoader` | Glob pattern for JSON seed data files |
| `extensions` | Ordered list of extensions with per-app config |
| `dependencies` | Additional Rust crate dependencies for resources |

## Application Discovery

On startup, Yeti scans `~/yeti/applications/*/` for directories containing a `config.yaml` file. Each valid directory becomes a registered application. No manual registration is needed -- drop a directory with a config file and restart the server.

```
~/yeti/applications/
  application-template/    # Discovered as "application-template"
  graphql-explorer/        # Discovered as "graphql-explorer"
  yeti-auth/               # Discovered as "yeti-auth" (extension)
  yeti-telemetry/          # Discovered as "yeti-telemetry" (extension)
  my-new-app/              # Discovered automatically on next restart
```

Applications with `enabled: false` are discovered but not loaded.

## Multi-Tenancy

Each application operates in its own isolated namespace:

- **URL prefix**: All routes are prefixed with `/{app-id}/`. A `Book` table in the `graphql-explorer` app is accessible at `https://localhost:9996/graphql-explorer/Book`.
- **Database namespace**: The `@table(database: "...")` directive in schemas controls storage isolation. Tables from different apps never collide.
- **Route space**: Resources and tables within an app share the same route space but cannot conflict with other apps.

## Extension Applications

Applications with `extension: true` in their config provide shared services to other applications. Extension apps are loaded before regular apps and can supply authentication providers, middleware, telemetry pipelines, and other cross-cutting concerns.

```yaml
# yeti-auth/config.yaml
name: Yeti Auth
enabled: true
extension: true      # This makes it an extension
version: "0.1.0"
schemas:
  - schema.graphql
resources:
  - resources/*.rs
```

Consumer apps opt in to extensions via the `extensions:` field. Only declared extensions have their tables and services merged into the consuming app. See [Extensions](extensions.md) for details.

## Apps Without Tables

Not every application needs a database. Apps that only serve static files or provide custom API endpoints can omit the `schemas:` section entirely:

```yaml
name: "Yeti Documentation"
app_id: "documentation"
version: "1.0.0"
enabled: true

static_files:
  path: web
  route: /
  index: index.html
```

This pattern is common for documentation sites, single-page applications, and pure-API services that consume data from extensions.
