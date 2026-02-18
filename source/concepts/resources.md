# Resources

Resources are custom HTTP endpoint handlers written in Rust. They override or extend the auto-generated table endpoints, giving you full control over request handling, response formatting, and business logic.

## The Simplest Resource

Using the `resource!` macro, a complete endpoint can be defined in a few lines:

```rust
// resources/greeting.rs
use yeti_core::prelude::*;

resource!(Greeting {
    get => json!({"greeting": "Hello, World!"})
});
```

This creates a GET endpoint at `/{app-id}/greeting` that returns JSON. The macro handles struct definition, trait implementation, and plugin registration automatically.

## The Resource Trait

Under the hood, every resource implements the `Resource` trait. The macro-free version gives full control:

```rust
use yeti_core::prelude::*;

#[derive(Clone, Default)]
pub struct Protected;

impl Resource for Protected {
    fn name(&self) -> &str { "protected" }

    fn get(&self, _req: Request<Vec<u8>>, ctx: ResourceParams) -> ResourceFuture {
        Box::pin(async move {
            if let Some(identity) = ctx.auth_identity() {
                return ok(json!({
                    "authenticated": true,
                    "username": identity.username(),
                }));
            }
            unauthorized("Authentication required.")
        })
    }
}
```

The trait provides methods for each HTTP verb. Unimplemented methods return 405 Method Not Allowed by default.

| Method | HTTP Verb | Purpose |
|--------|-----------|---------|
| `get()` | GET | Retrieve a record or collection |
| `post()` | POST | Create a new record or trigger an action |
| `put()` | PUT | Create or fully replace a record |
| `patch()` | PATCH | Partially update a record |
| `delete()` | DELETE | Remove a record |
| `search()` | GET (collection) | Query/filter a collection |
| `subscribe()` | SSE/WS | Subscribe to real-time updates |
| `connect()` | WS | Handle WebSocket connections |

## Macro Shortcuts

The `resource!` macro supports multiple methods and access to request/params:

```rust
use yeti_core::prelude::*;

resource!(Items {
    get(request, ctx) => {
        let items = ctx.get_table("Items")?;
        let results = items.get(None).await?;
        ok(json!(results))
    },
    post(request, ctx) => {
        let body: serde_json::Value = request.json()?;
        let items = ctx.get_table("Items")?;
        let id = body["id"].as_str().unwrap_or("auto");
        items.put(id, body.clone()).await?;
        created(body)
    }
});
```

Individual method macros (`get!`, `post!`, `put!`, `patch!`, `delete!`, `search!`) are also available for use inside manual `impl Resource` blocks when you need to mix macro convenience with custom trait methods.

## ResourceParams

The `ResourceParams` (often named `ctx`) object is passed to every handler and provides access to the application context:

| Method | Returns | Purpose |
|--------|---------|---------|
| `ctx.get_table("Name")` | `Result<Table>` | Access a table by name |
| `ctx.tables()` | `Result<&Tables>` | Access all tables |
| `ctx.id()` | `Option<&str>` | The `{id}` from the URL path |
| `ctx.config()` | `&Config` | Application configuration |
| `ctx.auth_identity()` | `Option<&AuthIdentity>` | Authenticated user identity |
| `ctx.access_control()` | `Option<&dyn AccessControl>` | Resolved role/permissions |
| `ctx.response_headers()` | `&HeaderMap` | Set custom response headers |
| `ctx.extension_config("name")` | `Option<&Value>` | Per-app extension config |

## Response Helpers

Yeti provides response helpers and macros for common patterns:

```rust
// JSON responses
ok(json!({"status": "ok"}))          // 200 with JSON body
created(json!({"id": "new-123"}))    // 201 Created
bad_request("Invalid input")         // 400 Bad Request
unauthorized("Login required")       // 401 Unauthorized
not_found("No such record")          // 404 Not Found

// Response macros
ok_json!({"status": "ok"})           // 200 with JSON (macro)
ok_html("<h1>Hello</h1>")            // 200 with HTML content type
ok_text("plain text")                // 200 with text/plain

// Builder pattern for full control
reply().status(302).header("Location", "/new-url").empty()
reply().header("x-cache", "HIT").json(&data)
```

## Default Resources

A default resource catches all paths not matched by other resources or tables. This is used for single-page application routing and caching proxies:

```rust
use yeti_core::prelude::*;

resource!(PageCache {
    default = true,
    get(request, ctx) => {
        let path = ctx.id().unwrap_or("/");
        let cache = ctx.get_table("PageCache")?;
        if let Some(cached) = cache.get_by_id(path).await? {
            ok_html(cached.as_str().unwrap_or_default())
        } else {
            not_found("Page not cached")
        }
    }
});
```

When `is_default()` returns `true` (or `default = true` in the macro), the resource handles all unmatched URL paths. The full path is available via `ctx.id()`.

## Plugin Compilation

Resources are compiled to dynamic libraries (`.dylib` on macOS, `.so` on Linux) and loaded at runtime. This enables hot-reload during development -- change a resource file, restart the server, and the new code takes effect. Initial compilation takes approximately 2 minutes per resource file; cached rebuilds are much faster.

Resource files are placed in the `resources/` directory and referenced in `config.yaml`:

```yaml
resources:
  - resources/*.rs
```

Each `.rs` file should contain one resource definition. The `register_resource!` macro (called automatically by `resource!`) exports the resource for the plugin loader.
