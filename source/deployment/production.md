# Production Checklist

Before deploying Yeti to production, review each item in this checklist. Items marked **required** will cause security or reliability issues if skipped.

## TLS Certificates (Required)

Replace self-signed certificates with real ones:

```yaml
# yeti-config.yaml
tls:
  autoGenerate: false
  privateKey: "/etc/yeti/tls/private.key"
  certificate: "/etc/yeti/tls/certificate.pem"
```

Self-signed certificates trigger browser warnings and require `-sk` flags with curl. In production, use certificates from Let's Encrypt, your CDN, or an internal CA.

## Environment Variables (Required)

Set production secrets via environment variables or `.env` file:

```bash
# JWT signing key (MUST change from default)
export JWT_SECRET_KEY="your-production-secret-min-32-chars"

# OAuth provider credentials
export GITHUB_CLIENT_ID="..."
export GITHUB_CLIENT_SECRET="..."
export GOOGLE_CLIENT_ID="..."
export GOOGLE_CLIENT_SECRET="..."

# Environment flag
export ENVIRONMENT="production"

# Optional overrides
export APPLICATION_PORT=443
export OPERATIONS_PORT=9995
export LOG_LEVEL=warn
```

## Storage (Required)

### Embedded Mode (RocksDB)

Dedicate a volume for data persistence:

```yaml
storage:
  mode: embedded
  path: "/var/lib/yeti/data"
  caching: true
  compression: true
```

Ensure the volume has:
- Sufficient IOPS for your workload (SSD recommended).
- Enough capacity for data growth.
- Regular backup schedule (see [Backup & Recovery](backup.md)).

### Cluster Mode

For distributed deployments with high availability:

```yaml
storage:
  mode: cluster
  caching: true
  cluster:
    pdEndpoints:
      - "pd1:23791"
      - "pd2:23792"
      - "pd3:23793"
    tlsCaPath: /etc/yeti/tls/ca.pem
    tlsCertPath: /etc/yeti/tls/client.pem
    tlsKeyPath: /etc/yeti/tls/client-key.pem
    timeoutMs: 5000
    autoStart: false
```

For cluster mode:
- Provision a dedicated cluster (3+ PD nodes, 3+ storage nodes).
- Enable mTLS between Yeti and the cluster in production.
- Set `autoStart: false` (auto-start is for development only).
- Ensure cluster hostnames resolve from the Yeti server.

## Logging Level

Set to `info` or `warn` in production to reduce log volume:

```yaml
logging:
  level: "warn"
  auditLog: true
```

Keep `auditLog: true` for security audit trails.

## Rate Limiting

Configure appropriate limits for your traffic:

```yaml
rateLimiting:
  maxRequestsPerSecond: 1000
  maxConcurrentConnections: 100
  maxStorageGB: 50
```

## CORS Configuration

Restrict CORS to your domains:

```yaml
http:
  cors: true
  corsAccessList:
    - "https://app.yourdomain.com"
    - "https://admin.yourdomain.com"
```

Never use `"*"` in production.

## Operations API

Secure the operations API:

```yaml
operationsApi:
  enabled: true
  port: 9995
  requireAuth: true
  cors: false
  corsAccessList: []
```

Consider binding the operations API to localhost only and accessing it through a bastion host or VPN.

## Application Review

For each enabled application:

- [ ] Remove or disable unused example applications.
- [ ] Verify `extension` configurations have appropriate OAuth rules.
- [ ] Check that seed data (`dataLoader`) is appropriate for production.
- [ ] Ensure `app_id` values are stable (changing them breaks client URLs).

## Disable Development Features

```yaml
environment: "production"
```

Setting `environment` to `production` enables:
- SSRF validation (OAuth URLs must be HTTPS, no private IPs).
- Stricter security defaults.

## Recommended Server Configuration

```yaml
# yeti-config.yaml (production)
environment: "production"
rootDirectory: "/var/lib/yeti"

http:
  port: 443
  securePort: 443
  cors: true
  corsAccessList: ["https://yourdomain.com"]
  timeout: 30000
  keepAliveTimeout: 75000
  maxInFlightRequests: 10000
  compressionThreshold: 1024

operationsApi:
  port: 9995
  enabled: true
  requireAuth: true

storage:
  mode: embedded     # or "cluster" for distributed
  caching: true
  compression: true

logging:
  level: "warn"
  auditLog: true

tls:
  autoGenerate: false
  privateKey: "/etc/yeti/tls/private.key"
  certificate: "/etc/yeti/tls/certificate.pem"

telemetry:
  metrics: true
  serviceName: "yeti-production"
  otlpEndpoint: "http://otel-collector:4317"
```

## Post-Deployment Verification

```bash
# Health check
curl -s https://your-server:9995/health

# Verify TLS
curl -v https://your-server:443/documentation/ 2>&1 | grep "SSL certificate"

# Test auth
curl -s -X POST https://your-server/yeti-auth/login \
  -H "Content-Type: application/json" \
  -d '{"username":"admin","password":"..."}'
```
