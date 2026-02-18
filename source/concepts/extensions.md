# Extensions

Extensions provide shared services to multiple applications. Where a regular application is self-contained, an extension makes its capabilities -- authentication, telemetry, custom middleware -- available to any application that opts in.

## Built-in Extensions

Yeti ships with two extensions:

| Extension | App ID | Purpose |
|-----------|--------|---------|
| **yeti-auth** | `yeti-auth` | Authentication (Basic, JWT, OAuth) and role-based access control |
| **yeti-telemetry** | `yeti-telemetry` | Observability: log collection, span tracing, metrics, and a real-time dashboard |

Both are standard Yeti applications with `extension: true` in their config. You can disable, replace, or supplement them with your own extensions.

## Creating an Extension

An extension is an application whose config sets `extension: true` and whose resources include a struct implementing the `Extension` trait:

```yaml
name: "My Extension"
app_id: "my-extension"
version: "1.0.0"
enabled: true
extension: true
schemas:
  - schema.graphql
resources:
  - resources/*.rs
```

The extension resource implements the `Extension` trait:

```rust
use yeti_core::prelude::*;

pub struct MyServiceExtension;

impl Extension for MyServiceExtension {
    fn name(&self) -> &str {
        "my-service"
    }

    fn initialize(&self) -> Result<()> {
        eprintln!("[my-service] Extension initialized");
        Ok(())
    }

    fn on_ready(&self, ctx: &ExtensionContext) -> Result<()> {
        // Access tables, set up event subscribers, etc.
        if let Some(table) = ctx.table("config") {
            eprintln!("[my-service] Config table available");
        }
        Ok(())
    }
}
```

The compiler auto-detects the extension type by scanning source files for `struct {Type}Extension` -- no additional configuration field is needed.

## The Extension Trait

The `Extension` trait defines the extension lifecycle:

```rust
pub trait Extension: Send + Sync {
    fn name(&self) -> &str;

    fn initialize(&self) -> Result<()> { Ok(()) }

    fn middleware(&self) -> Option<Arc<dyn RequestMiddleware>> { None }

    fn auth_providers(&self) -> Vec<Arc<dyn AuthProvider>> { Vec::new() }

    fn auth_hooks(&self) -> Vec<Arc<dyn AuthHook>> { Vec::new() }

    fn on_ready(&self, ctx: &ExtensionContext) -> Result<()> { Ok(()) }
}
```

| Method | Purpose |
|--------|---------|
| `name()` | Unique identifier for the extension |
| `initialize()` | Called once at registration (early setup) |
| `middleware()` | Return request middleware that runs before handlers |
| `auth_providers()` | Return authentication providers (Basic, JWT, OAuth, etc.) |
| `auth_hooks()` | Return hooks that can override default role resolution |
| `on_ready()` | Called after routes and tables are registered |

## ExtensionContext

The `on_ready` method receives an `ExtensionContext` with these methods:

| Method | Purpose |
|--------|---------|
| `ctx.table("name")` | Get an `Arc<TableResource>` by name |
| `ctx.root_dir()` | Get the root directory path |
| `ctx.auto_router()` | Access the AutoRouter for host-side table lookup |
| `ctx.set_event_subscriber(sub)` | Register a telemetry event handler |

## Consumer Configuration

Applications opt in to extensions via the `extensions:` field in their `config.yaml`. Each entry can include inline configuration specific to that application:

```yaml
# web-auth-demo/config.yaml
extensions:
  - yeti-auth:
      oauth:
        rules:
          - strategy: provider
            pattern: "google"
            role: admin
          - strategy: email
            pattern: "*@mycompany.com"
            role: standard
          - strategy: provider
            pattern: "github"
            role: standard
```

The inline config is accessible from resources via `ctx.extension_config("yeti-auth")`, allowing each app to customize extension behavior independently. For example, one app might map Google OAuth users to admin while another maps them to read-only.

## Extension Lifecycle

Extensions follow a specific load order:

1. **Discovery** -- Extension apps (with `extension: true`) are identified during application scanning.
2. **Load first** -- Extension dylibs are compiled and loaded before regular applications. Telemetry extensions are sorted to load first among extensions so they capture startup events.
3. **Initialize** -- `initialize()` is called once at registration for early setup.
4. **Routes registered** -- The extension's own tables and resources are registered in the router.
5. **on_ready** -- Called after all routes/tables are in place. This is where extensions wire up providers, start background services, and register event subscribers.
6. **Merge into consumers** -- Only declared extensions' tables and auth providers are merged into each consuming application's context.

## Dylib Boundary Rules

Extensions compile as dynamic libraries, which imposes important constraints:

- **No `tokio::spawn`** -- The dylib has its own Tokio runtime copy. Spawning tasks corrupts the host runtime. Use `set_event_subscriber()` to hand off background work to the host.
- **No `tracing::info!`** -- Tracing macros are silently lost due to TLS isolation. Use `eprintln!` for logging from extension code, or `yeti_log!` for bridged logging.
- **No host statics** -- `OnceLock` and other statics are duplicated in the dylib. Use dylib-local statics for state that must be shared between the extension and its resources.
- **Flag-based patterns** -- For anything requiring host runtime operations, set flags in `on_ready()` and let the host check them after the call returns.

These constraints apply to all code executing in dylib context, including methods on host-defined types when called from the dylib.

## Example: Auth Extension Config

The `yeti-auth` extension supports per-app OAuth role mapping with ordered rules (first match wins):

```yaml
extensions:
  - yeti-auth:
      oauth:
        default_role: "viewer"      # Fallback for unmatched users (omit to deny)
        rules:
          - strategy: provider      # Match by OAuth provider name
            pattern: "google"
            role: admin
          - strategy: email         # Match by email pattern (wildcard)
            pattern: "*@corp.com"
            role: standard
```
