# Attribute-Level Access

Beyond table-level CRUD permissions, Yeti supports field-level (attribute-level) access control. This allows roles to restrict which fields a user can read or write, enabling a single table to serve different views to different users.

## How It Works

The permission structure supports an optional `attributePermissions` field within each table's permission block:

```json
{
  "super_user": false,
  "databases": {
    "data": {
      "tables": {
        "Employee": {
          "read": true,
          "insert": false,
          "update": false,
          "delete": false,
          "attributePermissions": {
            "salary": { "read": false, "write": false },
            "ssn": { "read": false, "write": false },
            "personalEmail": { "read": false, "write": false }
          }
        }
      }
    }
  }
}
```

With this configuration, users with this role can read Employee records but will never see the `salary`, `ssn`, or `personalEmail` fields in responses.

## Read Projection

When a user queries a table, the `PermissionContext` calculates the set of readable attributes at query planning time. Each record in the response is then projected to only include permitted fields.

For an Employee table with fields `id`, `name`, `department`, `salary`, `ssn`:

**Admin role** (full access) sees:
```json
{
  "id": "emp-1",
  "name": "Alice Smith",
  "department": "Engineering",
  "salary": 150000,
  "ssn": "123-45-6789"
}
```

**Viewer role** (salary and ssn restricted) sees:
```json
{
  "id": "emp-1",
  "name": "Alice Smith",
  "department": "Engineering"
}
```

The restricted fields are completely absent from the response -- not null, not redacted, but omitted entirely.

## Write Validation

When a user attempts to create or update a record, `validate_writable_attributes()` checks every field in the request body against the role's write permissions:

```bash
# This request will be rejected with 403 if the role cannot write "salary"
curl -sk -u viewer:password -X PUT https://localhost:9996/my-app/Employee/emp-1 \
  -H "Content-Type: application/json" \
  -d '{
    "id": "emp-1",
    "name": "Alice Smith",
    "department": "Engineering",
    "salary": 200000
  }'
```

Response:
```json
{
  "error": "Access denied: cannot write attributes [salary] in data.Employee"
}
```

## Example: Web Auth Demo

The `web-auth-demo` application demonstrates attribute-level access with an Employee resource. Different roles see different fields:

### Role Definitions

**admin** role -- full access:
```json
{
  "super_user": true
}
```

**standard** role -- restricted sensitive fields:
```json
{
  "super_user": false,
  "databases": {
    "data": {
      "tables": {
        "Employee": {
          "read": true,
          "insert": true,
          "update": true,
          "delete": false,
          "attributePermissions": {
            "salary": { "read": false, "write": false },
            "ssn": { "read": false, "write": false }
          }
        }
      }
    }
  }
}
```

### Testing Different Views

```bash
# Admin sees all fields
curl -sk -u admin:admin https://localhost:9996/web-auth-demo/Employee

# Standard user sees public fields only (salary, ssn removed)
curl -sk -u user:password https://localhost:9996/web-auth-demo/Employee
```

## Implementation Details

Attribute filtering is performed by the `PermissionContext` struct:

1. **`from_params()`** -- At query planning time, calculates the list of readable attributes by intersecting the schema's fields with the role's read permissions.
2. **`needs_projection()`** -- Returns true if any fields need to be filtered out.
3. **`project_record()`** -- Removes non-readable attributes from a single record.
4. **`project_records()`** -- Removes non-readable attributes from a batch of records.

This approach is efficient because the readable attribute set is calculated once per query, not once per record.

## Super Users

Users with `super_user: true` bypass all attribute-level checks. They always see and can write all fields.

## See Also

- [Roles & Permissions](auth-rbac.md) -- Table-level CRUD permissions
- [Authentication Overview](auth-overview.md) -- How auth providers work
