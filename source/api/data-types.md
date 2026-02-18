# Data Types

Yeti schemas use GraphQL type syntax to define table fields. Each type maps to a specific storage representation in MessagePack and a JSON wire format.

## Scalar Types

| GraphQL Type | Description | JSON Format | MessagePack Format |
|-------------|-------------|-------------|-------------------|
| `ID` | Unique identifier | String | String (msgpack str) |
| `String` | UTF-8 text | String | String (msgpack str) |
| `Int` | 64-bit signed integer | Number | Integer (msgpack int64) |
| `Float` | 64-bit IEEE 754 | Number | Float (msgpack float64) |
| `Boolean` | True or false | Boolean | Boolean (msgpack bool) |

## Non-Null Modifier

Append `!` to make a field required:

```graphql
type User @table @export {
    id: ID!            # Required
    name: String!      # Required
    bio: String        # Optional (nullable)
}
```

Required fields must be provided on insert. Optional fields default to `null` if omitted.

## ID Type

The `ID` type is stored as a string. Yeti generates UUIDs by default when no ID is provided on insert:

- **UUID v7** (default) -- Time-ordered UUIDs. Records sort chronologically by ID, which is efficient for time-series queries and range scans.
- **UUID v4** -- Random UUIDs. Available as an alternative.

```bash
# Auto-generated ID (UUID v7)
curl -sk -X POST https://localhost:9996/my-app/User \
  -H "Content-Type: application/json" \
  -d '{"name": "Alice"}'
# -> {"id": "0191a2b3-c4d5-7e6f-8a9b-0c1d2e3f4a5b", "name": "Alice"}

# Custom ID
curl -sk -X POST https://localhost:9996/my-app/User \
  -H "Content-Type: application/json" \
  -d '{"id": "alice-001", "name": "Alice"}'
```

Custom string IDs are supported -- the `ID` type does not enforce UUID format.

## Date Handling

Yeti does not have a native `Date` type. Dates are stored as ISO 8601 strings:

```graphql
type Event @table @export {
    id: ID! @primaryKey
    name: String!
    startDate: String      # "2024-01-15T10:30:00Z"
}
```

Automatic timestamps are available via system fields:

```graphql
type Message @table @export {
    id: ID! @primaryKey
    content: String!
    __createdAt__: String    # Auto-populated on insert
}
```

The `__createdAt__` field is automatically set to an ISO 8601 timestamp when a record is created.

## Array Types (Vectors)

Array types are used for vector embeddings in HNSW (Hierarchical Navigable Small World) search:

```graphql
type Document @table @export {
    id: ID! @primaryKey
    content: String!
    embedding: [Float!]!    # Vector array for similarity search
}
```

The `[Float!]!` type represents a non-null array of non-null floats. Vector dimensions are determined by the embedding model used (e.g., 1536 for OpenAI `text-embedding-3-small`).

## Key Encoding

Primary keys are encoded in a **lexicographic binary format** for storage in RocksDB:

```
Key = {table_name} \x00 {primary_key_bytes}
```

This encoding preserves sort order, enabling efficient:
- Point lookups by exact key.
- Prefix scans for all records in a table.
- Range queries over ordered keys.

Since UUID v7 values are time-ordered, records naturally sort by creation time when using auto-generated IDs.

## Value Encoding

Record values are serialized with **MessagePack**, a compact binary format:

```
Field       JSON size    MessagePack size
─────       ─────────    ────────────────
"Alice"     7 bytes      6 bytes
42          2 bytes      1 byte
true        4 bytes      1 byte
3.14        4 bytes      9 bytes
null        4 bytes      1 byte
```

MessagePack is schema-aware within Yeti: fields are stored as a map of field names to values, matching the JSON structure but in binary form.

## Type Coercion

Yeti performs limited type coercion on input:

| Input | Target Type | Result |
|-------|-------------|--------|
| `"42"` | Int | `42` |
| `42` | String | `"42"` |
| `"true"` | Boolean | `true` |
| `1` | Boolean | `true` |
| `0` | Boolean | `false` |

Coercion failures return a 400 validation error.

## Schema Directives Reference

| Directive | Applies To | Purpose |
|-----------|-----------|---------|
| `@table` | Type | Declares a persistent table |
| `@table(database: "name")` | Type | Specifies the database name |
| `@table(expiration: 3600)` | Type | Auto-expire records (seconds) |
| `@export` | Type | Expose via REST and GraphQL |
| `@export(rest: true, graphql: true)` | Type | Fine-grained interface control |
| `@export(sse: true)` | Type | Enable SSE streaming |
| `@export(name: "custom/path")` | Type | Custom endpoint path |
| `@primaryKey` | Field | Mark as primary key |
| `@indexed` | Field | Create secondary index |
| `@relationship(from: "field")` | Field | Foreign key lookup |
| `@relationship(to: "field")` | Field | Reverse relationship |
