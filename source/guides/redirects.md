# URL Redirect Management

The `redirect-manager` application provides URL redirect management with static and regex pattern matching, versioning for mass cutover, host-specific rules, and time-window activation.

---

## Architecture

The redirect manager operates as a default resource that catches all incoming requests. It looks up redirect rules by composite key and either performs an HTTP redirect or returns JSON metadata for edge workers.

Two modes are supported:
- **redirect** -- Returns HTTP 301/302/307/308 responses directly
- **check** -- Returns JSON with redirect rule data (for CDN/edge worker integration)

Mode is set in `config.yaml`:

```yaml
environment:
  MODE: "redirect"   # or "check"
```

---

## Schema

Three tables manage redirect rules:

### Rule Table

```graphql
type Rule @table(database: "redirect-manager") @export(name: "rule") {
  id: ID!
  staticPath: String
  regexPattern: String
  targetUrl: String!
  statusCode: Int!        # 301, 302, 307, or 308
  queryStringOp: String   # preserve, ignore, filter, append
  host: String            # Host filter (empty = global rule)
  version: String         # Version identifier for mass cutover
  utcStartTime: String    # ISO 8601 activation start
  utcEndTime: String      # ISO 8601 activation end
  active: Boolean!
  priority: Int
}
```

### Hosts Table

```graphql
type Hosts @table(database: "redirect-manager") @export(name: "hosts") {
  id: ID!                 # Hostname (e.g., example.com)
  activeVersion: String
  enabled: Boolean!
  fallbackUrl: String
}
```

### Version Table

```graphql
type Version @table(database: "redirect-manager") @export(name: "version") {
  id: ID!                 # Version identifier (e.g., "v1", "prod-2024-01")
  name: String!
  active: Boolean!
  activatedAt: String
}
```

---

## Composite Key Lookup

Rules are stored with composite keys in the format:

```
{version}||{host}||{path}
```

For example: `0||example.com||/old-page`

The lookup process:
1. Build key from version, host, and path
2. Look up the key in the Rule table
3. Check time-window activation (utcStartTime, utcEndTime)
4. If no match and host was specified, try again with an empty host (global fallback)

---

## Creating Rules

Use the REST API to create redirect rules:

```bash
# Create a redirect rule
curl -sk -X POST https://localhost:9996/redirect-manager/rule \
  -H "Content-Type: application/json" \
  -d '{
    "id": "0||example.com||/old-page",
    "staticPath": "/old-page",
    "targetUrl": "https://example.com/new-page",
    "statusCode": 301,
    "host": "example.com",
    "version": "0",
    "active": true,
    "createdAt": "2025-01-01T00:00:00Z",
    "updatedAt": "2025-01-01T00:00:00Z"
  }'
```

---

## Version Control

Versions enable mass cutover of redirect rules. Create rules under a new version, test them, then activate:

```bash
# Create rules under version "v2"
curl -sk -X POST https://localhost:9996/redirect-manager/rule \
  -H "Content-Type: application/json" \
  -d '{"id": "v2||example.com||/about", "targetUrl": "/new-about", ...}'

# Activate version v2 for a specific host
curl -sk -X PUT https://localhost:9996/redirect-manager/hosts/example.com \
  -H "Content-Type: application/json" \
  -d '{"id": "example.com", "activeVersion": "v2", "enabled": true, ...}'
```

---

## Time-Window Activation

Rules can be constrained to specific time windows using ISO 8601 timestamps:

```json
{
  "utcStartTime": "2025-06-01T00:00:00Z",
  "utcEndTime": "2025-12-31T23:59:59Z"
}
```

Rules outside their time window are skipped during lookup, and the system falls through to the next matching rule or returns a 404.

---

## Check Mode (Edge Worker Integration)

In "check" mode, the redirect manager returns JSON instead of performing HTTP redirects. This is designed for CDN or edge worker integration:

```bash
curl -sk "https://localhost:9996/redirect-manager/old-page?h=example.com&v=0"
```

Response:
```json
{
  "path": "/old-page",
  "host": "example.com",
  "redirectURL": "/new-page",
  "statusCode": 301,
  "version": "0"
}
```

If no matching rule exists, the response is `null`.

---

## See Also

- [CRUD Operations](crud.md) -- REST API for managing rules
- [Custom Resources](custom-resources.md) -- How default resources work
- [Application Configuration](../reference/app-config.md) -- App config reference
