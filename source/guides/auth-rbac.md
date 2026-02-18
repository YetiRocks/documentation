# Roles & Permissions

Yeti uses role-based access control (RBAC) to determine what authenticated users can do. Each user has a `roleId` that maps to a Role record containing a structured permissions object.

## Role Structure

Roles are stored in the `yeti-auth` Role table with the following fields:

```json
{
  "id": "standard",
  "name": "Standard User",
  "permissions": "{\"super_user\":false,\"databases\":{\"data\":{\"tables\":{\"*\":{\"read\":true,\"insert\":true,\"update\":true,\"delete\":false}}}}}"
}
```

The `permissions` field is a JSON string with this structure:

```json
{
  "super_user": false,
  "databases": {
    "data": {
      "tables": {
        "*": {
          "read": true,
          "insert": true,
          "update": true,
          "delete": false
        },
        "AuditLog": {
          "read": true,
          "insert": false,
          "update": false,
          "delete": false
        }
      }
    }
  }
}
```

## Permission Hierarchy

Permissions are evaluated from most specific to least specific:

1. **Table-specific**: `databases.{db}.tables.{TableName}` -- Permissions for a specific table.
2. **Wildcard**: `databases.{db}.tables.*` -- Default permissions for all tables in a database.
3. **Super user**: `super_user: true` -- Full access to everything, bypasses all checks.

## Built-in Roles

Yeti ships with four default roles seeded from `yeti-auth/data/roles.json`:

| Role | Permissions |
|------|-------------|
| `super_user` | Full access. Cannot be deleted or have privileges removed. |
| `admin` | Full access (super_user: true). |
| `standard` | Read, insert, update on all tables. No delete. |
| `viewer` | Read-only access to all tables. |

## Managing Roles

### Create a Role

```bash
curl -sk -u admin:admin -X POST https://localhost:9996/yeti-auth/roles \
  -H "Content-Type: application/json" \
  -d '{
    "id": "editor",
    "name": "Editor",
    "permissions": "{\"super_user\":false,\"databases\":{\"data\":{\"tables\":{\"*\":{\"read\":true,\"insert\":true,\"update\":true,\"delete\":false},\"Published\":{\"read\":true,\"insert\":true,\"update\":true,\"delete\":true}}}}}"
  }'
```

### List Roles

```bash
curl -sk -u admin:admin https://localhost:9996/yeti-auth/roles
```

### Update a Role

```bash
curl -sk -u admin:admin -X PUT https://localhost:9996/yeti-auth/roles/editor \
  -H "Content-Type: application/json" \
  -d '{
    "id": "editor",
    "name": "Editor (Updated)",
    "permissions": "{\"super_user\":false,\"databases\":{\"data\":{\"tables\":{\"*\":{\"read\":true,\"insert\":true,\"update\":true,\"delete\":true}}}}}"
  }'
```

### Delete a Role

```bash
curl -sk -u admin:admin -X DELETE https://localhost:9996/yeti-auth/roles/editor
```

Note: The `super_user` role is protected and cannot be deleted.

## Role Resolution

How a user's role is determined depends on the authentication method:

- **Basic Auth / JWT**: The `User.roleId` field is looked up in the Role table.
- **OAuth**: The per-app config rules map provider/email patterns to role IDs, which are then looked up in the Role table.

### Auth Hook Override

Extensions can provide an `AuthHook` that intercepts role resolution before the default logic runs. If the hook returns an `AccessControl` object, the default resolution is skipped. This enables custom authorization logic like external policy engines or dynamic role assignment.

## Seed Data

For new deployments, place role and user JSON files in the `yeti-auth/data/` directory:

**data/roles.json**:
```json
{
  "database": "auth",
  "table": "Role",
  "records": [
    {
      "id": "custom-role",
      "name": "Custom Role",
      "permissions": "{\"super_user\":false,\"databases\":{\"data\":{\"tables\":{\"*\":{\"read\":true,\"insert\":false,\"update\":false,\"delete\":false}}}}}",
      "createdAt": 1738800000
    }
  ]
}
```

Seed data is loaded automatically when the application starts with an empty database.

## Permission Checking in Practice

When a request arrives at a protected table:
1. `check_table_read_permission()` verifies the user can read from the table.
2. `check_table_write_permission()` verifies insert/update/delete access.
3. `validate_writable_attributes()` checks field-level write permissions.
4. `filter_readable_attributes()` removes restricted fields from responses.

Super users bypass all of these checks.

## See Also

- [Authentication Overview](auth-overview.md)
- [Attribute-Level Access](auth-attributes.md) -- Field-level permissions
- [Building Extensions](building-extensions.md) -- Creating custom auth hooks
