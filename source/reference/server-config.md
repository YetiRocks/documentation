# Server Configuration

Complete reference for `yeti-config.yaml`, the server-level configuration file located in the root directory (default: `~/yeti/yeti-config.yaml`).

---

## environment

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `environment` | string | `"development"` | Runtime environment. Values: `development`, `production`, `test`. Affects TLS validation, logging verbosity, and default settings. |

---

## rootDirectory

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `rootDirectory` | string | `"~/yeti"` | Root directory for all Yeti data. Applications, databases, certificates, and caches are stored relative to this path. Can be overridden with `--root-dir` CLI argument. |

---

## http

Application API server settings (default port 9996, HTTPS).

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `http.port` | integer | `9996` | HTTP/HTTPS port for the application API |
| `http.securePort` | integer | `9996` | HTTPS port (typically same as `port` since Yeti serves HTTPS by default) |
| `http.cors` | boolean | `true` | Enable CORS headers on all responses |
| `http.corsAccessList` | string[] | `["*"]` | Allowed CORS origins. Use `["*"]` for development, restrict in production |
| `http.timeout` | integer | `120000` | Request timeout in milliseconds |
| `http.keepAliveTimeout` | integer | `30000` | Keep-alive connection timeout in milliseconds |
| `http.compressionThreshold` | integer | `1024` | Compress responses larger than this size in bytes. Requires client `Accept-Encoding: gzip` |
| `http.maxConnectionRate` | integer | `256` | Maximum new connections accepted per second |
| `http.maxInFlightRequests` | integer | `500` | Maximum concurrent requests. Returns 503 when exceeded |
| `http.disconnectTimeout` | integer | `5000` | Timeout for graceful connection shutdown in milliseconds |

---

## operationsApi

Administrative API settings (default port 9995, HTTP without TLS).

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `operationsApi.port` | integer | `9995` | Operations API port |
| `operationsApi.enabled` | boolean | `true` | Enable or disable the operations API |
| `operationsApi.cors` | boolean | `true` | Enable CORS for the operations API |
| `operationsApi.corsAccessList` | string[] | `["*"]` | CORS allowed origins |
| `operationsApi.requireAuth` | boolean | `false` | Require authentication for operations API (reserved for future use) |

---

## storage

Default storage settings for all applications. Yeti supports two storage modes: **embedded** (single-node, default) and **cluster** (distributed).

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `storage.mode` | string | `"embedded"` | Storage mode: `"embedded"` (single-node) or `"cluster"` (distributed) |
| `storage.caching` | boolean | `true` | Enable in-memory read cache for table data |
| `storage.compression` | boolean | `true` | Enable data compression |
| `storage.path` | string | `null` | Custom storage path. `null` uses `$rootDirectory/data/` |

### storage.cluster

Cluster settings. Only used when `storage.mode` is `"cluster"`.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `storage.cluster.pdEndpoints` | string[] | `[]` | Placement driver endpoints for the cluster |
| `storage.cluster.tlsCaPath` | string | `null` | Path to CA certificate for mTLS |
| `storage.cluster.tlsCertPath` | string | `null` | Path to client certificate for mTLS |
| `storage.cluster.tlsKeyPath` | string | `null` | Path to client private key for mTLS |
| `storage.cluster.timeoutMs` | integer | `5000` | Timeout per operation in milliseconds |
| `storage.cluster.autoStart` | boolean | `false` | Auto-start Docker cluster on startup (development only) |

When `autoStart` is `true` in development, Yeti starts a cluster via Docker Compose. Cluster node hostnames must resolve in `/etc/hosts`.

---

## logging

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `logging.level` | string | `"info"` | Log level. Values: `error`, `warn`, `info`, `debug`, `trace` |
| `logging.auditLog` | boolean | `true` | Enable audit logging for data operations |

---

## threads

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `threads.count` | integer | `null` | Thread pool size. `null` uses the number of CPU cores |
| `threads.debug` | boolean | `false` | Enable thread debugging output |

