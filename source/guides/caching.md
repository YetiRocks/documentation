# Caching & Performance

Yeti provides several caching and performance mechanisms at different layers of the stack. This guide gives an overview of each approach and when to use it.

---

## Table-Level Caching

When `storage.caching` is enabled in `yeti-config.yaml` (the default), Yeti maintains an in-memory read cache for table data. This cache sits in front of RocksDB and dramatically reduces disk I/O for read-heavy workloads.

```yaml
# yeti-config.yaml
storage:
  caching: true
```

Writes automatically invalidate the corresponding cache entries. The cache is per-table and uses LRU eviction. No additional configuration is required -- it is transparent to application code.

---

## Full-Page Caching

For HTTP content caching, use the full-page cache pattern. This stores entire HTTP responses (HTML, JSON, etc.) keyed by URL path. On cache hit, the stored content is returned immediately without contacting the origin. On cache miss, the origin is fetched and the result is stored for subsequent requests.

This pattern is implemented by the `full-page-cache` example application and can be adapted for any content-caching use case.

See [Full-Page Caching](full-page-cache.md) for implementation details.

---

## Table Expiration (TTL)

Tables can be configured with automatic expiration so that records are deleted after a specified time period. This is useful for session storage, temporary caches, and rate-limiting data that should not persist indefinitely.

```graphql
type Session @table(expiration: 3600) @export {
    id: ID! @primaryKey
    userId: String!
    token: String!
}
```

Records in this table are automatically removed by RocksDB TTL compaction after 3600 seconds (1 hour).

See [Table Expiration](table-expiration.md) for configuration details.

---

## Rate Limiting

Yeti includes server-level rate limiting to protect against abuse and ensure fair resource usage. Rate limits are configured globally in `yeti-config.yaml`:

```yaml
rateLimiting:
  maxRequestsPerSecond: 1000
  maxConcurrentConnections: 100
  maxStorageGB: 10
```

The backpressure layer returns HTTP 503 when the server is overloaded.

See [Rate Limiting](rate-limiting.md) for all configuration options.

---

## Compression

Responses larger than the configured `compressionThreshold` are automatically compressed using gzip. This reduces bandwidth for large JSON responses and static file serving.

```yaml
http:
  compressionThreshold: 1024  # bytes
```

Clients must send `Accept-Encoding: gzip` to receive compressed responses.

---

## Performance Tuning Checklist

1. **Enable caching** -- Ensure `storage.caching: true` (default).
2. **Index selectively** -- Only `@indexed` fields you filter on. Each index slows writes.
3. **Use TTL for ephemeral data** -- Prevent unbounded table growth.
4. **Set appropriate rate limits** -- Protect against runaway clients.
5. **Tune thread count** -- Set `threads.count` to match your CPU cores.
6. **Enable compression** -- Set a reasonable `compressionThreshold` for API traffic.
7. **Monitor with telemetry** -- Use the yeti-telemetry dashboard to identify slow queries.

---

## Performance Characteristics

| Operation | Throughput (no indexes) | Notes |
|-----------|------------------------|-------|
| Read | 186K ops/s | Direct RocksDB with caching |
| Create | 82K ops/s | Single write path |
| Mixed (70R/30W) | 156K ops/s | Typical workload pattern |
| With 1 index | 25K creates/s | Trade-off for query speed |
| With 2 indexes | 15K creates/s | Only index what you filter on |

See [Performance Benchmarks](../reference/benchmarks.md) for detailed measurements.

---

## See Also

- [Full-Page Caching](full-page-cache.md) -- HTTP content caching pattern
- [Table Expiration](table-expiration.md) -- Automatic record TTL
- [Rate Limiting](rate-limiting.md) -- Server-level throttling
- [Server Configuration](../reference/server-config.md) -- All server settings
