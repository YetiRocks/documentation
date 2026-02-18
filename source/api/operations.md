# Operations API

The Operations API provides administrative and management capabilities for Yeti. It runs on a separate port (default 9995) from the main application server and uses plain HTTP (no TLS).

---

## Connection Details

| Property | Value |
|----------|-------|
| Port | 9995 (configurable) |
| Protocol | HTTP (no TLS) |
| Method | POST with JSON body |
| Content-Type | `application/json` |
| Health endpoint | `GET /health` |

All operations use the same request format:

```json
{
  "operation": "operation_name"
}
```

---

## System Operations

### health_check

Check server health status.

```bash
curl -X POST http://localhost:9995/ \
  -H "Content-Type: application/json" \
  -d '{"operation": "health_check"}'
```

**Response:**
```json
{
  "data": {
    "status": "healthy",
    "uptime_seconds": 3600,
    "databases": 2,
    "applications": 5
  }
}
```

A quick health check is also available via GET:

```bash
curl http://localhost:9995/health
```

### system_information

Get system and runtime information.

```bash
curl -X POST http://localhost:9995/ \
  -H "Content-Type: application/json" \
  -d '{"operation": "system_information"}'
```

**Response:**
```json
{
  "data": {
    "system": {
      "hostname": "my-server",
      "os": "macos",
      "arch": "aarch64",
      "cpus": 10,
      "memory_total_mb": 16384,
      "memory_used_mb": 8192
    },
    "process": {
      "pid": 12345,
      "uptime_seconds": 3600,
      "memory_mb": 128
    },
    "yeti": {
      "version": "0.1.0",
      "databases": 2,
      "applications": 5
    }
  }
}
```

### get_configuration

Get the current server configuration (secrets are sanitized).

```bash
curl -X POST http://localhost:9995/ \
  -H "Content-Type: application/json" \
  -d '{"operation": "get_configuration"}'
```

---

## Application Operations

### list_applications

List all deployed applications with metadata.

```bash
curl -X POST http://localhost:9995/ \
  -H "Content-Type: application/json" \
  -d '{"operation": "list_apps"}'
```

**Response:**
```json
{
  "data": {
    "total_count": 5,
    "apps": [
      {
        "id": "my-app",
        "name": "My Application",
        "route_prefix": "/my-app",
        "table_count": 3,
        "has_graphql": true,
        "has_rest": true
      }
    ]
  }
}
```

### list_components

Harper-compatible component listing.

```bash
curl -X POST http://localhost:9995/ \
  -H "Content-Type: application/json" \
  -d '{"operation": "get_components"}'
```

---

## Describe Operations

### describe_all

List all databases and their tables.

```bash
curl -X POST http://localhost:9995/ \
  -H "Content-Type: application/json" \
  -d '{"operation": "describe_all"}'
```

### describe_table

Describe a specific table's schema.

```bash
curl -X POST http://localhost:9995/ \
  -H "Content-Type: application/json" \
  -d '{"operation": "describe_table", "database": "data", "table": "User"}'
```

**Response:**
```json
{
  "data": {
    "table": "User",
    "database": "data",
    "attributes": [
      {"name": "id", "type": "String", "primary_key": true},
      {"name": "email", "type": "String", "indexed": true},
      {"name": "name", "type": "String"}
    ]
  }
}
```

---

## Deployment Operations

### package_component

Package an application for deployment to another server.

```bash
curl -X POST http://localhost:9995/ \
  -H "Content-Type: application/json" \
  -d '{"operation": "package_component", "project": "my-app"}'
```

**Response:**
```json
{
  "data": {
    "project": "my-app",
    "payload": "H4sIAAAAAAAA...",
    "platform": "macos-aarch64",
    "has_plugins": true,
    "contents": ["config.yaml", "schema.graphql"],
    "size_bytes": 12345
  }
}
```

### deploy_component

Deploy a packaged application to this server.

```bash
curl -X POST http://localhost:9995/ \
  -H "Content-Type: application/json" \
  -d '{"operation": "deploy_component", "project": "my-app", "payload": "H4sIAAAAAAAA..."}'
```

Deployments validate that the package platform matches the target server. Cross-platform deployment requires building on the target platform.

---

## Error Format

All errors return a consistent JSON structure:

```json
{
  "error": "Error message describing what went wrong"
}
```

---

## Configuration

```yaml
# yeti-config.yaml
operationsApi:
  port: 9995
  enabled: true
  cors: true
  corsAccessList:
    - "*"
```

---

## Security

The Operations API runs on plain HTTP. Do not expose port 9995 to the public internet. Restrict access using firewall rules or bind to localhost only in production.

---

## See Also

- [REST API](rest.md) -- Application data API
- [Server Configuration](../reference/server-config.md) -- Full config reference
- [Error Codes](errors.md) -- Error response details
