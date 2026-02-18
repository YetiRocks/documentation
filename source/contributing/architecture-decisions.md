# Architecture Decision Records (ADR)

## Overview

This document records the key architectural decisions for yeti-core v1.0, explaining the rationale behind our technology choices and design philosophy.

---

## ADR-001: RocksDB as Primary Storage Backend

**Status:** Accepted
**Date:** 2025-01-11
**Decision Makers:** Platform Team

### Context

Yeti-core requires a high-performance, reliable storage backend that can handle:
- High write throughput (>100k ops/sec)
- Efficient range queries and scans
- Large datasets (100GB+)
- Production-grade reliability
- Embeddable design (no separate database server)
- Harper API compatibility

### Decision

**Selected: RocksDB**

RocksDB is the storage backend for all Yeti deployments, supporting both embedded (single-node) and cluster (distributed) modes.

### Rationale

#### Performance Characteristics
- **LSM-tree architecture**: Optimized for write-heavy workloads
- **Proven at scale**: Used by Facebook, LinkedIn, Netflix for billions of operations/day
- **Efficient compaction**: Automatic space reclamation and optimization
- **Cache-friendly**: Built-in block cache for hot data
- **Range queries**: Native support for ordered scans

#### Production Readiness
- **Battle-tested**: 10+ years in production at scale
- **Active maintenance**: Regular updates and security patches
- **Rich ecosystem**: Extensive tooling and monitoring
- **Well-documented**: Comprehensive tuning guides

#### Harper Compatibility
- **Key-value model**: Matches Harper's storage paradigm
- **Ordered keys**: Enables efficient index scans
- **Atomic writes**: Batch operations with ACID guarantees
- **TTL support**: Native time-to-live for record expiration

### Alternatives Considered

**Sled** (Rejected)
- *Pros*: Pure Rust, simpler API
- *Cons*:
  - Beta quality, not production-ready
  - Limited production usage
  - Slower performance on large datasets
  - Less mature compaction strategy
  - Smaller community and tooling ecosystem
- *Decision*: Insufficient production track record

**LMDB** (Rejected)
- *Pros*: High read performance, ACID transactions
- *Cons*:
  - Copy-on-write creates large database files
  - Write throughput lower than RocksDB
  - Requires careful memory management
  - Less suitable for write-heavy workloads
- *Decision*: Not optimized for our write patterns

**SQLite** (Rejected)
- *Pros*: Universal, well-tested, SQL interface
- *Cons*:
  - Relational model overhead for key-value operations
  - Global write lock limits concurrency
  - Larger file sizes than LSM-tree
  - SQL query parsing overhead unnecessary
- *Decision*: Unnecessary complexity for key-value workload

### Consequences

#### Positive
- **Consistent performance**: Single backend simplifies optimization
- **Production confidence**: Proven reliability at scale
- **Rich features**: Native support for TTL, compaction, snapshots
- **Active ecosystem**: Extensive tuning documentation and tools
- **Long-term viability**: Facebook/Meta backing ensures continued development

#### Negative
- **C++ dependency**: Requires system libraries (acceptable trade-off for performance)
- **Configuration complexity**: Many tuning knobs (mitigated by sensible defaults)
- **Storage overhead**: LSM-tree requires more disk space than some alternatives (acceptable for performance gains)

#### Neutral
- **No in-memory-only option**: All data persisted (aligns with production use case)
- **Single backend**: Simpler codebase, clearer performance characteristics

### Implementation Notes

**Default Configuration:**
```yaml
storage:
  cache_mb: 128           # Block cache size
  write_buffer_size_mb: 64  # Memtable size
  enable_compression: true   # Snappy compression
  sync_writes: false        # Async writes for performance
  ttl: null                 # TTL disabled by default
```

**Performance Tuning:**
- Increase `cache_mb` for read-heavy workloads
- Increase `write_buffer_size_mb` for write bursts
- Disable `enable_compression` for maximum write speed
- Enable `sync_writes` only for critical durability requirements

**Monitoring:**
- RocksDB statistics via `get_statistics()`
- Compaction metrics
- Cache hit rates
- Disk usage patterns

### Validation

**Benchmarks confirm decision:**
- Write throughput: 100k-400k ops/sec (exceeds requirements)
- Read throughput: >500k ops/sec (exceeds requirements)
- Scan performance: 381k-430k ops/sec (efficient)
- Reliability: Zero data loss in production testing

**See:** `docs/PERFORMANCE.md` for detailed benchmarks

---

## ADR-002: Rust as Implementation Language

**Status:** Accepted
**Date:** 2025-01-11
**Decision Makers:** Platform Team

### Context

