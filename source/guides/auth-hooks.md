# Auth Hooks

Auth hooks allow extensions to override role resolution at request time. They run before the default role-resolution logic, enabling patterns such as tenant-based roles, subdomain-specific permissions, or header-driven access control.

---

## The AuthHook Trait

```rust
#[async_trait::async_trait]
pub trait AuthHook: Send + Sync {
    async fn on_resolve_role(
        &self,
        identity: &AuthIdentity,
        params: &ResourceParams,
    ) -> Option<Arc<dyn AccessControl>>;
}
```

When a request arrives with a valid identity (from Basic, JWT, or OAuth authentication), the auth middleware calls each registered hook in order. If any hook returns `Some(access)`, that access control object is used and the default role-resolution logic is skipped. If all hooks return `None`, the standard config-based resolution proceeds.

---

## AuthIdentity

The `AuthIdentity` struct contains the raw identity data extracted by an auth provider:

| Field | Type | Description |
|-------|------|-------------|
| `username` | `String` | Authenticated username or identifier |
| `provider` | `String` | Which provider authenticated this user (e.g., "basic", "jwt", "oauth") |
| `claims` | `Option<Value>` | JWT claims or OAuth user data (JSON) |
| `role_id` | `Option<String>` | Role identifier from the User record |

---

## AccessControl Trait

Hooks return an implementation of the `AccessControl` trait:

```rust
pub trait AccessControl: Send + Sync + std::fmt::Debug {
    fn is_super_user(&self) -> bool;
    fn username(&self) -> &str;
    fn can_read_table(&self, database: &str, table: &str) -> bool;
    fn can_insert_table(&self, database: &str, table: &str) -> bool;
    fn can_update_table(&self, database: &str, table: &str) -> bool;
    fn can_delete_table(&self, database: &str, table: &str) -> bool;
    fn can_read_attribute(&self, database: &str, table: &str, attr: &str) -> bool;
    fn can_write_attribute(&self, database: &str, table: &str, attr: &str) -> bool;
}
```

---

## Registering Auth Hooks

Extensions register hooks by implementing `auth_hooks()` on the `Extension` trait:

```rust
impl Extension for MyExtension {
    fn name(&self) -> &str { "my-auth-hook" }

    fn auth_hooks(&self) -> Vec<Arc<dyn AuthHook>> {
        vec![Arc::new(TenantHook::new())]
    }
}
```

---

## Example: Tenant-Based Roles

A hook that resolves roles based on a `X-Tenant-ID` header:

```rust
pub struct TenantHook {
    tenant_roles: HashMap<String, Arc<dyn AccessControl>>,
}

#[async_trait::async_trait]
impl AuthHook for TenantHook {
    async fn on_resolve_role(
        &self,
        identity: &AuthIdentity,
        params: &ResourceParams,
    ) -> Option<Arc<dyn AccessControl>> {
        let tenant_id = params.header("x-tenant-id")?;
        self.tenant_roles.get(tenant_id).cloned()
    }
}
```

---

## Example: Subdomain-Specific Permissions

A hook that checks the `Host` header to apply different permissions per subdomain:

```rust
#[async_trait::async_trait]
impl AuthHook for SubdomainHook {
    async fn on_resolve_role(
        &self,
        identity: &AuthIdentity,
        params: &ResourceParams,
    ) -> Option<Arc<dyn AccessControl>> {
        let host = params.header("host")?;
        if host.starts_with("admin.") {
            Some(Arc::new(AdminAccess::new(identity.username.clone())))
        } else {
            None  // Fall through to default resolution
        }
    }
}
```

---

## Hook Execution Order

1. Request arrives with credentials
2. `AuthPipeline` runs providers to produce `AuthIdentity`
3. Each registered `AuthHook` is called in order
4. First hook to return `Some(access)` wins
5. If all hooks return `None`, default role resolution runs
6. Default resolution: look up User record, resolve `roleId` to Role table

---

## Best Practices

- Return `None` from your hook to fall through to the default logic when you have no opinion about the request.
- Hooks should be fast -- avoid database queries when possible, or cache results.
- Use hooks for cross-cutting concerns (tenancy, subdomain routing). Use the standard Role table for per-user permissions.
- The `super_user` role is protected from deletion and cannot have its privileges removed through hooks.

---

## See Also

- [Roles & Permissions](auth-rbac.md) -- Default RBAC system
- [Building Extensions](building-extensions.md) -- Extension development guide
- [Basic Authentication](auth-basic.md) -- Basic auth provider details
