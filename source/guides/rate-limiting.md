# Rate Limiting

Yeti includes server-level rate limiting and backpressure to protect against abuse, prevent resource exhaustion, and ensure fair usage across clients.

---

## Server Configuration

Rate limiting is configured in `yeti-config.yaml` under the `rateLimiting` section:

```yaml
rateLimiting:
  maxRequestsPerSecond: 1000       # Maximum requests per second
  maxConcurrentConnections: 100    # Maximum simultaneous connections
  maxStorageGB: 10                 # Maximum storage per tenant

  ai:
    maxClaudeRequestsPerHour: 100       # AI generation rate limit
    maxEmbeddingRequestsPerHour: 1000   # Embedding generation rate limit
```

---

## Request Rate Limiting

The `maxRequestsPerSecond` setting caps the overall request throughput. When this limit is reached, additional requests receive an HTTP `429 Too Many Requests` response.

This is a server-wide limit. It applies to all applications and all clients combined.

---

## Connection Limiting

The `maxConcurrentConnections` setting limits the number of simultaneous TCP connections the server accepts. Connections beyond this limit are refused at the transport layer.

This protects against connection exhaustion attacks (slowloris, etc.) and ensures the server has headroom for legitimate traffic.

---

## Backpressure (In-Flight Requests)

The HTTP server includes a backpressure layer controlled by the `maxInFlightRequests` setting:

```yaml
http:
  maxInFlightRequests: 500
```

When the number of in-flight (currently processing) requests exceeds this threshold, the server returns HTTP `503 Service Unavailable`. This prevents cascading failures when the server is overloaded.

Unlike rate limiting (which caps throughput), backpressure limits concurrency. A server processing slow requests can hit the in-flight limit well below the per-second rate limit.

---

## Storage Quotas

The `maxStorageGB` setting caps the total storage consumed by a tenant. When the limit is reached, write operations (POST, PUT, PATCH) return an error. Read operations continue to work.

---

## AI Rate Limits

For applications that use AI features (Claude for content generation, Voyage for embeddings), separate rate limits prevent runaway costs:

```yaml
rateLimiting:
  ai:
    maxClaudeRequestsPerHour: 100
    maxEmbeddingRequestsPerHour: 1000
```

These are per-hour sliding windows.

---

## HTTP Timeout Settings

Related settings that affect request lifecycle:

```yaml
http:
  timeout: 120000            # Request timeout in ms (default: 2 minutes)
  keepAliveTimeout: 30000    # Keep-alive timeout in ms (default: 30 seconds)
```

Requests exceeding the `timeout` are terminated. The `keepAliveTimeout` controls how long idle connections are kept open.

---

## Error Responses

When rate limits are exceeded, the server returns structured error responses:

**429 Too Many Requests:**
```json
{
  "error": "Rate limit exceeded. Maximum 1000 requests per second."
}
```

**503 Service Unavailable:**
```json
{
  "error": "Server overloaded. Please retry later."
}
```

Clients should implement exponential backoff when receiving these responses.

---

## Monitoring Rate Limits

Use the yeti-telemetry dashboard to monitor rate limit hits:

```bash
# Check telemetry logs for rate limit events
curl -sk https://localhost:9996/yeti-telemetry/Log?filter=message==*rate*limit*
```

---

## Production Recommendations

| Setting | Development | Production |
|---------|-------------|------------|
| `maxRequestsPerSecond` | 1000 | Tune to hardware capacity |
| `maxConcurrentConnections` | 100 | 500-5000 depending on workload |
| `maxInFlightRequests` | 500 | Match to thread pool capacity |
| `maxStorageGB` | 10 | Size per tenant requirements |

---

## See Also

- [Caching & Performance](caching.md) -- Performance overview
- [Server Configuration](../reference/server-config.md) -- Complete server settings
- [Telemetry & Observability](telemetry.md) -- Monitoring rate limit events