Yeti-core aims to provide Harper-compatible API with native performance, requiring:
- High throughput (>100k ops/sec)
- Low latency (sub-millisecond p50)
- Memory safety
- Concurrent request handling
- Single binary deployment

### Decision

**Selected: Rust**

All yeti-core components are implemented in Rust.

### Rationale

#### Performance
- **Zero-cost abstractions**: No runtime overhead for safety
- **Native compilation**: Direct machine code, no JIT warmup
- **Predictable performance**: No garbage collection pauses
- **Efficient memory**: Manual control with safety guarantees

#### Safety & Reliability
- **Memory safety**: Prevents segfaults, buffer overflows, use-after-free
- **Thread safety**: Prevents data races at compile time
- **Error handling**: Explicit Result types enforce error checking
- **Type safety**: Strong type system catches bugs at compile time

#### Developer Experience
- **Cargo**: Excellent build system and package manager
- **Tooling**: rustfmt, clippy, cargo-bench built-in
- **Documentation**: First-class doc comments and testing
- **Community**: Active, helpful community

#### Deployment
- **Single binary**: No runtime dependencies
- **Cross-compilation**: Easy to build for multiple platforms
- **Small footprint**: Minimal resource requirements

### Alternatives Considered

**Go** (Rejected)
- *Pros*: Simple, good concurrency, fast compilation
- *Cons*:
  - Garbage collection causes latency spikes
  - Less control over memory layout
  - Slower raw performance than Rust
- *Decision*: GC unacceptable for low-latency requirements

**C++** (Rejected)
- *Pros*: Maximum performance, mature ecosystem
- *Cons*:
  - Memory safety requires constant vigilance
  - Complex build systems
  - Undefined behavior footguns
  - Harder to maintain
- *Decision*: Safety and maintainability concerns

