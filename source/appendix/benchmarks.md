# Performance Benchmarks

**Comprehensive benchmark results for Yeti's performance characteristics**

This document contains actual benchmark data from Yeti's test suite. All benchmarks are run using Criterion.rs with consistent settings to ensure reproducible results.

**Test Environment**:
- **Test Schema**: ~1KB records with 8 attributes
- **Multithreaded**: 8 threads per benchmark
- **Sample size**: 20 iterations per benchmark
- **Measurement time**: 5 seconds per benchmark

**See Also**:
- [Architecture](../foundations/architecture.md) - Why Yeti is fast
- [Performance Tuning](../administration/performance-tuning.md) - Optimize your application
- [ROADMAP](../../tasks/ROADMAP.md) - Planned optimizations

---

## Performance Summary

Yeti achieves 10-50x better performance than traditional multi-process architectures through its unified process model:

| Operation | Throughput | Notes |
|-----------|------------|-------|
| **Simple READ** | 186K ops/s | Direct RocksDB access |
| **Simple CREATE** | 82K ops/s | No indexes |
| **Mixed Workload** | 156K ops/s | 70% read, 30% write |
| **With Indexes** | 15-62K ops/s | Depends on index count |

---

## Core CRUD Benchmarks

| Operation | 0 indexes | 1 index (name) | 2 indexes (name, email) |
|-----------|-----------|----------------|-------------------------|
| **CREATE** | 82.5K ops/s | 25.4K ops/s (-69%) | 15.6K ops/s (-81%) |
| **READ** | 186.6K ops/s | 175K ops/s (-6%) | 172K ops/s (-8%) |
| **UPDATE** | 64.5K ops/s | 10.4K ops/s (-84%) | 5.7K ops/s (-91%) |
| **DELETE** | 32.9K ops/s | 9.7K ops/s (-71%) | 5.7K ops/s (-83%) |
| **MIXED (70R/30W)** | 156.2K ops/s | 62.0K ops/s (-60%) | 40.2K ops/s (-74%) |

---

## Full Stack Benchmarks

