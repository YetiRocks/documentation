# Telemetry & Observability

Yeti provides a complete telemetry pipeline through the `yeti-telemetry` extension. The core runtime captures tracing events and forwards them as JSON to the extension, which persists logs, spans, and metrics to tables, streams them over SSE, and optionally exports to OpenTelemetry collectors.

---

## Architecture

```
tracing::info!(...)
      |
      v
  DispatchLayer (core)         Captures tracing events as JSON
      |
      v
  mpsc::UnboundedChannel       Event channel (host-created)
      |
      v
  EventSubscriber (extension)  Processes events
      |
      +---> Log table           Persistent log storage
      +---> Span table          Persistent span storage
      +---> Metric table        Persistent metric storage
      +---> SSE streams         Real-time event streaming
      +---> OTLP export         OpenTelemetry collector (optional)
```

The core runtime contains only the `DispatchLayer` (~260 lines). All processing, persistence, and export logic lives in the yeti-telemetry extension.

---

## Tables

The extension uses three tables in the `yeti-telemetry` database:

```graphql
type Log @table(database: "yeti-telemetry") @export(sse: true) {
  id: ID! @primaryKey
  timestamp: String! @indexed
  level: String! @indexed
  target: String! @indexed
  message: String!
  fields: String
}

type Span @table(database: "yeti-telemetry") @export(sse: true) {
  id: ID! @primaryKey
  traceId: String @indexed
  name: String! @indexed
  target: String!
  level: String!
  startTime: String!
  endTime: String
  durationMs: Float
  fields: String
}

type Metric @table(database: "yeti-telemetry") @export(sse: true) {
  id: ID! @primaryKey
  name: String! @indexed
  value: Float!
  attributes: String
  timestamp: String!
}
```

---

## Querying Telemetry Data

### REST API

```bash
# Get recent log entries
curl -sk "https://localhost:9996/yeti-telemetry/Log?limit=50&sort=-timestamp"

# Filter by log level
curl -sk "https://localhost:9996/yeti-telemetry/Log?filter=level==ERROR"

# Get spans for a trace
curl -sk "https://localhost:9996/yeti-telemetry/Span?filter=traceId==abc-123"

# Get metrics by name
curl -sk "https://localhost:9996/yeti-telemetry/Metric?filter=name==request_count"
```

### SSE Streaming

Stream events in real time using Server-Sent Events:

```bash
# Stream all log events
curl -sk "https://localhost:9996/yeti-telemetry/Log?stream=sse"

# Stream span events
curl -sk "https://localhost:9996/yeti-telemetry/Span?stream=sse"

# Stream metric events
curl -sk "https://localhost:9996/yeti-telemetry/Metric?stream=sse"
```

---

## Dashboard

The yeti-telemetry extension serves a built-in dashboard with real-time log streaming:

```
https://localhost:9996/yeti-telemetry/
```

The dashboard shows live logs, allows filtering by level and target, and provides status information about the telemetry pipeline.

---

## OTLP Export

To export telemetry data to an OpenTelemetry collector (Jaeger, Grafana, Datadog, etc.), configure the OTLP endpoint:

```yaml
# yeti-config.yaml
telemetry:
  otlpEndpoint: "http://localhost:4317"
  serviceName: yeti
```

Or set via environment variable:

```bash
export OTEL_EXPORTER_OTLP_ENDPOINT="http://localhost:4317"
```

The extension uses lazy initialization for the OTLP meter provider, connecting only when the endpoint is configured.

---

## Feedback Prevention

The `DispatchLayer` filters events from internal infrastructure targets to prevent infinite recursion. When the subscriber writes to the Log table, that write generates tracing events. Without filtering, these would be captured and written again, creating an infinite loop.

Filtered targets:
- `yeti_core::pubsub`
- `yeti_core::backend`
- `yeti_core::platform::telemetry`
- `yeti_core::resource::table`
- `yeti_core::http::sse`

---

## Extension Load Order

Telemetry extensions are sorted first in the extension loading sequence so that the event subscriber is registered before other extensions start up. This ensures startup logs from other extensions (like yeti-auth) are captured.

---

## Custom Telemetry

To replace yeti-telemetry with your own implementation:

1. Create a new extension with `extension: true` in its `config.yaml`
2. Implement the `EventSubscriber` trait
3. Register it via `ctx.set_event_subscriber()` in `on_ready()`
4. Disable yeti-telemetry by setting `enabled: false` in its config

Without any telemetry extension loaded, the `DispatchLayer` is a no-op and only stdout logging works.

---

## Configuration

```yaml
# yeti-config.yaml
telemetry:
  metrics: true           # Enable metrics collection
  tracing: false          # Enable distributed tracing
  auditLog: true          # Enable audit logging
  serviceName: yeti       # Service name for telemetry reports
  # otlpEndpoint: ""      # OpenTelemetry endpoint (optional)

logging:
  level: info             # Log level: error, warn, info, debug, trace
  auditLog: true          # Enable audit logging
```

---

## See Also

- [Event Subscribers](event-subscribers.md) -- EventSubscriber trait details
- [Server-Sent Events](sse.md) -- SSE streaming guide
- [Server Configuration](../reference/server-config.md) -- Full config reference
