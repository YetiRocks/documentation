# Authentication & Authorization

Yeti provides a complete authentication and authorization system through the `yeti-auth` extension. It supports three authentication methods that can be used independently or combined, with role-based access control (RBAC) down to the field level.

## Architecture

The `yeti-auth` extension is a standalone application that provides authentication services to other applications. Each app opts in by declaring `yeti-auth` in its `extensions:` list:

```yaml
# my-app/config.yaml
name: "My App"
app_id: "my-app"
extensions:
  - yeti-auth:
      oauth:
        rules:
          - strategy: provider
            pattern: "github"
            role: standard
```

## Authentication Methods

Yeti supports three authentication methods. The AuthPipeline tries each configured provider in order until one succeeds:

| Method | Header | Use Case |
|--------|--------|----------|
| [Basic Auth](auth-basic.md) | `Authorization: Basic base64(user:pass)` | Server-to-server, scripts, simple integrations |
| [JWT](auth-jwt.md) | `Authorization: Bearer <token>` | SPAs, mobile apps, stateless APIs |
| [OAuth](auth-oauth.md) | Session cookie | Web applications with third-party login |

## How Auth Works

1. A request arrives at an authenticated application.
2. The **AuthPipeline** runs each registered auth provider in order.
3. The first provider that recognizes the credentials returns an **AuthIdentity**.
4. The identity's **roleId** is resolved to a **Role** from the Role table.
5. The role's **permissions** are attached to the request as an **AccessControl** object.
6. Resource handlers check permissions for table read/write and attribute-level access.

If no provider can authenticate the request, a `401 Unauthorized` response is returned.

## Auth is Per-App

Authentication is configured per application. An app without `yeti-auth` in its `extensions:` list has no authentication -- all requests are permitted. This means you can have public APIs alongside authenticated ones on the same Yeti instance.

## Quick Setup

1. Ensure the `yeti-auth` application is enabled (it ships with Yeti).
2. Create a user:

```bash
curl -sk -X POST https://localhost:9996/yeti-auth/users \
  -H "Content-Type: application/json" \
  -d '{
    "username": "myuser",
    "password": "secure-password-123",
    "roleId": "standard",
    "email": "myuser@example.com"
  }'
```

3. Add `yeti-auth` to your app's extensions:

```yaml
extensions:
  - yeti-auth: {}
```

4. Access your app with credentials:

```bash
# Basic Auth
curl -sk -u myuser:secure-password-123 https://localhost:9996/my-app/MyTable

# JWT
TOKEN=$(curl -sk -X POST https://localhost:9996/yeti-auth/login \
  -H "Content-Type: application/json" \
  -d '{"username":"myuser","password":"secure-password-123"}' | jq -r .access_token)

curl -sk -H "Authorization: Bearer $TOKEN" https://localhost:9996/my-app/MyTable
```

## Default Users and Roles

Yeti ships with seed data in `yeti-auth/data/`:

**Users**: `admin` (role: admin), `user` (role: viewer)

**Roles**: `super_user`, `admin`, `standard`, `viewer`

The `super_user` role has full access to everything and cannot be deleted.

## Sub-Guides

- [Basic Authentication](auth-basic.md)
- [JWT Authentication](auth-jwt.md)
- [OAuth Integration](auth-oauth.md)
- [Roles & Permissions](auth-rbac.md)
- [Attribute-Level Access](auth-attributes.md)