| Layer | Benchmark | Throughput | Notes |
|-------|-----------|------------|-------|
| **Encoding** | key_encoding (per request) | 2.19M ops/s | Single key encode overhead |
| **Encoding** | value_encoding (per request) | 7.60M ops/s | Single value encode overhead |
| **Storage** | backend (single put) | 845K ops/s | Single RocksDB put per request |
| **Storage** | backend (single get) | 821K ops/s | Single RocksDB get per request |
| **Indexes** | hash_index (single insert) | 14.1K ops/s | Single record index update |
| **Indexes** | hash_index (single lookup) | 52.5K ops/s | Single value lookup (19µs) |
| **Indexes** | range_index (single insert) | 16.7K ops/s | Single record index update |
| **Indexes** | range_index (single scan) | 5.13M ops/s | Single range query |
| **Query** | fiql_evaluation (simple eq) | 28.0M ops/s | Simple equality (36ns) |
| **Query** | fiql_evaluation (10k records) | 43.4M ops/s | Filter 10k records (230µs) |
| **Handlers** | handlers (1k posts) | **331K ops/s** | After count tracking removal |
| **Handlers** | handlers (concurrent 8T) | **807K ops/s** | Concurrent writes |
| **Handlers** | handlers (mixed 70R/30W) | **464K ops/s** | Mixed workload |
| **Ingress** | ingress_request_deserialize (small) | 4.29M ops/s | JSON request parsing (small) |
| **Ingress** | ingress_request_deserialize (medium) | 1.99M ops/s | JSON request parsing (medium) |
| **Ingress** | ingress_request_deserialize (large) | 978K ops/s | JSON request parsing (large) |
| **Ingress** | ingress_response_serialize (small) | 11.4M ops/s | JSON response building (small) |
| **Ingress** | ingress_response_serialize (medium) | 5.84M ops/s | JSON response building (medium) |
| **Ingress** | ingress_response_serialize (large) | 2.96M ops/s | JSON response building (large) |
| **Ingress** | ingress_header_parsing (concurrent) | 1.85M ops/s | HTTP header parsing (8T) |
| **Ingress** | ingress_body_extraction (1KB) | 12.3M ops/s | Request body extraction 1KB |
| **Ingress** | ingress_body_extraction (10KB) | 4.16M ops/s | Request body extraction 10KB |
| **Ingress** | ingress_body_extraction (100KB) | 376K ops/s | Request body extraction 100KB |
| **Ingress** | ingress_response_building (concurrent) | 5.93M ops/s | Response building (8T) |
| **Ingress** | ingress_request_processing (concurrent) | 1.22M ops/s | Full request/response cycle (8T) |
| **Routing** | routing (GET 1 table) | 179K ops/s | Route matching concurrent |
| **Routing** | routing (GET 10 tables) | 178K ops/s | Route matching concurrent |
| **Routing** | routing (GET 100 tables) | 173K ops/s | Route matching concurrent |
| **Routing** | routing (POST 1 table) | 87.6K ops/s | POST route matching |
| **Routing** | routing (POST 10 tables) | 88.6K ops/s | POST route matching |
| **Routing** | routing (POST 100 tables) | 85.1K ops/s | POST route matching |
| **Transaction** | transaction (single write) | 91.3K ops/s | Single-record ACID write |
| **Transaction** | transaction (read-modify-write) | 66.7K ops/s | Single-record RMW pattern |
| **Transaction** | transaction (delete) | 94.5K ops/s | Single-record delete |
| **Transaction** | transaction (abort) | 4.02M ops/s | Transaction abort cost |
| **Locking** | locking (acquire 100) | 3.06M ops/s | Lock acquisition unique keys |
| **Locking** | locking (acquire 1000) | 3.28M ops/s | Lock acquisition unique keys |
| **Locking** | locking (unlock cycles) | 2.03M ops/s | Lock/unlock full cycles |
| **Locking** | locking (contention 1 key) | 4.02M ops/s | Single hot key contention |
| **Locking** | locking (contention 10 keys) | 2.33M ops/s | 10 hot keys contention |
| **Locking** | locking (contention 100 keys) | 2.30M ops/s | 100 hot keys contention |
| **Locking** | locking (callbacks) | 77.2K ops/s | Lock with callback invocations |
| **Locking** | locking (timeout immediate) | 1.84M ops/s | Immediate lock success |
| **Locking** | locking (status checks) | 5.17M ops/s | is_locked() query overhead |
| **Locking** | locking (count queries) | 10.5M ops/s | lock_count() query overhead |
| **Static Files** | static_files (100 files) | 169K ops/s | Serve 100 files |
| **Static Files** | static_files (1000 files) | 173K ops/s | Serve 1000 files |
| **Static Files** | static_files (MIME detection) | 35.9M ops/s | MIME type lookups |
| **Static Files** | static_files (path matching) | 106M ops/s | Path resolution |
| **Static Files** | static_files (index fallback) | 233K ops/s | index.html serving |
| **Static Files** | static_files (extension match) | 176K ops/s | Extension fallback |
| **Static Files** | static_files (mixed types) | 176K ops/s | Mixed file types |
| **Static Files** | static_files (404) | 173K ops/s | Not found responses |
| **Observability** | observability (context 100) | 3.89M ops/s | RequestContext creation |
| **Observability** | observability (context 1000) | 9.35M ops/s | RequestContext creation |
| **Observability** | observability (context+metadata) | 8.78M ops/s | Context with metadata |
| **Observability** | observability (span creation) | 9.03M ops/s | Span creation overhead |
| **Observability** | observability (span enter/exit) | 8.99M ops/s | Span lifecycle overhead |
| **Observability** | observability (request IDs 100) | 4.88M ops/s | Request ID generation |
| **Observability** | observability (request IDs 1000) | 10.6M ops/s | Request ID generation |
| **Observability** | observability (request IDs 10k) | 11.5M ops/s | Request ID generation |
| **Observability** | observability (duration tracking) | 29.0M ops/s | Duration queries |
| **Observability** | observability (log completion) | 9.39M ops/s | Log completion overhead |
| **Observability** | observability (extract w/header) | 19.1M ops/s | Extract ID from header |
| **Observability** | observability (extract no header) | 25.5M ops/s | Extract ID without header |
| **Observability** | observability (full lifecycle) | 8.36M ops/s | Complete context lifecycle |

---

## Running Benchmarks

All benchmarks use `sample_size(20)` for fast iteration. The automated script runs the full suite including index comparison in ~10 minutes.

### Automated Benchmark Suite (Recommended)

```bash
# From platform/yeti-core/

# Run ALL benchmarks and update both CRUD and detailed tables
python3 run-benches.py

# Run only integration benchmarks with 0, 1, 2 indexes
python3 run-benches.py integration

# Run specific layer benchmark and update detailed table
python3 run-benches.py transaction
python3 run-benches.py ingress

# Test optimizations without updating PERFORMANCE.md (dry-run)
python3 run-benches.py transaction --dry-run
python3 run-benches.py integration --dry-run

# Show help
python3 run-benches.py --help
```

**Features:**
- **Timeout protection**: Aborts if benchmarks hang (2 min per benchmark, 10 min total)
- **Watchdog monitoring**: Detects hangs if no output for 2 minutes
- **Automatic table updates**: Updates CRUD and detailed tables in PERFORMANCE.md
- **Index comparison**: Runs integration benchmarks with 0, 1, and 2 indexes
- **Error handling**: Exits immediately on compilation errors, test failures, or timeouts

