# Performance Tuning

This guide covers the key configuration parameters that affect Yeti's performance. Start with the defaults and adjust based on your workload characteristics.

## Build Profile

### Release Build

Always use release builds in production:

```bash
cargo build --release
```

### Production Profile

For maximum performance, use the production profile with fat LTO (Link-Time Optimization):

```bash
cargo build --profile production
```

Fat LTO produces a single, highly optimized binary at the cost of longer compile times. Expect 10-20% throughput improvement over a standard release build.

## Storage Tuning

### RocksDB (Embedded Mode)

#### Block Cache

RocksDB's block cache holds frequently accessed data in memory:

```rust
StorageConfig {
    cache_size_mb: 2048,       // Block cache (default)
    write_buffer_size_mb: 512, // Memtable size before flush
    enable_compression: false,
    sync_writes: true,
    disable_wal: false,
}
```

**Guidelines:**
- `cache_size_mb` -- Set to 10-20% of available RAM. More cache = fewer disk reads.
- `write_buffer_size_mb` -- Larger buffers batch more writes before flushing. Increase for write-heavy workloads.
- `sync_writes` -- Sync WAL writes (default true). Set `false` for 5-10x write throughput at the cost of durability.
- `disable_wal` -- Keep WAL enabled for data safety. Only disable for ephemeral caches.

#### Compression

```yaml
storage:
  compression: true
```

When enabled, LZ4 compression reduces storage size by 50-70% with minimal CPU overhead. Disable only if CPU is the bottleneck and storage is abundant.

### Cluster Mode

#### Hot Cache

Each table maintains a local LRU cache (10,000 entries, max 64KB per value). This reduces round-trips for frequently accessed records.

- Cache is write-through: updates are reflected immediately.
- Negative caching avoids repeated lookups for non-existent keys.
- Large values (>64KB) bypass the cache.

#### Connection Pool

The connection pool size is `max(num_cpus / 2, 2)` persistent gRPC channels. For high-throughput workloads, ensure the Yeti server has sufficient CPU cores.

#### Network Latency

Cluster mode adds ~1-2ms per operation compared to local embedded mode. To minimize latency:
- Co-locate Yeti and cluster nodes in the same data center or availability zone.
- Use batch operations (`get_batch`, `write_batch`) to amortize round-trip costs.
- Tune `timeoutMs` based on your network characteristics (default: 5000ms).

#### Cluster Sizing

| Component | Minimum | Recommended |
|-----------|---------|-------------|
| PD nodes | 3 | 3-5 |
| Storage nodes | 3 | 3+ (scale with data size) |
| Storage per node | SSD, 100GB | SSD, 500GB+ |
| RAM per node | 8GB | 16GB+ |

## HTTP Tuning

```yaml
http:
  timeout: 60000              # Request timeout (ms)
  keepAliveTimeout: 75000     # Keep-alive timeout (ms)
  disconnectTimeout: 5000     # Client disconnect timeout (ms)
  maxConnectionRate: 256      # New connections per second
  maxInFlightRequests: 10000  # Concurrent request limit
  compressionThreshold: 1024  # Compress responses > 1KB
```

### Key Parameters

| Parameter | Default | Tuning Guidance |
|-----------|---------|----------------|
| `maxInFlightRequests` | 10,000 | Increase for high-concurrency workloads; decrease to protect downstream resources |
| `maxConnectionRate` | 256 | Limits connection storms; increase for high-traffic services |
| `keepAliveTimeout` | 75s | Reduce for stateless APIs with many short-lived clients |
| `compressionThreshold` | 1024 | Lower to compress more responses (trades CPU for bandwidth) |
| `timeout` | 60s | Reduce for APIs where fast-fail is preferred |

## Thread Configuration

```yaml
threads:
  count: null    # null = auto-detect (CPU count)
```

Yeti uses Tokio's multi-threaded runtime. By default, it creates one worker thread per CPU core. Override only if sharing the machine with other services:

```yaml
threads:
  count: 4    # Pin to 4 threads
```

## Plugin Compilation

Plugin compilation is a startup-time concern:

| Scenario | Time |
|----------|------|
| First build (cold cache) | ~2 minutes per plugin |
| Incremental rebuild (source change) | ~10 seconds |
| No changes (cached) | ~10 seconds |

To speed up development:
- Use `--apps yeti-auth,my-app` to load only the apps you need.
- Keep the plugin cache (`cache/builds/`) intact between restarts.

## Rate Limiting

```yaml
rateLimiting:
  maxRequestsPerSecond: 1000
  maxConcurrentConnections: 100
```

Rate limiting prevents resource exhaustion. The backpressure layer returns 503 when `maxInFlightRequests` is exceeded.

## Memory Usage

Primary memory consumers:

| Component | Typical Usage | Configuration |
|-----------|--------------|---------------|
| RocksDB block cache | 2 GB per database | `cache_size_mb` |
| RocksDB memtables | 512 MB per database | `write_buffer_size_mb` |
| Cluster hot cache | ~10 MB per table | Fixed (10,000 entries) |
| Cluster gRPC pool | ~5 MB per connection | Pool size = num_cpus / 2 |
| HTTP connections | ~10 KB each | `maxInFlightRequests` |
| Plugin dylibs | 5-20 MB each | N/A |

### Estimating Memory (Embedded Mode)

```
Total = (cache_size_mb + write_buffer_size_mb) * num_databases
      + maxInFlightRequests * 10KB
      + num_plugins * 15MB
      + base overhead (~100MB)
```

### Estimating Memory (Cluster Mode)

```
Total = num_tables * 10MB (hot cache)
      + gRPC_pool_size * 5MB
      + maxInFlightRequests * 10KB
      + num_plugins * 15MB
      + base overhead (~100MB)
```

## Monitoring Performance

Use the [Telemetry Dashboard](../examples/telemetry-dashboard.md) to monitor:
- Request latency (p50, p95, p99).
- Throughput (requests/second).
- Error rates by endpoint.
- Storage growth over time.

The Operations API provides system health:

```bash
curl -s http://localhost:9995/health
curl -s http://localhost:9995/system
```
