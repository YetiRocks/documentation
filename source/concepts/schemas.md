# Schemas & Tables

Yeti uses **GraphQL Schema Definition Language (SDL)** as a declarative way to define tables, fields, indexes, and relationships. This is not a GraphQL API -- it is a schema definition language that Yeti interprets to create database tables and auto-generate REST and GraphQL endpoints.

## Schema Files

Schema files are referenced in `config.yaml` under the `schemas:` key and use the `.graphql` extension:

```yaml
schemas:
  - schema.graphql
```

Each `type` definition annotated with `@table` becomes a database table with full CRUD support.

## A Complete Example

Here is a real schema from the graphql-explorer application showing multiple related tables:

```graphql
type Author @table(database: "graphql-explorer") @export(rest: true, graphql: true) {
    id: ID! @primaryKey
    name: String!
    email: String @indexed
    bio: String
    country: String
    books: [Book] @relationship(to: "authorId")
}

type Book @table(database: "graphql-explorer") @export(rest: true, graphql: true) {
    id: ID! @primaryKey
    title: String!
    isbn: String! @indexed
    publishedYear: Int
    genre: String @indexed
    price: Float
    authorId: ID! @indexed
    author: Author @relationship(from: "authorId")
    reviews: [Review] @relationship(to: "bookId")
}

type Review @table(database: "graphql-explorer") @export(rest: true, graphql: true) {
    id: ID! @primaryKey
    bookId: ID! @indexed
    rating: Int!
    title: String
    content: String
    reviewer: String
    book: Book @relationship(from: "bookId")
}
```

This schema creates three tables with primary keys, secondary indexes, and navigable relationships between them.

## Data Types

| GraphQL Type | Description | Storage |
|-------------|-------------|---------|
| `ID` / `ID!` | Unique identifier (string) | String key |
| `String` / `String!` | UTF-8 text | String |
| `Int` / `Int!` | 64-bit integer | i64 |
| `Float` / `Float!` | 64-bit floating point | f64 |
| `Boolean` / `Boolean!` | True/false | bool |
| `Date` | ISO 8601 date string | String |
| `[Float!]!` | Float array (used for vectors) | Vec\<f64\> |

The `!` suffix marks a field as non-nullable. Fields without `!` are optional and may be omitted when creating records.

## Directives

Directives are annotations on types and fields that control how Yeti processes the schema.

### Type-Level Directives

**`@table(database: "name")`** -- Declares this type as a persistent table stored in the named database. The database name controls storage isolation; multiple tables can share a database.

```graphql
type Product @table(database: "my-app") {
    ...
}
```

**`@export`** -- Makes the table available as an API endpoint. Accepts optional parameters to control which interfaces are enabled:

```graphql
# Export with defaults (REST only)
type Product @table(database: "my-app") @export { ... }

# Export with explicit interface selection
type Product @table(database: "my-app") @export(rest: true, graphql: true) { ... }
```

### Field-Level Directives

**`@primaryKey`** -- Designates the field as the table's primary key. Every table must have exactly one primary key field, typically `id: ID!`.

**`@indexed`** -- Creates a secondary index on the field, enabling fast lookups and FIQL filtering. Index any field you plan to query by:

```graphql
genre: String @indexed
price: Float @indexed
```

**`@relationship`** -- Defines a navigable relationship between tables. Two forms exist:

```graphql
# "from" = this table has the foreign key
author: Author @relationship(from: "authorId")

# "to" = the related table has the foreign key pointing here
books: [Book] @relationship(to: "authorId")
```

The `from` form is for belongs-to relationships (many-to-one). The `to` form is for has-many relationships (one-to-many). Relationships are resolved automatically in both REST and GraphQL queries.

**`@createdTime`** -- Automatically set to the current Unix timestamp when a record is created.

**`@updatedTime`** -- Automatically updated to the current Unix timestamp on every write.

**`@expiresAt`** -- Marks a field as a TTL (time-to-live) expiration timestamp. Records are automatically removed after this time.

## Storage Engine

Tables are stored in [RocksDB](https://rocksdb.org/), a high-performance embedded key-value store. Records are serialized using MessagePack for compact binary encoding. Each `@table(database: "name")` declaration maps to a RocksDB column family, providing logical isolation between databases.

Primary key lookups are sub-millisecond. Secondary indexes (`@indexed`) use prefix-scanned column families for efficient range queries and FIQL filtering.

## Auto-Generated Endpoints

Every table with `@export` automatically gets a full set of REST endpoints:

```
GET    /{app-id}/{TableName}        # List/search records
POST   /{app-id}/{TableName}        # Create a new record
GET    /{app-id}/{TableName}/{id}   # Get a single record
PUT    /{app-id}/{TableName}/{id}   # Create or replace a record
PATCH  /{app-id}/{TableName}/{id}   # Partially update a record
DELETE /{app-id}/{TableName}/{id}   # Delete a record
```

When `graphql: true` is set, the table is also queryable via the GraphQL endpoint at `POST /{app-id}/graphql`.

## Seed Data

Tables can be pre-populated with seed data using the `dataLoader` config field. JSON files matching the glob pattern are loaded on startup:

```yaml
dataLoader: data/*.json
```

Each JSON file should be an array of records matching the table schema. The filename (without extension) corresponds to the table name:

```json
// data/Author.json
[
  { "id": "author-1", "name": "Jane Doe", "email": "jane@example.com" },
  { "id": "author-2", "name": "John Smith", "country": "UK" }
]
```