---

## tls

TLS/HTTPS configuration. See [TLS & HTTPS](tls.md) for detailed setup.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `tls.autoGenerate` | boolean | `false` | Auto-generate self-signed certificates on startup |
| `tls.privateKey` | string | `null` | Path to PEM private key file |
| `tls.certificate` | string | `null` | Path to PEM certificate file |

When `autoGenerate` is `true`, self-signed certificates are generated at startup and stored in `$rootDirectory/certs/`. When manual paths are provided, those certificates are used. At least one TLS method must be configured for HTTPS to work.

---

## rateLimiting

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `rateLimiting.maxRequestsPerSecond` | integer | `1000` | Maximum requests per second (server-wide) |
| `rateLimiting.maxConcurrentConnections` | integer | `100` | Maximum simultaneous connections |
| `rateLimiting.maxStorageGB` | integer | `10` | Maximum storage per tenant in GB |
| `rateLimiting.ai.maxClaudeRequestsPerHour` | integer | `100` | AI generation rate limit per hour |
| `rateLimiting.ai.maxEmbeddingRequestsPerHour` | integer | `1000` | Embedding generation rate limit per hour |

---

## telemetry

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `telemetry.metrics` | boolean | `true` | Enable metrics collection |
| `telemetry.tracing` | boolean | `false` | Enable distributed tracing |
| `telemetry.auditLog` | boolean | `true` | Enable audit logging in telemetry |
| `telemetry.serviceName` | string | `"yeti"` | Service name for telemetry reports and OTLP export |
| `telemetry.otlpEndpoint` | string | `""` | OpenTelemetry collector endpoint (e.g., `http://localhost:4317`) |

---

## maintenance

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `maintenance.backup.enabled` | boolean | `true` | Enable automatic database backups |
| `maintenance.backup.intervalHours` | integer | `24` | Hours between automatic backups |
| `maintenance.backup.retentionDays` | integer | `30` | Number of days to retain backups |
| `maintenance.healthCheck.intervalSeconds` | integer | `30` | Health check polling interval |
| `maintenance.healthCheck.timeoutSeconds` | integer | `5` | Health check timeout |

---

## Example Configuration (Embedded)

```yaml
environment: production
rootDirectory: /opt/yeti

http:
  port: 9996
  cors: true
  corsAccessList:
    - "https://app.example.com"
  timeout: 30000
  compressionThreshold: 1024

operationsApi:
  port: 9995

storage:
  mode: embedded
  caching: true
  compression: true

logging:
  level: info
  auditLog: true

tls:
  privateKey: /etc/ssl/private/yeti.key
  certificate: /etc/ssl/certs/yeti.crt

rateLimiting:
  maxRequestsPerSecond: 5000
  maxConcurrentConnections: 1000

telemetry:
  serviceName: yeti-production
  otlpEndpoint: "http://otel-collector:4317"
```

## Example Configuration (Cluster)

```yaml
environment: production
rootDirectory: /opt/yeti

storage:
  mode: cluster
  caching: true
  compression: true
  cluster:
    pdEndpoints:
      - "pd1:23791"
      - "pd2:23792"
      - "pd3:23793"
    tlsCaPath: /etc/yeti/tls/ca.pem
    tlsCertPath: /etc/yeti/tls/client.pem
    tlsKeyPath: /etc/yeti/tls/client-key.pem
    timeoutMs: 5000

http:
  port: 9996

tls:
  privateKey: /etc/ssl/private/yeti.key
  certificate: /etc/ssl/certs/yeti.crt

telemetry:
  serviceName: yeti-cluster
  otlpEndpoint: "http://otel-collector:4317"
```

---

## See Also

- [CLI Arguments](cli.md) -- Command-line overrides
- [Environment Variables](environment-variables.md) -- Environment configuration
- [TLS & HTTPS](tls.md) -- Certificate setup
