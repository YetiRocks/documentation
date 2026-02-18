# Building Extensions

Extensions add shared services to the Yeti platform -- authentication, telemetry, custom middleware, and more. They are compiled as dylib plugins and loaded at startup, providing capabilities that applications can opt into.

## What Extensions Do

Extensions can:
- Register **auth providers** (Basic, JWT, OAuth).
- Register **middleware** that intercepts requests before they reach resources.
- Register **auth hooks** that override default role resolution.
- Provide an **event subscriber** for processing tracing events.
- Access **tables** from the host runtime for reading and writing data.

## Extension Trait

Every extension implements the `Extension` trait:

```rust
use yeti_core::prelude::*;

pub struct MyExtension;

impl Extension for MyExtension {
    fn name(&self) -> &str {
        "my-extension"
    }

    fn initialize(&self) -> Result<()> {
        eprintln!("[my-extension] Initializing");
        Ok(())
    }

    fn on_ready(&self, ctx: &ExtensionContext) -> Result<()> {
        eprintln!("[my-extension] Ready");

        // Access tables provided by this extension
        if let Some(table) = ctx.table("my-table") {
            eprintln!("[my-extension] Found table: {}", table.table_name());
        }

        Ok(())
    }
}
```

## Auto-Detection

The Yeti compiler auto-detects extensions by scanning source files for structs matching the pattern `struct {Name}Extension`. No configuration field is needed -- the compiler identifies extensions automatically.

## Application Configuration

Mark an application as an extension in its `config.yaml`:

```yaml
name: "My Extension"
app_id: "my-extension"
extension: true
schemas:
  - schema.graphql
resources:
  - resources/*.rs
```

Applications opt in to an extension by listing it in their `extensions:` config with an ordered list:

```yaml
# consumer-app/config.yaml
extensions:
  - my-extension: {}
  - yeti-auth:
      oauth:
        rules:
          - strategy: provider
            pattern: "github"
            role: standard
```

Extension order matters -- they are initialized and their middleware runs in the order listed.

## Critical Dylib Rules

Extensions compile as dynamic libraries (`.dylib`) that are loaded into the host process. The dylib boundary creates important constraints:

### Do NOT Use in on_ready()

- **`tokio::spawn()`** -- Crashes with "Rust cannot catch foreign exceptions". The dylib has its own tokio runtime copy.
- **`tracing::info!()` and other tracing macros** -- Messages do not reach the host log due to TLS (thread-local storage) isolation.
- **`tokio::sync::mpsc::channel()`** -- Channels created in dylib context do not work with the host runtime.
- **Host statics (`OnceLock`, etc.)** -- The dylib has its own copy of every static, separate from the host's copy.

### Safe to Use in on_ready()

- **`eprintln!()`** -- Bypasses tracing TLS isolation, output appears in the console.
- **`ctx.table(name)`** -- Returns `Arc<TableResource>` clones that work across the boundary.
- **`ctx.set_event_subscriber()`** -- Stores a handler that the host spawns after `on_ready()` returns.
- **`ctx.root_dir()`** -- Returns the runtime root directory path.
- **Pure functions** -- serde serialization, UUID generation, string manipulation, etc.

### The Pattern: Flags, Not Tasks

Instead of spawning async tasks in `on_ready()`, use the flag-based pattern:

1. The extension sets state and stores handler objects via `ExtensionContext` methods.
2. `on_ready()` returns.
3. The host code inspects the state and performs tokio operations (channel creation, task spawning) in host context.

## Providing Auth

Extensions can provide authentication providers:

```rust
impl Extension for MyAuthExtension {
    fn name(&self) -> &str {
        "my-auth"
    }

    fn auth_providers(&self) -> Vec<Arc<dyn AuthProvider>> {
        vec![
            Arc::new(BasicAuthProvider::new()),
            Arc::new(JwtAuthProvider::new("secret".to_string())),
        ]
    }

    fn auth_hooks(&self) -> Vec<Arc<dyn AuthHook>> {
        vec![Arc::new(CustomRoleResolver::new())]
    }
}
```

## Providing Middleware

```rust
impl Extension for MyMiddleware {
    fn name(&self) -> &str {
        "my-middleware"
    }

    fn middleware(&self) -> Option<Arc<dyn RequestMiddleware>> {
        Some(Arc::new(RateLimiter::new(100))) // 100 req/sec
    }
}
```

## Event Subscriber

For background processing of tracing events (logs, spans), implement `EventSubscriber`:

```rust
impl Extension for TelemetryExtension {
    fn on_ready(&self, ctx: &ExtensionContext) -> Result<()> {
        let log_table = ctx.table("log");
        let subscriber = Box::new(MyEventHandler { log_table });
        ctx.set_event_subscriber(subscriber);
        Ok(())
    }
}
```

See [Extension Lifecycle](extension-lifecycle.md) for the full initialization sequence.

## See Also

- [Extension Lifecycle](extension-lifecycle.md) -- Detailed initialization order
- [Telemetry & Observability](telemetry.md) -- Event subscriber example
- [Authentication Overview](auth-overview.md) -- Auth provider integration
