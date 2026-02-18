# REST API

Yeti automatically generates REST endpoints for every table with `@export(rest: true)`. Endpoints follow a consistent pattern for all CRUD operations.

---

## Base URL

All table endpoints are scoped under the application prefix:

```
https://localhost:9996/{app-id}/{TableName}
```

For example, the `User` table in the `yeti-auth` application is at:

```
https://localhost:9996/yeti-auth/User
```

---

## Endpoints

### List Records

```
GET /{app-id}/{TableName}
```

Returns an array of all records in the table.

**Query Parameters:**

| Parameter | Type | Description |
|-----------|------|-------------|
| `limit` | integer | Maximum number of records to return |
| `offset` | integer | Number of records to skip (for pagination) |
| `sort` | string | Sort field. Prefix with `-` for descending (e.g., `-createdAt`) |
| `select` | string | Comma-separated list of fields to include |
| `filter` | string | FIQL filter expression |
| `stream` | string | Set to `sse` for Server-Sent Events streaming |

**Example:**

```bash
curl -sk "https://localhost:9996/my-app/Product?limit=10&offset=0&sort=-createdAt&select=id,name,price"
```

**Response:**
```json
[
  { "id": "prod-1", "name": "Widget", "price": 9.99 },
  { "id": "prod-2", "name": "Gadget", "price": 19.99 }
]
```

### Create Record

```
POST /{app-id}/{TableName}
```

Creates a new record. The request body must include all required fields and the primary key.

**Headers:**

| Header | Value |
|--------|-------|
| `Content-Type` | `application/json` |

**Example:**

```bash
curl -sk -X POST https://localhost:9996/my-app/Product \
  -H "Content-Type: application/json" \
  -d '{
    "id": "prod-3",
    "name": "Doohickey",
    "price": 29.99,
    "category": "tools"
  }'
```

**Response (201 Created):**
```json
{
  "message": "Record created",
  "id": "prod-3"
}
```

### Read Record

```
GET /{app-id}/{TableName}/{id}
```

Returns a single record by primary key.

**Example:**

```bash
curl -sk https://localhost:9996/my-app/Product/prod-1
```

**Response (200 OK):**
```json
{
  "id": "prod-1",
  "name": "Widget",
  "price": 9.99,
  "category": "tools",
  "__createdAt__": "2025-01-15T10:00:00Z"
}
```

**Response (404 Not Found):**
```json
{
  "error": "Resource not found: Product with id 'prod-999'"
}
```

### Replace Record

```
PUT /{app-id}/{TableName}/{id}
```

Replaces an entire record. All fields must be provided.

**Example:**

```bash
curl -sk -X PUT https://localhost:9996/my-app/Product/prod-1 \
  -H "Content-Type: application/json" \
  -d '{
    "id": "prod-1",
    "name": "Widget Pro",
    "price": 14.99,
    "category": "tools"
  }'
```

### Partial Update

```
PATCH /{app-id}/{TableName}/{id}
```

Updates specific fields without replacing the entire record.

**Example:**

```bash
curl -sk -X PATCH https://localhost:9996/my-app/Product/prod-1 \
  -H "Content-Type: application/json" \
  -d '{"price": 12.99}'
```

### Delete Record

```
DELETE /{app-id}/{TableName}/{id}
```

Deletes a record by primary key.

**Example:**

```bash
curl -sk -X DELETE https://localhost:9996/my-app/Product/prod-1
```

**Response (200 OK):**
```json
{
  "message": "Record deleted"
}
```

---

## FIQL Filtering

The `filter` query parameter accepts FIQL (Feed Item Query Language) expressions:

| Operator | FIQL Syntax | Description |
|----------|-------------|-------------|
| Equal | `field==value` | Exact match |
| Not equal | `field!=value` | Exclude matches |
| Greater than | `field=gt=value` | Numeric/string comparison |
| Greater or equal | `field=ge=value` | Numeric/string comparison |
| Less than | `field=lt=value` | Numeric/string comparison |
| Less or equal | `field=le=value` | Numeric/string comparison |
| AND | `;` | Both conditions must match |
| OR | `,` | Either condition matches |
| Wildcard | `field==*value*` | Contains match |

**Examples:**

```bash
# Exact match
curl -sk "https://localhost:9996/my-app/Product?filter=category==tools"

# Range query
curl -sk "https://localhost:9996/my-app/Product?filter=price=gt=10;price=lt=50"

# Combined with sort and limit
curl -sk "https://localhost:9996/my-app/Product?filter=category==tools&sort=-price&limit=5"
```

---

## Authentication

When yeti-auth is loaded, endpoints require authentication:

```bash
# Basic auth
curl -sk -u admin:password https://localhost:9996/my-app/Product

# JWT Bearer token
curl -sk -H "Authorization: Bearer eyJ..." https://localhost:9996/my-app/Product
```

Without yeti-auth loaded, all endpoints are accessible without authentication.

---

## Response Headers

| Header | Description |
|--------|-------------|
| `Content-Type` | `application/json` |
| `x-cache` | `HIT` or `MISS` (when caching is active) |
| `x-request-id` | Unique request identifier |

---

## See Also

- [FIQL Queries](../guides/fiql.md) -- Complete FIQL query guide
- [Pagination & Sorting](../guides/pagination.md) -- Pagination patterns
- [GraphQL API](graphql.md) -- Alternative query interface
- [Error Codes](errors.md) -- Error response format
