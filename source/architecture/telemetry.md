# Telemetry Pipeline

Yeti's telemetry architecture separates concerns cleanly: the core provides a minimal event channel, while all processing and storage lives in an optional extension.

## Design Principles

- **Zero OpenTelemetry in core** -- No otel crate dependencies, no feature flags.
- **Core is ~260 lines** -- Single file: `src/platform/telemetry.rs`.
- **Extension-owned processing** -- All log writing, file rotation, and OTLP export lives in `yeti-telemetry`.
- **Pluggable** -- Delete yeti-telemetry and create your own extension implementing `EventSubscriber`.

## Pipeline Architecture

```
tracing::info!("message")
        │
        ▼
  DispatchLayer              (tracing subscriber layer)
        │
        ▼
  JSON serialization         {"kind":"log", "timestamp":..., "level":..., ...}
        │
        ▼
  UnboundedChannel           (tokio mpsc)
        │
        ▼
  EventSubscriber::run()     (implemented by extension)
        │
        ├──> Log/Span/Metric tables    (RocksDB persistence)
        ├──> FileProvider              (JSONL file rotation)
        ├──> SSE streams               (real-time dashboard)
        └──> OtlpOutput               (optional Grafana/Datadog export)
```

## DispatchLayer

The `DispatchLayer` is a `tracing` subscriber layer that intercepts all log events and span lifecycle events. It serializes them to JSON and sends them through an unbounded channel.

### Event Format

```json
{
  "kind": "log",
  "timestamp": 1700000000.123,
  "level": "INFO",
  "target": "yeti_core::routing",
  "message": "Request processed",
  "fields": {"method": "GET", "path": "/api/users", "status": 200}
}
```

```json
{
  "kind": "span",
  "timestamp": 1700000000.456,
  "name": "process_request",
  "target": "yeti_core::http",
  "level": "DEBUG",
  "duration_ms": 12.5
}
```

### Feedback Filter

The DispatchLayer skips events from these targets to prevent recursion (telemetry writes generating more telemetry events):

- `yeti_core::pubsub`
- `yeti_core::backend`
- `yeti_core::telemetry`
- `yeti_core::resource::table`
- `yeti_core::http::sse`

## EventSubscriber Trait

Extensions implement this trait to receive telemetry events:

```rust
pub trait EventSubscriber: Send + 'static {
    fn run(self: Box<Self>, rx: UnboundedReceiver<Value>);
}
```

The host creates the channel, registers the sender with `DispatchLayer`, and spawns the subscriber's `run()` method after `on_ready()` returns.

## Extension Load Order

Telemetry extensions are sorted first in the extension loading order. This ensures the event subscriber is active before other extensions start up, capturing their initialization logs.

## yeti-telemetry Extension

The built-in telemetry extension provides:

| Component | Purpose |
|-----------|---------|
| `TelemetryWriter` | Receives events and routes to outputs |
| `FileProvider` | JSONL file rotation (date-based) |
| `OtlpOutput` | OpenTelemetry Protocol export |
| Log/Span/Metric tables | Persistent storage with SSE streaming |
| Dashboard UI | Web interface at `/yeti-telemetry/` |

## OTLP Export

Configure in `yeti-config.yaml`:

```yaml
telemetry:
  metrics: true
  otlpEndpoint: "http://localhost:4317"
  serviceName: "yeti"
```

The extension lazily initializes an OpenTelemetry meter provider when `otlpEndpoint` is set.

## Without an Extension

If no extension implements `EventSubscriber`, the `DispatchLayer` becomes a no-op. Only stdout logging (via the core's `init_stdout()`) remains active.
