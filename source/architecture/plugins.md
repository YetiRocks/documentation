# Plugin System & Hot Reload

Yeti compiles custom application resources into dynamic libraries (dylibs) that are loaded at runtime. This enables application-specific logic while maintaining the platform's single-process architecture.

## Compilation Pipeline

The `ApplicationCompiler` transforms application source into loadable plugins:

```
config.yaml + schema.graphql + resources/*.rs
        │
        ▼
ApplicationCompiler
  1. Parse config.yaml
  2. Copy resource files to cache/builds/{app}/src/
  3. Generate Cargo.toml (with dependencies from config)
  4. Generate lib.rs (scans source directory for types)
  5. cargo build --release -> .dylib
        │
        ▼
cache/builds/{app}/target/release/lib{app}.dylib
```

## Pre-built RocksDB

Plugin compilation depends on `yeti-core`, which links against RocksDB. To avoid the 65-second C++ compilation of `librocksdb-sys`, Yeti ships pre-built RocksDB binaries. This reduces plugin compile time to approximately 2 minutes for a fresh build.

Cached rebuilds (when only plugin source changes) take approximately 10 seconds.

## Generated Cargo.toml

The compiler generates a Cargo project with:

```toml
[package]
name = "my-app"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
yeti-core = { path = "/path/to/yeti-core" }
# Additional dependencies from config.yaml:
serde_yaml = "0.9"
```

Application-specific dependencies are declared in `config.yaml`:

```yaml
dependencies:
  serde_yaml:
    version: "0.9"
  argon2: "0.5"
```

## Hot Reload

Yeti monitors plugin dylib files for changes using filesystem watchers:

1. File watcher detects change in `cache/builds/{app}/target/`.
2. New dylib is copied to a temp file (forces the OS to load a fresh copy).
3. Plugin is loaded from the temp copy.
4. AutoRouter is updated with new resource handlers.
5. Old dylib is unloaded.

Application-level hot reload also watches the `applications/` directory for new or removed apps.

## Plugin Source Cache

The compiler copies resource source files to `cache/builds/{app}/src/` before building. This is the copy that gets compiled, not the original.

**Important:** When changing plugin source, clear the cache to ensure changes take effect:

```bash
rm -rf ~/yeti/cache/builds/{app}/src/
rm -rf ~/yeti/cache/builds/{app}/target/
```

## Dylib Boundary Rules

Dynamic libraries have a separate copy of all static data. This creates critical constraints:

### Separate Tokio Runtime (TLS Isolation)

The dylib gets its own thread-local storage. Tokio's runtime handle is stored in TLS, so:

- **Do not call `tokio::spawn()`** from dylib code. It will crash with "Rust cannot catch foreign exceptions."
- **Use `futures::stream::unfold`** instead of spawn+channel patterns.
- **`OnceLock` statics are duplicated.** The host's copy and dylib's copy are independent.

### Logging

- **`tracing::info!()` and similar macros** do not reach the host's log subscriber (TLS isolation).
- **Use `eprintln!()`** for debug output from plugins.

### HTTP Clients

Dylib plugins have a separate TLS context. Use `reqwest::blocking::Client` directly for HTTP calls from within plugins.

### Host Methods Execute in Dylib Context

Even methods defined on host-compiled structs run in dylib context when called from dylib code. The dylib has its own compiled copy of the method. This means:

- Creating tokio channels/futures in such methods silently corrupts the host runtime.
- **Fix:** Use flag-based patterns. The dylib sets flags; the host checks flags after `on_ready()` returns and performs tokio operations in host code.

## Extension Plugins

Extensions follow the same compilation pipeline but provide shared services across applications:

```yaml
# config.yaml for an extension
extension: true
```

The compiler auto-detects extension types by scanning source for `struct {Type}Extension` patterns.
