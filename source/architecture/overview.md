# System Overview

Yeti is a single-process, schema-driven application platform built in Rust. It hosts multiple applications within one runtime, each with isolated databases, routes, and authentication pipelines.

## Architecture at a Glance

```
HTTPS :9996 ──> DynamicRouter ──> /{app-id}/ prefix match
                                        │
                                   AutoRouter
                                   (per-app)
                                        │
                              ┌─────────┴─────────┐
                              │                    │
                        TableResource        Custom Resource
                        (schema-driven)      (Rust plugin)
                              │
                         BackendManager
                              │
                    ┌─────────┴─────────┐
                    │                    │
              RocksDB Shards       RocksDB Cluster
              (embedded mode)      (cluster mode)
```

## Request Lifecycle

1. **HTTPS Termination** -- TLS connection accepted on port 9996 (self-signed or real certificates).
2. **DynamicRouter** -- Extracts the first path segment as the `app-id` and looks up the registered application.
3. **AutoRouter** -- Each application has its own `AutoRouter` generated from its schema. Routes map to table resources and custom plugin resources.
4. **Resource Handler** -- The matched resource processes the request (CRUD for tables, custom logic for plugins).
5. **Response** -- JSON (or SSE stream) returned to the client.

## Multi-Tenancy

Each application is fully isolated:

| Concern | Isolation |
|---------|-----------|
| Database | Each app declares its own `database:` name in schema directives |
| Routes | Prefixed by `/{app-id}/` automatically |
| Auth | Per-app extension configuration with independent rules |
| Plugins | Separate dylib per application |

## Startup Sequence

```
1. Load yeti-config.yaml (via ~/.yeti/settings.toml indirection)
2. Initialize stdout logging and tracing dispatch
3. Preflight checks (cluster mode: verify Docker, images, DNS)
4. Kill any existing yeti-core instances
5. Initialize YetiRuntime with DatabaseManager
6. ApplicationLoader discovers apps in {rootDirectory}/applications/
7. For each enabled app:
   a. Parse config.yaml and schema.graphql
   b. Create BackendManager (RocksDB shards or cluster clients)
   c. Compile plugin (if resources defined) -> dylib
   d. Register AutoRouter with DynamicRouter
   e. Run extension on_ready() hooks
8. Start HTTPS server on port 9996
9. Start Operations API on port 9995
10. Start metrics collector (if enabled)
11. Enable hot-reload watchers for plugins and applications
```

## Key Components

- **YetiRuntime** -- Owns the `DynamicRouter`, `DatabaseManager`, and server lifecycle.
- **ApplicationLoader** -- Discovers, compiles, and loads applications from disk.
- **ApplicationCompiler** -- Generates Cargo projects from config.yaml and builds dylibs.
- **AutoRouter** -- Schema-driven router that maps table types to REST/GraphQL/SSE endpoints.
- **BackendManager** -- Maps table names to storage backends (RocksDB shards or cluster clients).
- **OperationsServer** -- Separate HTTP server (port 9995) for health checks, config, and system info.

## CLI Arguments

```bash
yeti-core --root-dir ~/yeti --apps yeti-auth,documentation
```

- `--root-dir` -- Override the root directory from config.
- `--apps` -- Comma-separated list of app IDs to load (useful for development).

## Directory Layout

```
~/yeti/                          # Root directory (runtime)
├── yeti-config.yaml             # Server configuration
├── applications/                # All applications
│   ├── yeti-auth/
│   ├── documentation/
│   └── ...
├── data/                        # Storage data
│   ├── yeti-auth/               # RocksDB databases (embedded mode)
│   ├── ...
│   └── cluster/                 # Cluster data (cluster mode)
│       ├── docker-compose.yml
│       ├── pd1-data/
│       └── node1-data/
├── certs/                       # TLS certificates
│   └── localhost/
└── cache/                       # Compiled plugin cache
    └── builds/
        └── {app-id}/
```
