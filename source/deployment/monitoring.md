# Monitoring

Yeti provides multiple monitoring interfaces: the Operations API for health checks, the yeti-telemetry extension for log/span/metric persistence, and OTLP export for integration with external monitoring systems.

## Operations API

The Operations API runs on port 9995 (separate from the application server on 9996) and provides system-level health and configuration endpoints.

### Health Check

```bash
curl -s http://localhost:9995/health
```

Returns HTTP 200 when the server is running. Use this as a liveness probe in container orchestration.

### System Information

```bash
curl -s http://localhost:9995/system
```

Returns system details including:
- Uptime.
- Memory usage.
- CPU information.
- Loaded application count.

### Configuration

```bash
curl -s http://localhost:9995/config
```

Returns the active server configuration (secrets are redacted).

### Operations API Security

In production, secure the operations API:

```yaml
operationsApi:
  enabled: true
  port: 9995
  requireAuth: true
  cors: false
```

Consider firewall rules to restrict access to the operations port.

## Telemetry Extension

The `yeti-telemetry` extension provides persistent observability data.

### Log Monitoring

```bash
# Recent logs
curl -sk https://localhost:9996/yeti-telemetry/Log

# Error logs only
curl -sk "https://localhost:9996/yeti-telemetry/Log?level=ERROR"

# Logs from a specific module
curl -sk "https://localhost:9996/yeti-telemetry/Log?target=yeti_core::routing"
```

### Span Monitoring

```bash
# Recent spans
curl -sk https://localhost:9996/yeti-telemetry/Span

# Slow operations (sorted by duration)
curl -sk "https://localhost:9996/yeti-telemetry/Span?$sort=-durationMs&$limit=20"
```

### Metric Monitoring

```bash
# Recent metrics
curl -sk https://localhost:9996/yeti-telemetry/Metric

# Specific metric name
curl -sk "https://localhost:9996/yeti-telemetry/Metric?name=http_request_duration"
```

## Real-Time SSE Streams

For live monitoring, connect to SSE endpoints:

```bash
# Live log stream
curl -sk -N "https://localhost:9996/yeti-telemetry/Log?stream=sse"

# Live span stream
curl -sk -N "https://localhost:9996/yeti-telemetry/Span?stream=sse"

# Live metric stream
curl -sk -N "https://localhost:9996/yeti-telemetry/Metric?stream=sse"
```

These are standard Server-Sent Events and work with any SSE client library.

## Dashboard

The built-in dashboard at `https://localhost:9996/yeti-telemetry/` provides a web interface for real-time log monitoring with level filtering and search.

## OTLP Export

For integration with external monitoring systems (Grafana, Datadog, Jaeger, Prometheus), configure OTLP export:

```yaml
# yeti-config.yaml
telemetry:
  metrics: true
  tracing: true
  serviceName: "yeti-production"
  otlpEndpoint: "http://otel-collector:4317"
```

This exports telemetry data using the OpenTelemetry Protocol (OTLP) over gRPC to a collector endpoint. The collector can then forward to any supported backend.

### Grafana Stack Example

```
Yeti ──OTLP──> OpenTelemetry Collector ──> Grafana Loki (logs)
                                       ──> Grafana Tempo (traces)
                                       ──> Prometheus (metrics)
```

### Environment Variable

The OTLP endpoint can also be set via environment variable:

```bash
export OTEL_EXPORTER_OTLP_ENDPOINT="http://otel-collector:4317"
```

## Alerting Patterns

### Log-Based Alerts

Monitor the SSE stream for error patterns:

```bash
# Example: watch for errors and send to alerting
curl -sk -N "https://localhost:9996/yeti-telemetry/Log?stream=sse&level=ERROR" | \
  while read -r line; do
    # Parse and forward to alerting system
    echo "$line" | your-alert-script
  done
```

### Health Check Integration

Use the Operations API health endpoint with your monitoring tool:

```bash
# Nagios/Icinga style check
curl -sf http://localhost:9995/health > /dev/null && echo "OK" || echo "CRITICAL"
```

## Recommended Monitoring Setup

| Tier | Approach | Latency |
|------|----------|---------|
| Real-time | SSE streams or Dashboard | Instant |
| Near-real-time | OTLP export to Grafana | Seconds |
| Historical | Query Log/Span/Metric tables via REST | On-demand |
| Uptime | Operations API health check | Poll-based |
