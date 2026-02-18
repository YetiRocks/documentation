# Application Configuration

Complete reference for application-level `config.yaml` files located at `~/yeti/applications/{app-id}/config.yaml`.

---

## Metadata Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | string | yes | Human-readable application name |
| `app_id` | string | no | Application identifier. Defaults to directory name. Used as the URL prefix |
| `version` | string | no | Semantic version (e.g., `"1.0.0"`) |
| `description` | string | no | Application description |

```yaml
name: "My Application"
app_id: "my-app"
version: "1.0.0"
description: "A sample Yeti application"
```

---

## Application State

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `enabled` | boolean | `true` | Whether the application is loaded at startup |
| `extension` | boolean | `false` | Whether this application provides an extension (shared service) |

```yaml
enabled: true
extension: false
```

When `extension: true`, the compiler scans source files for `struct {TypeName}Extension` and generates the extension registration code automatically.

---

## Interface Flags

Control which protocols are exposed for this application's tables. Individual schemas can override these per-table using `@export` directives.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `rest` | boolean | `true` | Enable REST API endpoints |
| `graphql` | boolean | `false` | Enable GraphQL endpoint at `/{app-id}/graphql` |
| `ws` | boolean | `false` | Enable WebSocket subscriptions |
| `sse` | boolean | `false` | Enable Server-Sent Events streaming |

```yaml
rest: true
graphql: true
ws: true
sse: false
```

---

## Schemas

List of GraphQL schema files that define tables and their export directives. Paths are relative to the application directory.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `schemas` | string[] | `[]` | List of `.graphql` schema file paths |

```yaml
schemas:
  - schema.graphql
```

Applications without tables should omit the `schemas` section entirely.

---

## Resources

Custom resource files compiled as dynamic library plugins. Supports glob patterns. Paths are relative to the application directory.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `resources` | string[] | `[]` | List of `.rs` resource file paths or glob patterns |

```yaml
resources:
  - resources/*.rs
```

---

## Static Files

Serve static files (HTML, CSS, JS, images) from a directory.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `static_files.path` | string | -- | Directory containing static files (relative to app directory) |
| `static_files.route` | string | `"/"` | URL route prefix for static file serving |
| `static_files.index` | string | `"index.html"` | Default file served for directory requests |

```yaml
static_files:
  path: web
  route: "/"
  index: index.html
```

---

## Extensions

List of extension app IDs that this application depends on. Extensions are loaded before this application and their tables are merged into the application's backend manager.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `extensions` | string[] | `[]` | Ordered list of extension app IDs to load |

```yaml
extensions:
  - yeti-auth
  - yeti-telemetry
```

---

## Dependencies

Rust crate dependencies for the application's compiled plugin. Uses YAML syntax for version specification.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `dependencies` | map | `{}` | Crate name to version mapping |

```yaml
dependencies:
  argon2: "0.5"
  jsonwebtoken: { version: "10.3", features: ["rust_crypto"] }
  serde_yaml: "0.9"
```

---

## Data Loader

Seed data files to load on first startup. Supports glob patterns. Files must be JSON arrays of records.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `dataLoader` | string | -- | Glob pattern for seed data files |
| `dataLoader.files` | string | -- | Alternative: files sub-key with glob pattern |

```yaml
# Simple form
dataLoader: data/*.json

# Object form
dataLoader:
  files: data/*.json
```

---

## Custom Configuration

Application-specific configuration accessible via `ctx.config()` in resource handlers and `params.extension_config()` for extensions.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `custom` | map | `{}` | Arbitrary key-value configuration |
| `origin` | map | `{}` | Origin server configuration (used by caching patterns) |
| `environment` | map | `{}` | Application-specific environment variables |

```yaml
custom:
  jwt:
    secret: "${JWT_SECRET:-development-secret}"
    access_ttl: 900

origin:
  url: "https://www.example.com/"

environment:
  MODE: "redirect"
```

---

## Complete Example

```yaml
name: "My Application"
app_id: "my-app"
version: "1.0.0"
description: "Full-featured Yeti application"

enabled: true
extension: false

rest: true
graphql: true
ws: false
sse: true

schemas:
  - schema.graphql

resources:
  - resources/*.rs

static_files:
  path: web
  route: "/"
  index: index.html

extensions:
  - yeti-auth

dependencies:
  reqwest: { version: "0.12", features: ["blocking"] }

dataLoader: data/*.json
```

---

## See Also

- [Schema Directives](schema-directives.md) -- Table and field directives
- [Server Configuration](server-config.md) -- Server-level settings
- [Building Extensions](../guides/building-extensions.md) -- Extension development
