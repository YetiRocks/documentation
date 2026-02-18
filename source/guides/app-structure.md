# Application Structure

Every Yeti application lives in its own directory under `~/yeti/applications/`. The directory name should match the `app_id` in your configuration.

## Directory Layout

Here is a complete example showing all possible files and directories:

```
~/yeti/applications/my-app/
  config.yaml          # Required. Application configuration
  schema.graphql       # Schema defining tables and types
  resources/           # Custom Rust resource handlers
    greeting.rs
    page_cache.rs
  data/                # Seed data (JSON files)
    products.json
    categories.json
  web/                 # Static files (frontend app)
    index.html
    assets/
      main.js
      style.css
  Cargo.toml           # Auto-generated for plugin compilation
  build.rs             # Auto-generated build script
  source/              # Auto-generated: compiled resource sources
  target/              # Auto-generated: build artifacts
  test/                # Integration tests
```

## Required Files

### config.yaml

The only truly required file. At minimum it needs:

```yaml
name: "My Application"
app_id: "my-app"
version: "1.0.0"
enabled: true
rest: true
```

### Key Configuration Fields

| Field | Type | Description |
|-------|------|-------------|
| `name` | String | Human-readable application name |
| `app_id` | String | URL-safe identifier (used in routes) |
| `version` | String | Semantic version |
| `enabled` | Boolean | Whether the app loads at startup |
| `rest` | Boolean | Enable REST API endpoints |
| `graphql` | Boolean | Enable GraphQL endpoint |
| `ws` | Boolean | Enable WebSocket support |
| `sse` | Boolean | Enable Server-Sent Events |
| `schemas` | List | GraphQL schema files to load |
| `resources` | List | Glob patterns for custom resource `.rs` files |
| `dataLoader` | String | Glob pattern for seed data JSON files |
| `static_files` | Object | Static file serving configuration |

## Optional Files

### schema.graphql

Defines your data model. If your application has no tables (e.g., it only serves static files or provides custom resources), you can omit both this file and the `schemas:` config entry.

### resources/*.rs

Custom Rust handlers. Each `.rs` file is compiled into the application plugin. Reference them in config:

```yaml
resources:
  - resources/*.rs
```

### data/*.json

Seed data loaded on startup. Each file targets a specific table. Reference them in config:

```yaml
dataLoader: data/*.json
```

### web/

Static files served via the built-in file server. Configure the mapping in config:

```yaml
static_files:
  path: web
  route: "/"
  index: index.html
```

## Naming Conventions

- **app_id**: Use lowercase with hyphens: `my-app`, `graphql-explorer`, `yeti-auth`
- **Schema types**: PascalCase: `Product`, `UserProfile`, `OrderItem`
- **Resource files**: snake_case: `greeting.rs`, `page_cache.rs`
- **Seed data files**: Match table names in lowercase: `products.json`, `authors.json`
- **Database names**: Lowercase with hyphens, typically matching the app_id

## Auto-Generated Files

When Yeti compiles your application, it generates several files automatically:

- **`Cargo.toml`** -- Rust project manifest for the plugin
- **`build.rs`** -- Build script for compilation
- **`source/`** -- Copied and generated source files (including `lib.rs`)

These files are managed by the compiler. Do not edit them manually. The compiler copies your resource files into `source/` before generating `lib.rs`, which scans the sources to detect resource types.

## Complete Example: graphql-explorer

```
~/yeti/applications/graphql-explorer/
  config.yaml
  schema.graphql
  data/
    authors.json
    books.json
    categories.json
    publishers.json
    reviews.json
  web/
    index.html
    src/
    public/
    package.json
    vite.config.ts
```

With this `config.yaml`:

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

This gives you REST endpoints for every table type, a GraphQL endpoint, seed data loaded at startup, and a web UI served at the application root.
