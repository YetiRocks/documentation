# Storage Engine

Yeti uses RocksDB as its storage engine, providing an LSM-tree based key-value store. It supports two deployment modes: **embedded** (single-node, default) and **cluster** (distributed, multi-node).

## Storage Modes

| Mode | Use Case |
|------|----------|
| `embedded` (default) | Single-node, development, small-to-medium workloads |
| `cluster` | Distributed, high-availability, large-scale production |

Configure the mode in `yeti-config.yaml`:

```yaml
storage:
  mode: embedded    # or "cluster"
```

---

## Embedded Mode

In embedded mode, RocksDB runs in-process. Writes go to an in-memory buffer (memtable), then flush to sorted on-disk files (SSTables). Background compaction merges SSTables to maintain read performance.

### Sharding

Each database is split across multiple RocksDB instances (shards) for parallel I/O:

```
Database: "my-app"
├── shard-0/    # Keys hashing to shard 0
├── shard-1/    # Keys hashing to shard 1
├── shard-2/    # Keys hashing to shard 2
└── shard-3/    # Keys hashing to shard 3
```

The default shard count is calculated as `max(num_cpus / 2, 2)`. Keys are distributed across shards using a consistent hash of the primary key.

### Data Directory

```
{rootDirectory}/data/
├── my-app/
│   ├── shard-0/
│   │   ├── 000001.sst
│   │   ├── MANIFEST-000001
│   │   └── ...
│   ├── shard-1/
│   └── ...
└── yeti-auth/
    └── ...
```

---

## Cluster Mode

In cluster mode, RocksDB is deployed as a distributed cluster with automatic data sharding, replication, and fault tolerance across multiple nodes.

### Prerequisites

- Docker and Docker Compose (for auto-start mode)
- Host entries for cluster nodes in `/etc/hosts`

### Configuration

```yaml
storage:
  mode: cluster
  cluster:
    pdEndpoints:
      - "pd1:23791"
      - "pd2:23792"
      - "pd3:23793"
    tlsCaPath: null          # Optional: CA certificate for mTLS
    tlsCertPath: null        # Optional: Client certificate
    tlsKeyPath: null         # Optional: Client private key
    timeoutMs: 5000          # Timeout per operation in milliseconds
    autoStart: true          # Auto-start Docker cluster (dev only)
```

### Auto-Start (Development)

When `autoStart: true` and `environment: development`, Yeti automatically:

1. Checks for Docker and Docker Compose availability.
2. Creates data directories at `{rootDirectory}/data/cluster/`.
3. Starts the cluster nodes via Docker Compose.
4. Polls health with exponential backoff (up to 90 seconds).
5. Adds `/etc/hosts` entries if missing (requires sudo).

The Docker Compose template is embedded in the binary and written to `{rootDirectory}/data/cluster/`.

### Connection Pool

Cluster clients use a connection pool sized at `max(num_cpus / 2, 2)`. Each connection maintains a persistent gRPC channel. Requests are distributed across the pool via round-robin.

### Hot Cache

Each table has a local LRU cache (10,000 entries, max 64KB per value) for frequently accessed records:

- **Write-through**: Cache updated before writes reach the cluster.
- **Negative caching**: Non-existent keys are tracked to avoid repeated lookups.
- **Batch-aware**: `get_batch()` checks cache first, fetches only uncached keys from the cluster.

### Key Prefixing

Cluster mode isolates tables using key prefixes: `{table_name}:{user_key}`. Prefix scans compute exclusive upper bounds for efficient range queries.

### Cluster Data Directory

```
{rootDirectory}/data/cluster/
├── docker-compose.yml
├── pd1-data/
├── pd2-data/
├── pd3-data/
├── node1-data/
├── node2-data/
└── node3-data/
```

---

## Key Encoding

Keys use a **lexicographic binary encoding** that preserves sort order for range queries:

- String keys sort alphabetically in storage.
- Prefix scans retrieve all records for a table efficiently.
- Range queries (`key >= "A" AND key < "B"`) map directly to iterators.

Key format: `{table_name}\x00{primary_key_bytes}`

## Value Encoding

Values are encoded with **MessagePack**, a compact binary serialization format:

- 30-50% smaller than JSON.
- Faster to serialize/deserialize than JSON.
- Supports all Yeti data types natively.

```
JSON:        {"name":"Alice","age":30}     = 27 bytes
MessagePack: \x82\xa4name\xa5Alice\xa3age\x1e = 18 bytes
```

## BackendManager

The `BackendManager` maps table names to backend instances. Each application declares its database in the schema:

```graphql
type User @table(database: "my-app") @export {
    id: ID! @primaryKey
    name: String!
}
```

In **embedded mode**, the BackendManager opens one sharded RocksDB instance per unique database name. Tables within a database are stored as column families.

In **cluster mode**, the BackendManager creates a backend for each table with key prefix isolation. All tables share the connection pool.

Extension tables are merged into application BackendManagers via `with_merged_tables()`. Only tables from extensions declared in the application's `extensions:` list are merged.

## KvBackend Trait

Both storage modes implement the same `KvBackend` trait:

| Method | Description |
|--------|-------------|
| `put(key, value)` | Single write |
| `get(key)` | Single read |
| `get_batch(keys)` | Batch read (optimized for relationships) |
| `delete(key)` | Single deletion |
| `scan_prefix(prefix)` | Scan with values |
| `scan_keys(prefix)` | Scan keys only (efficient) |
| `count_prefix(prefix)` | Count keys (fast, no values) |
| `write_batch(ops)` | Atomic batch operations |
| `flush()` | Flush pending writes |
| `truncate()` | Truncate entire table |

## Storage Configuration

In `yeti-config.yaml`:

```yaml
storage:
  mode: embedded       # "embedded" or "cluster"
  caching: true        # Enable read cache
  compression: true    # Enable data compression
  path: null           # Custom path (default: {rootDirectory}/data/)
```

The backend-level `StorageConfig` provides finer control:

| Parameter | Default | Description |
|-----------|---------|-------------|
| `cache_size_mb` | 2048 | Block cache size per database |
| `write_buffer_size_mb` | 512 | Memtable size before flush |
| `enable_compression` | false | LZ4 compression for SSTables |
| `sync_writes` | true | Sync WAL on every write (can be disabled for performance) |
| `disable_wal` | false | Disable write-ahead log (data loss risk) |

### High-Performance Preset

For write-heavy workloads, the high-performance preset enables async writes for 5-10x throughput improvement:

```rust
StorageConfig::high_performance()
// cache: 2GB, async writes enabled
```

## Transactions

Atomic writes use batch operations:

```rust
// Multiple operations committed atomically
backend.write_batch(vec![
    BatchOp::Put(key1, value1),
    BatchOp::Put(key2, value2),
    BatchOp::Delete(key3),
]).await?;
```

## Record Locking

Table resources support record-level locking for concurrent access:

- **ReadLock** -- Multiple readers allowed simultaneously.
- **WriteLock** -- Exclusive access for mutations.

Locks are scoped to individual records by primary key, minimizing contention.

## Table Expiration

Tables can specify automatic record expiration:

```graphql
type PageCache @table(database: "cache", expiration: 3600) {
    path: String! @primaryKey
    content: String
}
```

Records older than `expiration` seconds are automatically cleaned up.

## Backend Type Validation

The BackendManager tracks which storage backend each table uses via metadata keys (`__yeti_table_backend:{prefix}:{table}`). If a table's configured backend type changes, Yeti raises an error at startup to prevent data corruption. An LRU cache (10,000 entries) accelerates validation lookups.
