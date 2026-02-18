# Event Subscribers

Event subscribers receive structured tracing events as JSON values over an unbounded channel. This is the mechanism by which extensions like yeti-telemetry capture log and span data from the core runtime.

---

## The EventSubscriber Trait

Extensions that want to receive tracing events implement the `EventSubscriber` trait:

```rust
use tokio::sync::mpsc;
use serde_json::Value;
use std::pin::Pin;
use std::future::Future;

pub trait EventSubscriber: Send + 'static {
    fn run(
        self: Box<Self>,
        rx: mpsc::UnboundedReceiver<Value>,
    ) -> Pin<Box<dyn Future<Output = ()> + Send>>;
}
```

The `run` method is called by the host runtime after `on_ready()` returns. The host creates the channel, registers the sender with the global `DispatchLayer`, and spawns the subscriber's future on the host tokio runtime.

---

## JSON Event Format

Every event sent through the channel is a `serde_json::Value` with one of two shapes:

### Log Events

```json
{
  "kind": "log",
  "timestamp": 1710000000000.0,
  "level": "INFO",
  "target": "my_app::handler",
  "message": "Request processed successfully",
  "fields": {
    "request_id": "abc-123",
    "duration_ms": "42"
  }
}
```

### Span Events

```json
{
  "kind": "span",
  "name": "handle_request",
  "target": "my_app::handler",
  "level": "INFO",
  "startTime": 1710000000000.0,
  "endTime": 1710000000042.0,
  "fields": {
    "method": "GET",
    "path": "/users"
  }
}
```

Fields are always string-valued in the JSON representation, even for numeric and boolean tracing fields.

---

## Registering a Subscriber

Extensions register their subscriber during `on_ready()` using the `ExtensionContext`:

```rust
fn on_ready(&self, ctx: &ExtensionContext) -> Result<()> {
    let log_table = ctx.table("log");
    let span_table = ctx.table("span");

    let subscriber = Box::new(MyTelemetrySubscriber {
        log_table,
        span_table,
    });

    ctx.set_event_subscriber(subscriber);
    Ok(())
}
```

Only one subscriber can be active at a time. The last extension to call `set_event_subscriber()` wins. If no subscriber is registered, the `DispatchLayer` silently drops all events.

---

## Implementing a Subscriber

A minimal subscriber that prints events to stderr:

```rust
pub struct DebugSubscriber;

impl EventSubscriber for DebugSubscriber {
    fn run(
        self: Box<Self>,
        mut rx: mpsc::UnboundedReceiver<Value>,
    ) -> Pin<Box<dyn Future<Output = ()> + Send>> {
        Box::pin(async move {
            while let Some(event) = rx.recv().await {
                let kind = event["kind"].as_str().unwrap_or("unknown");
                let msg = event["message"].as_str().unwrap_or("");
                eprintln!("[{}] {}", kind, msg);
            }
        })
    }
}
```

---

## Feedback Prevention

The `DispatchLayer` filters events from internal infrastructure targets to prevent infinite recursion. When a subscriber writes to a table, that write generates tracing events which would be captured again. Filtered targets include `yeti_core::pubsub`, `yeti_core::backend`, `yeti_core::resource::table`, and `yeti_core::http::sse`.

---

## Dylib Safety

The subscriber object is constructed in dylib context during `on_ready()` but executed by the host. This works safely because:

- The channel receiver is host-created and passed to `run()`
- `Arc<dyn KvBackend>` table references use vtable dispatch across the boundary
- No tokio channel creation or `spawn` calls happen inside `run()`

Do not call `tokio::spawn` or create channels inside the subscriber. Use `futures::stream::unfold` or loop over `rx.recv().await` directly.

---

## See Also

- [Building Extensions](building-extensions.md) -- Extension development overview
- [Extension Lifecycle](extension-lifecycle.md) -- Startup and initialization order
- [Telemetry & Observability](telemetry.md) -- The yeti-telemetry extension
