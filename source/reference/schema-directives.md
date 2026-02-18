# Schema Directives

Complete reference for all GraphQL schema directives supported by Yeti. Directives control how types and fields are stored, indexed, and exposed through APIs.

---

## Type Directives

### @table

Marks a GraphQL type as a persistent table backed by the configured storage engine.

```graphql
@table
@table(database: "db-name")
@table(database: "db-name", table: "custom-table-name")
@table(expiration: 3600)
@table(storage: "rocksdb")
```

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `database` | string | app_id | Database name for storage isolation |
| `table` | string | type name | Custom table name in the database |
| `storage` | string | `"rocksdb"` | Storage backend type |
| `expiration` | integer | none | TTL in seconds. Records expire after this duration |

Examples:

```graphql
# Basic table (database defaults to app_id)
type User @table { ... }

# Explicit database
type User @table(database: "yeti-auth") { ... }

# Custom table name
type Rule @table(name: "rule", database: "redirect-manager") { ... }

# Table with 1-hour TTL
type Session @table(expiration: 3600) { ... }
```

### @export

Controls which API interfaces expose this table. Without `@export`, the table is internal-only.

```graphql
@export
@export(name: "custom/path")
@export(rest: true, graphql: true, ws: false, sse: false)
```

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `name` | string | type name | Custom endpoint path |
| `rest` | boolean | app default | Expose REST CRUD endpoints |
| `graphql` | boolean | app default | Include in GraphQL schema |
| `ws` | boolean | app default | Enable WebSocket subscriptions |
| `sse` | boolean | app default | Enable Server-Sent Events streaming |

Examples:

```graphql
# Export with all app defaults
type User @table @export { ... }

# Custom endpoint path
type Rule @table @export(name: "rule") { ... }

# SSE streaming only
type Log @table @export(sse: true) { ... }

# REST and GraphQL, no real-time
type Product @table @export(rest: true, graphql: true, ws: false, sse: false) { ... }
```

---

## Field Directives

### @primaryKey

Designates the primary key field for the table. Every table must have exactly one `@primaryKey` field. The field type is typically `ID!`, `String`, or `String!`.

```graphql
type User @table @export {
    id: ID! @primaryKey
    name: String!
}
```

```graphql
type User @table @export {
    username: String @primaryKey
    email: String!
}
```

### @indexed

Creates a secondary index on the field for fast query lookups. Supports hash indexes (default) and vector indexes (HNSW).

```graphql
@indexed
@indexed(type: "HNSW", m: 16, ef: 200)
```

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `type` | string | `"hash"` | Index type: `"hash"` or `"HNSW"` |
| `m` | integer | `16` | HNSW parameter: max connections per node |
| `ef` | integer | `200` | HNSW parameter: search beam width |

Examples:

```graphql
# Hash index for equality and range queries
type User @table @export {
    id: ID! @primaryKey
    email: String! @indexed
    roleId: String @indexed
}

# Vector index for similarity search
type Document @table @export {
    id: ID! @primaryKey
    content: String!
    embedding: [Float!]! @indexed(type: "HNSW", m: 16, ef: 200)
}
```

Each additional index slows write operations. Only index fields that are frequently used in filters and lookups.

### @relationship

Defines a relationship between tables. Creates navigable joins for GraphQL queries and REST `?select` expansion.

```graphql
@relationship(from: fieldName)
@relationship(to: fieldName)
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `from` | string | Local field that references the foreign table's primary key |
| `to` | string | Foreign field that references this table's primary key (reverse relationship) |

Examples:

```graphql
type User @table @export {
    username: String @primaryKey
    roleId: String @indexed
    role: Role @relationship(from: roleId)   # User -> Role (many-to-one)
}

type Role @table @export {
    id: String @primaryKey
    name: String
    users: [User] @relationship(to: roleId)  # Role -> Users (one-to-many)
}
```

The `from` direction follows a foreign key on the current type. The `to` direction defines a reverse lookup from the related type back to this type.

### @createdTime

Automatically populated with the creation timestamp (Unix epoch integer) when a record is inserted.

```graphql
type Post @table @export {
    id: ID! @primaryKey
    title: String!
    __createdAt__: String @createdTime
}
```

### @updatedTime

Automatically populated with the current timestamp on every update.

```graphql
type Post @table @export {
    id: ID! @primaryKey
    title: String!
    __updatedAt__: String @updatedTime
}
```

### @expiresAt

Designates a field as a per-record expiration timestamp. The value must be a Unix timestamp (integer). Records are removed when the current time exceeds this value, overriding the table-level `expiration` setting for individual records.

```graphql
type OAuthSession @table @export {
    sessionId: String @primaryKey
    expiresAt: Int @expiresAt
}
```

---

## Field Types

Supported GraphQL field types and their storage representations:

| GraphQL Type | Rust/Storage Type | Notes |
|-------------|-------------------|-------|
| `ID` / `ID!` | String | Typically used with `@primaryKey` |
| `String` / `String!` | String | UTF-8 text |
| `Int` / `Int!` | i64 | 64-bit signed integer |
| `Float` / `Float!` | f64 | 64-bit floating point |
| `Boolean` / `Boolean!` | bool | true/false |
| `[Type]` / `[Type!]!` | Array | Used for relationships and vector fields |

The `!` suffix means the field is required (non-nullable).

---

## Complete Example

```graphql
type Product @table(database: "store") @export(rest: true, graphql: true, sse: true) {
    id: ID! @primaryKey
    name: String!
    description: String
    price: Float! @indexed
    category: String! @indexed
    inStock: Boolean!
    embedding: [Float!]! @indexed(type: "HNSW", m: 16, ef: 200)
    __createdAt__: String @createdTime
    __updatedAt__: String @updatedTime
}
```

---

## See Also

- [Defining Schemas](../guides/defining-schemas.md) -- Schema authoring guide
- [Application Configuration](app-config.md) -- Schema file configuration
- [Vector Search](../guides/vector-search.md) -- HNSW vector search guide
