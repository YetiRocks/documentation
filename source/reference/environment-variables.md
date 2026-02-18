# Environment Variables

Yeti reads configuration from environment variables for settings that should not be stored in config files (secrets, credentials) or that need to vary between environments.

---

## Core Settings

| Variable | Description | Default |
|----------|-------------|---------|
| `SETTINGS_PATH` | Path to `yeti-config.yaml` | `$ROOT_DIRECTORY/yeti-config.yaml` |
| `ROOT_DIRECTORY` | Root directory for all Yeti data | `~/yeti` |
| `YETI_ROOT_DIR` | Alias for `ROOT_DIRECTORY` | -- |
| `APPLICATION_PORT` | Override `http.port` and `http.securePort` | `9996` |
| `OPERATIONS_PORT` | Override `operationsApi.port` | `9995` |
| `LOG_LEVEL` | Override `logging.level` | `"info"` |
| `ENVIRONMENT` | Override `environment` | `"development"` |

`ROOT_DIRECTORY` and `YETI_ROOT_DIR` are interchangeable. If both are set, `YETI_ROOT_DIR` takes precedence. The `--root-dir` CLI argument overrides both.

---

## Storage

| Variable | Description | Default |
|----------|-------------|---------|
| `STORAGE_MODE` | Override `storage.mode` (`"embedded"` or `"cluster"`) | `"embedded"` |
| `CLUSTER_PD_ENDPOINTS` | Override `storage.cluster.pdEndpoints` (comma-separated) | -- |

```bash
# Switch to cluster mode via environment
export STORAGE_MODE="cluster"
export CLUSTER_PD_ENDPOINTS="pd1:23791,pd2:23792,pd3:23793"
```

---

## Authentication Secrets

### JWT

| Variable | Description | Default |
|----------|-------------|---------|
| `JWT_SECRET_KEY` | Secret key for signing and verifying JWT tokens | `"development-secret-change-in-production"` |

In production, always set a strong, unique secret. The default is only suitable for development.

```bash
export JWT_SECRET_KEY="your-256-bit-secret-key-here"
```

### OAuth: Google

| Variable | Description | Default |
|----------|-------------|---------|
| `GOOGLE_CLIENT_ID` | Google OAuth 2.0 client ID | -- |
| `GOOGLE_CLIENT_SECRET` | Google OAuth 2.0 client secret | -- |

Obtain these from the [Google Cloud Console](https://console.cloud.google.com/apis/credentials).

### OAuth: GitHub

| Variable | Description | Default |
|----------|-------------|---------|
| `GITHUB_CLIENT_ID` | GitHub OAuth App client ID | -- |
| `GITHUB_CLIENT_SECRET` | GitHub OAuth App client secret | -- |

Obtain these from [GitHub Developer Settings](https://github.com/settings/developers).

### OAuth: Microsoft

| Variable | Description | Default |
|----------|-------------|---------|
| `MICROSOFT_CLIENT_ID` | Microsoft Entra (Azure AD) client ID | -- |
| `MICROSOFT_CLIENT_SECRET` | Microsoft Entra client secret | -- |
| `MICROSOFT_TENANT` | Microsoft Entra tenant ID | `"common"` |

Obtain these from the [Azure Portal](https://portal.azure.com/#blade/Microsoft_AAD_RegisteredApps).

---

## AI Service Keys

| Variable | Description | Default |
|----------|-------------|---------|
| `ANTHROPIC_API_KEY` | Anthropic Claude API key for content generation | -- |
| `VOYAGE_API_KEY` | Voyage AI API key for embedding generation | -- |

These are required only for applications that use AI features (vector search with auto-embedding, content generation).

```bash
export ANTHROPIC_API_KEY="sk-ant-..."
export VOYAGE_API_KEY="pa-..."
```

---

## OpenTelemetry

| Variable | Description | Default |
|----------|-------------|---------|
| `OTEL_EXPORTER_OTLP_ENDPOINT` | OpenTelemetry collector endpoint | -- |
| `OTEL_SERVICE_NAME` | Service name for OTLP export | `telemetry.serviceName` from config |

```bash
export OTEL_EXPORTER_OTLP_ENDPOINT="http://localhost:4317"
```

This can also be set via `telemetry.otlpEndpoint` in `yeti-config.yaml`. The environment variable takes precedence.

---

## Variable Substitution in Config Files

Application `config.yaml` files support environment variable substitution using the `${VAR:-default}` syntax:

```yaml
custom:
  jwt:
    secret: "${JWT_SECRET:-development-secret-change-in-production}"
  oauth:
    github:
      client_id: "${GITHUB_CLIENT_ID:-}"
      client_secret: "${GITHUB_CLIENT_SECRET:-}"
```

The `:-` separator provides a default value when the variable is not set. An empty default (`${VAR:-}`) results in an empty string.

---

## Setting Environment Variables

### Shell Export

```bash
export JWT_SECRET_KEY="my-secret-key"
export GOOGLE_CLIENT_ID="12345.apps.googleusercontent.com"
yeti-core --root-dir /opt/yeti
```

### Systemd Service

```ini
[Service]
Environment="JWT_SECRET_KEY=my-secret-key"
Environment="ROOT_DIRECTORY=/opt/yeti"
ExecStart=/usr/local/bin/yeti-core
```

### Docker

```bash
docker run -e JWT_SECRET_KEY=my-secret \
           -e ROOT_DIRECTORY=/data/yeti \
           yeti-core
```

---

## Security Notes

- Never commit secrets to version control.
- Use `${VAR:-}` substitution in config files to reference secrets from the environment.
- In production, use a secrets manager (Vault, AWS Secrets Manager, etc.) to inject environment variables.
- The `get_configuration` operations API endpoint sanitizes secrets from its output.

---

## See Also

- [Server Configuration](server-config.md) -- Config file reference
- [CLI Arguments](cli.md) -- Command-line arguments
- [TLS & HTTPS](tls.md) -- Certificate configuration
