# Security Architecture

Yeti provides a layered authentication and authorization system through the `yeti-auth` extension. Multiple auth providers are tried in sequence, and role-based access control governs what authenticated users can do.

## Auth Pipeline

```
Incoming Request
      │
      ▼
┌─────────────────┐
│ BasicAuthProvider│──> Authorization: Basic header
│                  │    Argon2id password verification
└────────┬────────┘
         │ (not matched)
         ▼
┌─────────────────┐
│ JwtAuthProvider  │──> Authorization: Bearer header
│                  │    Token validation + embedded permissions
└────────┬────────┘
         │ (not matched)
         ▼
┌─────────────────┐
│ OAuthAuthProvider│──> Session cookie lookup
│                  │    In-memory cache -> DB fallback
└────────┬────────┘
         │ (not matched)
         ▼
    401 Unauthorized
```

Providers are tried in order. The first successful match determines the user's identity and role.

## Password Hashing

All passwords are hashed with **Argon2id** using OWASP-recommended minimum parameters:

- Memory: 19 MiB
- Iterations: 2
- Parallelism: 1

Passwords are never stored in plaintext. The `BasicAuthProvider` maintains a credential cache with a 5-minute TTL to avoid repeated hashing on every request.

## JWT Authentication

JWT tokens use HMAC-SHA256 signing. The system issues two token types:

| Token | TTL | Purpose |
|-------|-----|---------|
| Access token | 15 minutes | API authentication, embeds permissions |
| Refresh token | 7 days | Exchange for new token pair |

Access tokens embed the user's permissions directly, eliminating a database lookup on every request. Configure in `yeti-auth/config.yaml`:

```yaml
custom:
  jwt:
    secret: "${JWT_SECRET:-development-secret-change-in-production}"
    access_ttl: 900
    refresh_ttl: 604800
```

### Endpoints

```bash
# Login (returns access + refresh tokens)
curl -sk -X POST https://localhost:9996/yeti-auth/login \
  -H "Content-Type: application/json" \
  -d '{"username":"admin","password":"password"}'

# Refresh tokens
curl -sk -X POST https://localhost:9996/yeti-auth/jwt_refresh \
  -H "Content-Type: application/json" \
  -d '{"refresh_token":"..."}'
```

## OAuth Integration

OAuth support includes GitHub and Google providers. Each application configures its own OAuth rules:

```yaml
# Per-app config in config.yaml
extensions:
  - yeti-auth:
      oauth:
        rules:
          - strategy: provider
            pattern: "google"
            role: admin
          - strategy: email
            pattern: "*@mycompany.com"
            role: standard
```

### CSRF Protection

OAuth flows use CSRF tokens stored in a `DashMap`:
- Tokens have a 10-minute TTL.
- Periodic cleanup runs every 100 insertions.
- State parameter validated on callback.

### SSRF Validation

OAuth provider URLs are validated at startup:
- Private IP addresses are rejected (10.x, 172.16-31.x, 192.168.x, 127.x).
- HTTPS is required in production environments.

### Session Persistence

OAuth sessions are stored in two layers:
1. **In-memory cache** -- Fast lookup for active sessions.
2. **Database** -- Survives server restarts.

Session cookies are set with `Secure`, `HttpOnly`, and `SameSite` attributes.

## Role-Based Access Control

Roles define permissions for tables and operations:

```json
{
  "id": "admin",
  "name": "Administrator",
  "permissions": {
    "super_user": true,
    "tables": {
      "*": { "read": true, "insert": true, "update": true, "delete": true }
    }
  }
}
```

The `super_user` role is protected from deletion and privilege removal.

### Role Resolution

- **Basic/JWT** -- User record contains `roleId`, resolved against the Role table.
- **OAuth** -- Config rules map provider/email patterns to roles.

### Attribute-Level Filtering

Roles can restrict which fields are visible:

```json
{
  "tables": {
    "employees": {
      "read": true,
      "attributePermissions": {
        "salary": { "read": false }
      }
    }
  }
}
```

Non-admin users see `salary` filtered from responses.

## Rate Limiting

Configured in `yeti-config.yaml`:

```yaml
rateLimiting:
  maxRequestsPerSecond: 1000
  maxConcurrentConnections: 100
```

The HTTP layer enforces backpressure via `maxInFlightRequests` (default: 10,000).