### Manual Benchmark Runs

For direct `cargo bench` invocations:

```bash
# Run all benchmarks
cargo bench

# Run specific benchmark
cargo bench --bench transaction
cargo bench --bench integration
cargo bench --bench ingress

# Save baseline for comparison
cargo bench -- --save-baseline before-optimization

# Compare against baseline
cargo bench -- --baseline before-optimization

# List all benchmarks
cargo bench -- --list
```

---

## Benchmark Configuration

All benchmarks use Criterion.rs with consistent settings:

- **Sample size**: 20 iterations per benchmark
- **Warm-up time**: 3 seconds
- **Measurement time**: 5 seconds
- **Concurrent threads**: 8 threads for concurrent benchmarks
- **Timeouts**: 2 minutes per benchmark, 10 minutes total (enforced by `run-benches.py`)

**Index Optimization Status:**
- ✅ Differential index updates (Harper-compatible)
- ✅ Batch operations with WriteBatch
- ✅ Composite key indexing (eliminates read-modify-write)
- ✅ UPDATE performance: Only changed values re-indexed

---

## Resource Monitoring

Monitor CPU/Memory/Disk during benchmarks:

```bash
# macOS
top -pid $(pgrep -f "cargo-criterion")

# Linux
htop -p $(pgrep -f "cargo-criterion")

# Monitor specific benchmark
ps aux | grep "benchmark-name"
```

**Expected Resource Usage:**
- CPU: 200-400% (multi-threaded benchmarks use ~4 cores)
- Memory: 100-300 MB per benchmark process
- Disk: Temporary RocksDB instances in `/tmp/`

See individual benchmark files in `benches/` for detailed metrics documentation.

## Understanding the Numbers

### What These Benchmarks Mean

**Key Takeaways**:

1. **Reads are Fast** (186K ops/s)
   - Direct RocksDB access with minimal overhead
   - In-memory caching makes hot data even faster
   - Index presence has minimal impact on reads (~8% slower)

2. **Writes Scale with Indexes** (82K → 15K ops/s)
   - No indexes: Very fast (82K ops/s)
   - 1 index: 3x slower (25K ops/s) - still excellent
   - 2 indexes: 5x slower (15K ops/s) - trade-off for query speed
   - **Key Insight**: Only index fields you filter on

3. **Mixed Workloads are Realistic** (156K ops/s)
   - Most applications are 70% reads, 30% writes
   - Yeti handles this extremely well (156K ops/s with no indexes)
   - With indexes: Still fast (40-62K ops/s)

### Comparison to Alternatives

| System | Simple Read | Simple Write | Notes |
|--------|-------------|--------------|-------|
| **Yeti** | 186K ops/s | 82K ops/s | Unified process, RocksDB |
| **Traditional DB** | 5-10K ops/s | 3-5K ops/s | Network + serialization overhead |
| **MongoDB** | 10-20K ops/s | 5-10K ops/s | Separate process, network calls |
| **PostgreSQL** | 15-30K ops/s | 10-15K ops/s | Separate process, network calls |
| **In-Memory Cache** | 100-500K ops/s | N/A | No persistence |

**Yeti's Advantage**: 10-20x faster than traditional databases due to zero network overhead and direct memory access.

### Performance Tips

**DO**:
- ✅ Index fields you filter on frequently
- ✅ Use batch operations when possible
- ✅ Enable caching for read-heavy workloads
- ✅ Monitor index count (keep under 3-4 per table)

**DON'T**:
- ❌ Over-index (every extra index slows writes significantly)
- ❌ Skip indexes on filtered fields (slow scans)
- ❌ Ignore slow query logs
- ❌ Run development mode in production

**See**: [Performance Tuning Guide](../administration/performance-tuning.md) for detailed optimization strategies.

---

## Interpreting Layer-by-Layer Results

The detailed benchmarks show performance at each layer:

- **Encoding** (2-7M ops/s): Extremely fast, not a bottleneck
- **Storage** (821K-845K ops/s): RocksDB is highly optimized
- **Indexes** (14-52K ops/s): Main write-path cost
- **Handlers** (331-807K ops/s): Application logic layer
- **Ingress** (1-12M ops/s): HTTP parsing is fast

**Bottleneck Analysis**:
1. **For Writes**: Index updates (14-16K ops/s) dominate cost
2. **For Reads**: RocksDB access (821K ops/s) is fast, minimal overhead
3. **Overall**: Indexes are the main performance tuning lever

---

**Last Updated**: 2025-01-12  
**Benchmark Version**: v0.1.0  
**Machine**: Development environment (8-core, 16GB RAM)
