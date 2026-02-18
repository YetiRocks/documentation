# Extension Lifecycle

This guide details the complete lifecycle of a Yeti extension, from source detection through runtime operation. Understanding this sequence is essential for building extensions that interact correctly with the host process.

## Lifecycle Stages

### 1. Compiler Detection

The Yeti compiler scans extension source files for structs matching the pattern `struct {Name}Extension`. This auto-detection means you do not need to declare the extension type in config -- the compiler infers it from the source code.

```rust
// The compiler finds this struct and registers it as an extension
pub struct TelemetryExtension {
    config: TelemetryConfig,
}
```

The compiler copies resource source files to `cache/builds/{app}/src/` and generates `lib.rs` with the appropriate entry points.

### 2. Dylib Compilation

The extension is compiled as a dynamic library (`.dylib` on macOS, `.so` on Linux). This compilation takes approximately 2 minutes per plugin on first build. Subsequent builds use cached artifacts and complete in approximately 10 seconds.

### 3. Dylib Loading

At startup, the host process loads each extension dylib using `dlopen`. This gives the extension access to yeti-core types via the shared ABI, but with separate copies of:
- Thread-local storage (including tracing subscribers)
- Static variables (`OnceLock`, `lazy_static`, etc.)
- The tokio runtime copy

### 4. Extension::initialize()

The host calls `initialize()` on each loaded extension. This is the earliest point where the extension can run code. Use this for lightweight setup that does not require access to tables or routes.

```rust
impl Extension for MyExtension {
    fn initialize(&self) -> Result<()> {
        eprintln!("[my-ext] Version 1.0 loaded");
        Ok(())
    }
}
```

### 5. Auth Provider Registration

The host calls `auth_providers()` and `auth_hooks()` to collect authentication components. These are registered with the `AuthPipeline` before any requests are processed.

```rust
fn auth_providers(&self) -> Vec<Arc<dyn AuthProvider>> {
    vec![
        Arc::new(BasicAuthProvider::new(self.backend.clone())),
        Arc::new(JwtAuthProvider::new(self.jwt_secret.clone())),
    ]
}
```

### 6. Routes and Tables Registered

The host's `AutoRouter` registers all of the extension's schema-defined tables and custom resources. At this point:
- Tables are created in the storage backend.
- REST and GraphQL routes are mapped.
- SSE and WebSocket endpoints are wired up.
- Seed data is loaded from `data/*.json` files.

### 7. on_ready() Called

This is the main setup hook. The `ExtensionContext` provides safe methods for accessing the fully-initialized runtime:

```rust
fn on_ready(&self, ctx: &ExtensionContext) -> Result<()> {
    // Access tables by lowercase name
    let log_table = ctx.table("log")
        .expect("Log table should be registered");
    let span_table = ctx.table("span");

    // Read the root directory
    let root = ctx.root_dir();
    eprintln!("[my-ext] Root directory: {}", root);

    // Register an event subscriber for async processing
    let handler = Box::new(MyEventHandler {
        log_table,
        span_table,
    });
    ctx.set_event_subscriber(handler);

    Ok(())
}
```

### 8. Host Spawns Event Subscriber

After `on_ready()` returns, the host code:

1. Calls `ctx.take_event_subscriber()` to retrieve the handler.
2. Creates a `mpsc::unbounded_channel()` in **host context** (safe for tokio).
3. Registers the channel sender with the global `DispatchLayer`.
4. Spawns the subscriber's `run()` method on the host's tokio runtime.

This sequence ensures all async operations happen in host context, avoiding dylib TLS corruption.

```rust
// This runs in HOST context (app_loader), not in the dylib
if let Some(subscriber) = ctx.take_event_subscriber() {
    let (tx, rx) = mpsc::unbounded_channel();
    register_subscriber(tx);
    tokio::spawn(subscriber.run(rx));
}
```

## ExtensionContext Methods

| Method | Returns | Description |
|--------|---------|-------------|
| `table(name)` | `Option<Arc<TableResource>>` | Get a table by lowercase name |
| `root_dir()` | `&str` | Runtime root directory path |
| `auto_router()` | `&Arc<AutoRouter>` | Access to the app's router |
| `set_event_subscriber(sub)` | -- | Store an event handler for the host to spawn |
| `app_id` | `String` | The application identifier |
| `root_directory` | `String` | The root directory path |

## Extension Load Order

Telemetry extensions are sorted first in `load_extensions` so the event subscriber is registered before other extensions start emitting tracing events during their `on_ready()` hooks. This ensures no startup events are lost.

## Complete Example

```rust
use yeti_core::prelude::*;
use std::sync::Arc;

pub struct MetricsExtension;

impl Extension for MetricsExtension {
    fn name(&self) -> &str {
        "metrics"
    }

    fn initialize(&self) -> Result<()> {
        eprintln!("[metrics] Extension loaded");
        Ok(())
    }

    fn middleware(&self) -> Option<Arc<dyn RequestMiddleware>> {
        Some(Arc::new(RequestCounter::new()))
    }

    fn on_ready(&self, ctx: &ExtensionContext) -> Result<()> {
        if let Some(metrics_table) = ctx.table("metric") {
            eprintln!("[metrics] Metric table available");
            let handler = Box::new(MetricsHandler { metrics_table });
            ctx.set_event_subscriber(handler);
        }
        Ok(())
    }
}
```

## Troubleshooting

**Plugin segfaults after cargo clean**: Clear the plugin cache at `$ROOT_DIR/cache/builds/*/target/`. Stale `.dylib` files with ABI mismatches cause silent segfaults.

**Source changes not taking effect**: Clear `cache/builds/{app}/src/` -- the compiler copies source there, and stale copies override your edits.

**tracing macros produce no output**: Use `eprintln!()` instead. Tracing macros in dylib context use the dylib's tracing subscriber, not the host's.

**tokio::spawn crashes**: Use `ctx.set_event_subscriber()` and let the host spawn tasks after `on_ready()` returns.

## See Also

- [Building Extensions](building-extensions.md) -- Extension trait overview
- [Telemetry & Observability](telemetry.md) -- Event subscriber example
- [PubSub](pubsub.md) -- Internal messaging used by extensions
