# Schema Directives

Reference for all GraphQL schema directives. Directives control how types and fields are stored, indexed, and exposed.

## Type Directives

### @table

Marks a type as a persistent table.

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `database` | string | app_id | Database name for storage isolation |
| `table` | string | type name | Custom table name |
| `storage` | string | `"rocksdb"` | Storage backend |
| `expiration` | integer | none | TTL in seconds |

```graphql
type User @table { ... }
type User @table(database: "yeti-auth") { ... }
type Session @table(expiration: 3600) { ... }
```

### @export

Controls which APIs expose this table. Without `@export`, the table is internal-only.

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `name` | string | type name | Custom endpoint path |
| `rest` | boolean | app default | Expose REST CRUD endpoints |
| `graphql` | boolean | app default | Include in GraphQL schema |
| `ws` | boolean | app default | Enable WebSocket subscriptions |
| `sse` | boolean | app default | Enable Server-Sent Events |

```graphql
type User @table @export { ... }
type Rule @table @export(name: "rule") { ... }
type Log @table @export(sse: true) { ... }
```

## Field Directives

### @primaryKey

Designates the primary key. Every table needs exactly one. Typical types: `ID!`, `String`, `String!`.

```graphql
type User @table @export {
    id: ID! @primaryKey
    name: String!
}
```

### @indexed

Creates a secondary index for fast query lookups.

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `type` | string | `"hash"` | `"hash"` or `"HNSW"` |
| `m` | integer | `16` | HNSW: max connections per node |
| `ef` | integer | `200` | HNSW: search beam width |

```graphql
email: String! @indexed
embedding: [Float!]! @indexed(type: "HNSW", m: 16, ef: 200)
```

Each additional index slows writes. Only index fields used in filters.

### @relationship

Defines a relationship between tables for GraphQL joins and REST `?select` expansion.

| Parameter | Type | Description |
|-----------|------|-------------|
| `from` | string | Local field referencing the foreign table's primary key |
| `to` | string | Foreign field referencing this table's primary key (reverse) |

```graphql
type User @table @export {
    username: String @primaryKey
    roleId: String @indexed
    role: Role @relationship(from: roleId)
}

type Role @table @export {
    id: String @primaryKey
    users: [User] @relationship(to: roleId)
}
```

### @createdTime

Auto-populated with the creation timestamp (Unix epoch) on insert.

```graphql
__createdAt__: String @createdTime
```

### @updatedTime

Auto-populated with the current timestamp on every update.

```graphql
__updatedAt__: String @updatedTime
```

### @expiresAt

Per-record expiration timestamp (Unix epoch). Overrides table-level `expiration`.

```graphql
expiresAt: Int @expiresAt
```

## Field Types

| GraphQL Type | Storage Type | Notes |
|-------------|-------------|-------|
| `ID` / `ID!` | String | Typically with `@primaryKey` |
| `String` / `String!` | String | UTF-8 text |
| `Int` / `Int!` | i64 | 64-bit signed integer |
| `Float` / `Float!` | f64 | 64-bit floating point |
| `Boolean` / `Boolean!` | bool | true/false |
| `[Type]` / `[Type!]!` | Array | For relationships and vector fields |

The `!` suffix means the field is required (non-nullable).

## See Also

- [Defining Schemas](../guides/defining-schemas.md) - Schema authoring guide
- [Application Configuration](app-config.md) - Schema file configuration
- [Vector Search](../guides/vector-search.md) - HNSW vector search guide