**Node.js** (Harper's implementation)
- *Pros*: Rapid development, large ecosystem
- *Cons*:
  - Single-threaded event loop limits scalability
  - Dynamic typing causes runtime errors
  - GC pauses affect latency
  - Slower than native code
- *Decision*: Performance insufficient for high-scale deployments

### Consequences

#### Positive
- **Performance**: Native speed with safety guarantees
- **Reliability**: Compiler prevents entire classes of bugs
- **Maintainability**: Clear ownership and lifetime rules
- **Deployment**: Single binary, no runtime dependencies

#### Negative
- **Learning curve**: Ownership and lifetimes require understanding
- **Compile times**: Slower than interpreted languages (acceptable)
- **Ecosystem maturity**: Some crates less mature than equivalents in other languages

### Validation

**Production metrics confirm decision:**
- p50 latency: <500µs (exceeds requirement)
- p99 latency: <5ms (acceptable)
- Memory usage: Predictable, no spikes
- Uptime: No memory-related crashes

---

## ADR-003: Actix-Web for HTTP Server

**Status:** Accepted
**Date:** 2025-01-11
**Decision Makers:** Platform Team

### Context

Yeti-core requires an HTTP server that provides:
- High request throughput
- Async I/O for concurrent connections
- TLS/HTTPS support
- Low overhead
- Good ergonomics

### Decision

**Selected: actix-web**

Actix-web is our HTTP server framework.

### Rationale

#### Performance
- **Actor model**: Efficient concurrent request handling
- **Async/await**: Modern Rust async for I/O-bound operations
- **Benchmarks**: Among the fastest Rust web frameworks
- **Connection pooling**: Efficient resource management

#### Features
- **TLS built-in**: Native HTTPS support
- **Middleware**: Request/response transformation
- **Streaming**: Efficient large response handling
- **WebSockets**: Future extensibility
- **HTTP/2**: Modern protocol support

#### Maturity
- **Battle-tested**: Used in production by many companies
- **Active development**: Regular updates and fixes
- **Good documentation**: Comprehensive guides
- **Large community**: Many examples and patterns

### Alternatives Considered

**Axum** (Considered)
- *Pros*: Modern, type-safe, Tower ecosystem
- *Cons*:
  - Younger, less battle-tested
  - Less extensive middleware ecosystem
  - Different programming model
- *Decision*: Actix-web more proven

**Rocket** (Rejected)
- *Pros*: Excellent ergonomics, compile-time routing
- *Cons*:
  - Historically slower
  - Less flexible middleware
  - Sync-first design
- *Decision*: Performance and async concerns

**Warp** (Rejected)
- *Pros*: Functional style, type-safe
- *Cons*:
  - Steeper learning curve
  - Less documentation
  - Smaller community
- *Decision*: Ergonomics and community size

### Consequences

#### Positive
- **High throughput**: Handles >10k concurrent connections
- **Low latency**: Minimal overhead per request
- **TLS support**: Built-in HTTPS with rustls
- **Streaming responses**: Efficient for large result sets

#### Negative
- **Actor model complexity**: Requires understanding (mitigated by abstractions)
- **Breaking changes**: Actix has had major version changes (stable now on v4)

### Validation

**Production metrics:**
- Requests/second: >1k sustained
- Connection handling: >10k concurrent
- Memory per connection: <100KB
- TLS overhead: <10% latency increase

---

## ADR-004: FIQL for Query Language

**Status:** Accepted
**Date:** 2025-01-11
**Decision Makers:** Platform Team

### Context

Yeti-core needs a REST query language that is:
- Harper-compatible
- URL-safe
- Human-readable
- Powerful enough for complex queries

### Decision

**Selected: FIQL (Feed Item Query Language)**

FIQL is our REST query language for filtering and searching.

### Rationale

#### Harper Compatibility
- **Exact match**: Harper uses FIQL
- **Proven design**: Works well for REST APIs
- **User familiarity**: Existing Harper users know FIQL

#### Design Benefits
- **URL-safe**: All characters are URL-encodable
- **Human-readable**: `name==john;age>18` is intuitive
- **Composable**: AND (;) and OR (,) operators
- **Type-safe**: Operators match data types

#### Implementation
- **Parser**: Handwritten recursive descent (fast)
- **Optimizer**: Index selection and query planning
- **Evaluator**: Efficient record filtering

### Alternatives Considered

**GraphQL** (Already supported separately)
- *Pros*: Rich query language, schema-driven
- *Cons*: Not URL-friendly, requires POST
- *Decision*: Use both - GraphQL for complex queries, FIQL for REST

**OData** (Rejected)
- *Pros*: Industry standard, comprehensive
- *Cons*:
  - Complex specification
  - More verbose than FIQL
  - Steeper learning curve
- *Decision*: FIQL simpler and Harper-compatible

**Custom DSL** (Rejected)
- *Pros*: Full control over features
- *Cons*:
  - No user familiarity
  - Would break Harper compatibility
  - Reinventing the wheel
- *Decision*: FIQL already solves the problem

### Consequences

#### Positive
- **Harper compatibility**: Drop-in replacement
- **Simple implementation**: Clear specification
- **Performance**: Optimizable with indexes

#### Negative
- **Limited features**: No joins, aggregations (acceptable for key-value model)
- **Learning curve**: New users must learn FIQL syntax (mitigated by good docs)

### Validation

**Query performance benchmarks:**
- Parsing: <1µs per query
- Evaluation: <100ns per record
- Index optimization: 10-100x faster on large datasets

---

## ADR-005: No Backward Compatibility

**Status:** Accepted
**Date:** 2025-01-11
**Decision Makers:** Platform Team

### Context

This is yeti-core v1.0 - the first production-ready release.

### Decision

**No legacy support or backward compatibility**

Yeti-core v1.0 represents the current state. No historical versions, deprecated features, or migration paths exist.

### Rationale

#### Clean Slate
- **No technical debt**: Current design is optimal
- **Single code path**: No feature flags or compatibility layers
- **Clear documentation**: One way to do things
- **Simplified testing**: Test current behavior only

#### Focus
- **Current needs**: Optimize for today's requirements
- **Fast iteration**: No constraints from past decisions
- **Best practices**: Use modern Rust patterns throughout

### Consequences

#### Positive
- **Simpler codebase**: No legacy code paths
- **Clear documentation**: No deprecated features to document
- **Better performance**: No compatibility overhead

#### Negative
- **No migration path**: Users must start fresh (acceptable - new project)

### Future Compatibility

Starting with v1.0:
- **Semantic versioning**: Major versions may break compatibility
- **Deprecation policy**: Features deprecated for one major version before removal
- **Migration guides**: Provided for major version upgrades

---

## Summary

**Technology Stack (v1.0):**
- **Language**: Rust
- **Storage**: RocksDB (embedded and cluster modes)
- **HTTP Server**: actix-web
- **Query Language**: FIQL
- **Compatibility**: Harper API

**Design Philosophy:**
- Performance first
- Production-ready reliability
- Simple, focused features
- No unnecessary complexity
- Harper compatibility

**Decision Criteria:**
1. Production readiness (battle-tested components)
2. Performance benchmarks (measured, not assumed)
3. Maintainability (clear, simple code)
4. Harper compatibility (API parity)
5. Long-term viability (active development)

---

## Document History

| Version | Date | Changes |
|---------|------|---------|
| 1.0 | 2025-01-11 | Initial ADR document for v1.0 release |

---

## References

- Harper API Compatibility: `tasks/RESOURCE_API.md`
- Performance Benchmarks: `docs/PERFORMANCE.md`
- Architecture Overview: `docs/ARCHITECTURE.md` (if exists)
- Benchmark Details: `benches/README.md`
