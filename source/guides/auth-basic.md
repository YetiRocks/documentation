# Basic Authentication

HTTP Basic authentication is the simplest way to secure a Yeti application. The client sends credentials with every request using the standard `Authorization: Basic` header.

## How It Works

1. The client sends `Authorization: Basic base64(username:password)` with each request.
2. The `BasicAuthProvider` decodes the header and looks up the user in the User table.
3. The password is verified against the stored Argon2id hash.
4. On success, the user's `roleId` resolves to a Role with permissions.

## Configuration

Add `yeti-auth` to your application's extensions list. No additional configuration is needed for Basic auth -- it is always available when `yeti-auth` is enabled:

```yaml
# my-app/config.yaml
name: "My App"
app_id: "my-app"
schemas:
  - schema.graphql
extensions:
  - yeti-auth: {}
```

## Creating Users

Create users via the `yeti-auth` REST API:

```bash
curl -sk -X POST https://localhost:9996/yeti-auth/users \
  -H "Content-Type: application/json" \
  -d '{
    "username": "alice",
    "password": "strong-password-here",
    "roleId": "standard",
    "email": "alice@example.com"
  }'
```

The password is automatically hashed with **Argon2id** (OWASP minimum parameters) before storage. The raw password is never persisted.

## Making Authenticated Requests

Use the `-u` flag with curl or set the `Authorization` header directly:

```bash
# Using curl's -u shorthand
curl -sk -u alice:strong-password-here \
  https://localhost:9996/my-app/MyTable

# Using the header directly
curl -sk -H "Authorization: Basic YWxpY2U6c3Ryb25nLXBhc3N3b3JkLWhlcmU=" \
  https://localhost:9996/my-app/MyTable
```

## Credential Cache

To avoid repeated Argon2id hash verification on every request, the `BasicAuthProvider` maintains an in-memory credential cache with a **5-minute TTL**. After a successful authentication, subsequent requests with the same credentials are served from cache without hitting the database.

The cache is automatically invalidated when:
- The TTL expires (5 minutes).
- The user's password is changed.
- The Yeti server restarts.

## Managing Users

```bash
# List all users
curl -sk -u admin:admin https://localhost:9996/yeti-auth/users

# Get a specific user
curl -sk -u admin:admin https://localhost:9996/yeti-auth/users/alice

# Update a user's role
curl -sk -u admin:admin -X PUT https://localhost:9996/yeti-auth/users/alice \
  -H "Content-Type: application/json" \
  -d '{
    "username": "alice",
    "roleId": "admin",
    "email": "alice@example.com"
  }'

# Delete a user
curl -sk -u admin:admin -X DELETE https://localhost:9996/yeti-auth/users/alice
```

## Password Hashing

Yeti uses **Argon2id** with OWASP-recommended minimum parameters:

- Memory: 19456 KiB (19 MiB)
- Iterations: 2
- Parallelism: 1

These parameters provide strong protection against brute-force and side-channel attacks while keeping authentication latency under 100ms.

## Security Considerations

- Always use HTTPS. Basic auth transmits credentials in base64 (not encrypted).
- Yeti runs on HTTPS port 9996 with TLS by default, so credentials are always encrypted in transit.
- For browser-based applications, prefer [JWT](auth-jwt.md) or [OAuth](auth-oauth.md) to avoid storing passwords client-side.
- Basic auth is ideal for server-to-server communication, CLI tools, and automated scripts.

## See Also

- [Authentication Overview](auth-overview.md)
- [JWT Authentication](auth-jwt.md) -- Stateless token-based auth
- [Roles & Permissions](auth-rbac.md) -- Configuring what users can access
