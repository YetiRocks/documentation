# CLI Arguments

The `yeti-core` binary accepts command-line arguments to override configuration settings.

---

## Usage

```bash
yeti-core [OPTIONS]
```

---

## Options

### --root-dir

Override the root directory for all Yeti data. This takes precedence over the `ROOT_DIRECTORY` and `YETI_ROOT_DIR` environment variables, and the `rootDirectory` setting in `yeti-config.yaml`.

```bash
yeti-core --root-dir /opt/yeti
```

| Argument | Type | Default |
|----------|------|---------|
| `--root-dir` | path | `~/yeti` (or `$ROOT_DIRECTORY` if set) |

The root directory must contain:
- `yeti-config.yaml` -- Server configuration
- `applications/` -- Application directories
- `data/` -- Database storage (created automatically)
- `certs/` -- TLS certificates (created automatically if `tls.autoGenerate: true`)

### --apps

Filter which applications to load at startup. Provide a comma-separated list of application IDs. Only the specified applications will be loaded; all others are skipped.

```bash
yeti-core --apps yeti-auth,my-app,yeti-telemetry
```

| Argument | Type | Default |
|----------|------|---------|
| `--apps` | comma-separated string | all enabled apps |

When `--apps` is not provided, all applications with `enabled: true` in their `config.yaml` are loaded.

---

## Examples

### Start with default settings

```bash
yeti-core
```

Loads all enabled applications from `~/yeti/applications/` using `~/yeti/yeti-config.yaml`.

### Start with a custom root directory

```bash
yeti-core --root-dir /opt/yeti-production
```

### Load only specific applications

```bash
yeti-core --apps application-template
```

### Combine options

```bash
yeti-core --root-dir /opt/yeti --apps yeti-auth,my-app
```

### Development: load only auth and your app

```bash
yeti-core --apps yeti-auth,my-app
```

This significantly reduces startup time by skipping compilation of unused applications.

---

## Startup Behavior

1. Parse CLI arguments
2. Resolve root directory (CLI > env var > config default)
3. Read `yeti-config.yaml` from root directory
4. Discover applications in `$rootDirectory/applications/`
5. Filter by `--apps` if specified, otherwise load all `enabled: true` apps
6. Compile application plugins (first run takes ~2 min per plugin)
7. Load compiled plugins and register resources
8. Start HTTPS server on configured port
9. Start Operations API on configured port

---

## Plugin Compilation

On first startup, each application's Rust source files are compiled into dynamic libraries. This takes approximately 2 minutes per application. Subsequent restarts use cached builds and take approximately 10 seconds.

To clear the plugin cache (required after yeti-core rebuild):

```bash
rm -rf ~/yeti/cache/builds/*/target/
```

To also clear copied source files (required when fixing plugin errors):

```bash
rm -rf ~/yeti/cache/builds/*/src/
```

---

## Checking Status

After startup, verify the server is running:

```bash
# Health check via operations API
curl http://localhost:9995/health

# Application API (with self-signed cert)
curl -sk https://localhost:9996/application-template/TableName
```

---

## See Also

- [Server Configuration](server-config.md) -- Full `yeti-config.yaml` reference
- [Environment Variables](environment-variables.md) -- Environment-based configuration
- [Application Configuration](app-config.md) -- Per-app config files
