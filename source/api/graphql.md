# GraphQL API

Yeti auto-generates a GraphQL schema from tables marked with `@export(graphql: true)`. The GraphQL endpoint is available at `/{app-id}/graphql` for each application with GraphQL enabled.

---

## Endpoint

```
POST /{app-id}/graphql
```

All GraphQL requests use HTTP POST with a JSON body containing `query`, optional `variables`, and optional `operationName`.

**Headers:**

| Header | Value |
|--------|-------|
| `Content-Type` | `application/json` |

---

## Query Syntax

### List Records

Query all records from a table:

```graphql
{
  Product {
    id
    name
    price
    category
  }
}
```

```bash
curl -sk -X POST https://localhost:9996/my-app/graphql \
  -H "Content-Type: application/json" \
  -d '{"query": "{ Product { id name price category } }"}'
```

**Response:**
```json
{
  "data": {
    "Product": [
      { "id": "prod-1", "name": "Widget", "price": 9.99, "category": "tools" },
      { "id": "prod-2", "name": "Gadget", "price": 19.99, "category": "electronics" }
    ]
  }
}
```

### Single Record by ID

```graphql
{
  Product(id: "prod-1") {
    id
    name
    price
  }
}
```

### Pagination and Sorting

```graphql
{
  Product(limit: 10, offset: 0, sort: "-price") {
    id
    name
    price
  }
}
```

| Argument | Type | Description |
|----------|------|-------------|
| `id` | String | Fetch a single record by primary key |
| `limit` | Int | Maximum number of records |
| `offset` | Int | Number of records to skip |
| `sort` | String | Sort field. Prefix with `-` for descending |

---

## Nested Relationships

When tables have `@relationship` directives, GraphQL queries can traverse relationships in a single request:

Given this schema:

```graphql
type User @table @export {
    username: String @primaryKey
    roleId: String @indexed
    role: Role @relationship(from: roleId)
}

type Role @table @export {
    id: String @primaryKey
    name: String
    users: [User] @relationship(to: roleId)
}
```

### Follow a Relationship

```graphql
{
  User(id: "alice") {
    username
    role {
      id
      name
    }
  }
}
```

**Response:**
```json
{
  "data": {
    "User": {
      "username": "alice",
      "role": {
        "id": "admin",
        "name": "Administrator"
      }
    }
  }
}
```

### Reverse Relationship

```graphql
{
  Role(id: "admin") {
    name
    users {
      username
    }
  }
}
```

**Response:**
```json
{
  "data": {
    "Role": {
      "name": "Administrator",
      "users": [
        { "username": "alice" },
        { "username": "bob" }
      ]
    }
  }
}
```

---

## Variables

Use GraphQL variables for parameterized queries:

```bash
curl -sk -X POST https://localhost:9996/my-app/graphql \
  -H "Content-Type: application/json" \
  -d '{
    "query": "query GetProduct($productId: String!) { Product(id: $productId) { id name price } }",
    "variables": { "productId": "prod-1" }
  }'
```

---

## Error Format

GraphQL errors follow the standard GraphQL error format:

```json
{
  "data": null,
  "errors": [
    {
      "message": "Resource not found: Product with id 'prod-999'",
      "locations": [{ "line": 1, "column": 3 }],
      "path": ["Product"]
    }
  ]
}
```

Partial success is possible -- `data` may contain results for some fields while `errors` contains failures for others.

---

## Schema Introspection

The GraphQL endpoint supports standard introspection queries:

```graphql
{
  __schema {
    types {
      name
      fields {
        name
        type { name }
      }
    }
  }
}
```

Use a GraphQL client like the bundled `graphql-explorer` application to browse the schema interactively:

```
https://localhost:9996/graphql-explorer/
```

---

## Enabling GraphQL

### Application-Wide

Set `graphql: true` in the application's `config.yaml`:

```yaml
graphql: true
```

### Per-Table

Override with the `@export` directive:

```graphql
# This table is available via GraphQL even if the app default is false
type Product @table @export(graphql: true) { ... }
```

---

## Authentication

GraphQL endpoints respect the same authentication as REST endpoints. Include credentials in the request:

```bash
curl -sk -X POST https://localhost:9996/my-app/graphql \
  -u admin:password \
  -H "Content-Type: application/json" \
  -d '{"query": "{ User { username email } }"}'
```

---

## See Also

- [REST API](rest.md) -- REST endpoint reference
- [Relationships & Joins](../guides/relationships.md) -- Relationship modeling
- [Schema Directives](../reference/schema-directives.md) -- Directive reference
