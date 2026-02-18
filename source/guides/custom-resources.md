# Custom Resources

Custom resources let you add business logic beyond the automatic CRUD operations that Yeti generates from your schema. Resources are Rust source files compiled into dynamic library plugins.

## Getting Started

Place `.rs` files in a `resources/` directory and reference them in `config.yaml`:

```yaml
resources:
  - resources/*.rs
```

Every resource file must start with the prelude import:

```rust
use yeti_core::prelude::*;
```

## The resource! Macro

The `resource!` macro is the simplest way to define a resource. It generates the struct, trait implementation, and plugin registration.

### Simple Resource (No Request Access)

```rust
use yeti_core::prelude::*;

resource!(Greeting {
    get => json!({"greeting": "Hello, World!"})
});
```

This creates a `GET /my-app/greeting` endpoint returning JSON.

### Resource with Request and Context Access

When you need to read the request body or access tables, name the parameters explicitly:

```rust
use yeti_core::prelude::*;

resource!(Items {
    get(request, ctx) => {
        let table = ctx.get_table("Items")?;
        let id = ctx.require_id()?;
        let item = table.get_by_id(id).await?;
        match item {
            Some(data) => ok_json!(data),
            None => reply().status(404).json(json!({"error": "Not found"})),
        }
    },
    post(request, ctx) => {
        let body = request.json_value()?;
        let name = body.require_str("name")?;
        let table = ctx.get_table("Items")?;
        table.put(&name, body.clone()).await?;
        created(body)
    }
});
```

### Resource with Options

```rust
// Custom endpoint name (URL path differs from struct name)
resource!(MyHandler {
    name = "custom-path",
    get => json!({"data": "served at /app/custom-path"})
});

// Default/catch-all resource (handles all unmatched paths)
resource!(Fallback {
    default = true,
    get(request, ctx) => {
        let path = ctx.path_id().unwrap_or("/");
        reply().status(404).json(json!({"error": "Not found", "path": path}))
    }
});

// Both options combined
resource!(CatchAll {
    name = "fallback",
    default = true,
    get => reply().text("Not found")
});
```

## ResourceParams API

The context parameter (`ctx` in examples above) is a `ResourceParams` that provides access to the application environment.

### Table Access

```rust
// Get a table handle by name
let table = ctx.get_table("Product")?;

// Read a record
let record = table.get_by_id("prod-123").await?;

// Write a record
table.put("prod-123", json!({"id": "prod-123", "name": "Widget"})).await?;
```

### Path Parameters

```rust
// Get path ID (from URL: /Resource/{id}) -- returns Option<&str>
let id = ctx.path_id();

// Get path ID or return 400 error
let id = ctx.require_id()?;

// Alias for path_id()
let id = ctx.id();
```

### Configuration Access

```rust
// Read from the app's config.yaml custom fields
let origin_url = ctx.config().get_str("origin.url", "https://default.com");
let timeout = ctx.config().get_i64("api.timeout", 30);
let enabled = ctx.config().get_bool("features.cache", false);
```

### Response Headers

```rust
ctx.response_headers().append("x-cache", "HIT");
ctx.response_headers().append("Set-Cookie", "session=abc123; Path=/; HttpOnly");
ctx.response_headers().set("X-Custom-Header", "value");
```

## Request Parsing

The `request` parameter is a standard `http::Request<Vec<u8>>`.

```rust
// Parse JSON body
let body = request.json_value()?;

// Extract required fields from JSON
let name = body.require_str("name")?;       // Returns Result<String>
let email = body.require_str("email")?;

// Access optional fields
let bio = body.get("bio").and_then(|v| v.as_str());
```

## Response Helpers

### JSON Responses

```rust
// 200 OK with JSON (macro form)
ok_json!({"status": "ok", "count": 42})

// 200 OK with JSON (function form)
ok_json!(data)

// 201 Created
created(json!({"id": "new-123"}))
created_json!({"id": "new-123"})

// Custom status with JSON
reply().status(201).json(&data)
reply().status(404).json(json!({"error": "Not found"}))
```

### Other Content Types

```rust
// HTML response
ok_html("<h1>Hello, World!</h1>")

// Plain text
reply().text("Hello, World!")

// Custom headers and status
reply()
    .status(200)
    .header("x-cache", "HIT")
    .header("x-request-id", "req-456")
    .json(json!({"message": "Hello"}))

// Redirect
reply().redirect("/new-location", Some(302))
```

## Default Resources

A default resource acts as a catch-all handler. When `is_default()` returns `true`, the resource receives all requests that do not match other routes. The requested path is available via `ctx.path_id()` or `ctx.id()`.

```rust
use yeti_core::prelude::*;

pub struct PageCache;

impl Default for PageCache {
    fn default() -> Self { Self }
}

impl Resource for PageCache {
    fn name(&self) -> &str { "PageCache" }

    fn is_default(&self) -> bool { true }

    fn get(&self, _request: Request<Vec<u8>>, ctx: ResourceParams) -> ResourceFuture {
        async_handler!({
            let path = ctx.id().unwrap_or("/");
            let cache = ctx.get_table("PageCache")?;

            if let Some(cached) = cache.get_by_id(path).await? {
                ctx.response_headers().append("x-cache", "HIT");
                return ok_html(cached.as_str().unwrap_or_default());
            }

            ctx.response_headers().append("x-cache", "MISS");
            reply().status(404).text("Not cached")
        })
    }
}

register_resource!(PageCache);
```

## Registration

When using the `resource!` macro, registration is automatic. For manual implementations, add `register_resource!()` at the end of the file:

```rust
register_resource!(PageCache);
```

This macro generates the C ABI entry point that the plugin loader calls to discover your resource.

## Supported HTTP Methods

The `resource!` macro and `Resource` trait support: `get`, `post`, `put`, `patch`, `delete`, and `search`. Unimplemented methods return 405 Method Not Allowed by default.
