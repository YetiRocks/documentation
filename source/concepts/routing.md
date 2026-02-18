# Routing & Endpoints

Yeti's **AutoRouter** generates routes automatically from schema definitions and resource registrations. Every table with `@export` and every resource file produces HTTP endpoints without manual route configuration.

## URL Structure

All application endpoints follow the pattern:

```
https://localhost:9996/{app-id}/{resource-or-table}
```

The server runs on HTTPS port 9996 with self-signed certificates by default. Use `curl -sk` to skip certificate verification during development.

## Table Routes

Every `@export`ed table automatically gets six REST endpoints:

| Method | Path | Action |
|--------|------|--------|
| GET | `/{app-id}/{Table}` | List records (with query support) |
| POST | `/{app-id}/{Table}` | Create a new record |
| GET | `/{app-id}/{Table}/{id}` | Retrieve a single record |
| PUT | `/{app-id}/{Table}/{id}` | Create or fully replace a record |
| PATCH | `/{app-id}/{Table}/{id}` | Partially update a record |
| DELETE | `/{app-id}/{Table}/{id}` | Delete a record |

```bash
# Create a record
curl -sk -X POST https://localhost:9996/graphql-explorer/Book \
  -H "Content-Type: application/json" \
  -d '{"id": "book-1", "title": "Rust in Action", "isbn": "978-1617294556"}'

# Get a record by ID
curl -sk https://localhost:9996/graphql-explorer/Book/book-1

# Delete a record
curl -sk -X DELETE https://localhost:9996/graphql-explorer/Book/book-1
```

## Query Parameters

Collection endpoints (GET without an ID) support filtering, pagination, sorting, and field selection:

```bash
# Pagination
curl -sk "https://localhost:9996/graphql-explorer/Book?limit=10&offset=20"

# Sorting (prefix with - for descending)
curl -sk "https://localhost:9996/graphql-explorer/Book?sort=-price"

# Field selection
curl -sk "https://localhost:9996/graphql-explorer/Book?select=title,price"

# FIQL filters on indexed fields
curl -sk "https://localhost:9996/example-queries/Products?price=gt=20;category==electronics"
curl -sk "https://localhost:9996/example-queries/Products?category==books,category==music"
```

FIQL operators: `==` (equals), `!=` (not equals), `=gt=` (greater than), `=ge=` (greater or equal), `=lt=` (less than), `=le=` (less or equal). Use `;` for AND and `,` for OR.

## Custom Resources Override Tables

When a custom resource has the same name as a table, the resource handler takes precedence. This lets you add validation, transformation, or access control logic on top of auto-generated CRUD:

```rust
// resources/users.rs -- overrides the auto-generated User table endpoints
use yeti_core::prelude::*;

resource!(Users {
    post(req, ctx) => {
        let mut body: serde_json::Value = req.json()?;
        // Add server-side validation or transformation
        body["createdAt"] = json!(unix_timestamp()?);
        let table = ctx.get_table("User")?;
        table.post(None, body.clone()).await?;
        created(body)
    }
});
```

Methods not overridden by the resource fall through to the default table handler.

## Default Resources

A default resource (with `is_default() = true`) catches all paths not matched by other resources or tables. This enables patterns like:

- **SPA routing** -- Serve `index.html` for all unmatched paths so client-side routing works.
- **Caching proxy** -- Use the URL path as a cache key and fetch from an origin on miss.
- **Custom 404 pages** -- Return a branded error page for unknown routes.

```rust
resource!(SpaFallback {
    default = true,
    get => ok_html(include_str!("../web/index.html"))
});
```

Only one default resource can be active per application.

## Real-Time Streaming

Table endpoints support real-time data streaming via query parameters:

```bash
# Server-Sent Events (requires sse: true in config)
curl -sk -N "https://localhost:9996/realtime-demo/Events?stream=sse"

# WebSocket (requires ws: true in config)
# Use a WebSocket client to connect to:
# wss://localhost:9996/realtime-demo/Events?stream=ws
```

SSE streams push JSON events whenever records in the table change. WebSocket connections support bidirectional communication for pub/sub patterns.

## GraphQL Endpoint

When `graphql: true` is set in the application config, a GraphQL endpoint is available:

```bash
curl -sk -X POST https://localhost:9996/graphql-explorer/graphql \
  -H "Content-Type: application/json" \
  -d '{"query": "{ Book { id title author { name } } }"}'
```

The GraphQL schema is auto-generated from the same `schema.graphql` files that define tables. Relationships defined with `@relationship` are automatically resolvable in GraphQL queries.

## Static File Serving

Applications can serve static files (HTML, CSS, JS, images) from a directory:

```yaml
static_files:
  path: web          # Directory relative to app root
  route: "/"         # URL mount point within the app prefix
  index: index.html  # Default file for directory requests
```

With this config, `https://localhost:9996/my-app/style.css` serves `~/yeti/applications/my-app/web/style.css`. The `index` file is served for bare directory requests.

## Operations API

A separate administration server runs on port 9995 for operational tasks:

```bash
# Operations API (separate port)
curl -sk https://localhost:9995/status
```

The operations API is intended for monitoring, health checks, and administrative actions. It runs independently from the application server to ensure management access even under heavy application load.

## Route Priority

When multiple handlers could match a request, Yeti resolves them in this order:

1. **Custom resources** -- Exact name match on the path segment
2. **Table endpoints** -- `@export`ed tables matching the path
3. **Default resource** -- The app's catch-all handler (if defined)
4. **Static files** -- Files in the `static_files` directory
5. **404 Not Found** -- No handler matched
