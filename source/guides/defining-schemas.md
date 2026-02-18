# Defining Schemas

Yeti uses GraphQL schema definitions with custom directives to define your data model. Each `type` with a `@table` directive becomes a table in the storage engine, and each `@export` directive makes it available as a REST or GraphQL endpoint.

## Running Example

Throughout this guide, we use the `graphql-explorer` application schema: a book catalog with Authors, Publishers, Books, Reviews, and Categories.

## Core Directives

### @table

Declares a type as a persistent table. Without `@table`, a type is just a GraphQL type definition with no storage.

```graphql
# Basic table (uses default database named after app_id)
type Product @table {
    id: ID! @primaryKey
    name: String!
}

# Table with explicit database name
type Author @table(database: "graphql-explorer") {
    id: ID! @primaryKey
    name: String!
}

# Table with TTL expiration (seconds)
type PageCache @table(database: "full-page-caching", expiration: 3600) {
    path: String! @primaryKey
    pageContents: String
}
```

The `database` parameter groups tables into logical databases. The `expiration` parameter sets a time-to-live in seconds -- records are automatically removed after this duration.

### @export

Controls how the table is exposed as an API endpoint.

```graphql
# Export as both REST and GraphQL
type Author @table(database: "graphql-explorer") @export(rest: true, graphql: true) {
    id: ID! @primaryKey
    name: String!
}

# Export with default settings (both REST and GraphQL)
type Product @table @export {
    id: ID! @primaryKey
    name: String!
}
```

Without `@export`, the table exists in storage but has no HTTP endpoint. You can still access it from custom resources via `ctx.get_table("Name")`.

### @primaryKey

Designates the primary key field. Every table must have exactly one.

```graphql
type Author @table @export {
    id: ID! @primaryKey       # Standard auto-ID field
    name: String!
}

type PageCache @table @export {
    path: String! @primaryKey  # Custom primary key (non-ID type)
    pageContents: String
}
```

The primary key is used for single-record lookups: `GET /{app-id}/{Table}/{id}`.

### @indexed

Creates a secondary index on a field for efficient filtering and querying.

```graphql
type Book @table(database: "graphql-explorer") @export(rest: true, graphql: true) {
    id: ID! @primaryKey
    title: String!
    isbn: String! @indexed            # Standard index
    genre: String @indexed            # Enables FIQL filtering on genre
    authorId: ID! @indexed            # Foreign key index
    publisherId: ID @indexed
}
```

Standard indexes support equality, range, and FIQL queries.

#### HNSW Vector Index

For vector similarity search, use the HNSW index type:

```graphql
type Document @table @export {
    id: ID! @primaryKey
    content: String
    embedding: [Float] @indexed(type: "HNSW", optimizeRouting: 0.6)
}
```

HNSW (Hierarchical Navigable Small World) indexes enable approximate nearest-neighbor search on vector fields. The `optimizeRouting` parameter (0.0 to 1.0) trades index build time for query accuracy.

### @relationship

Defines foreign key relationships between types. There are two patterns:

#### `from` -- Many-to-One (lookup parent)

The field resolves to a single related record by looking up a foreign key value in the other table.

```graphql
type Book @table(database: "graphql-explorer") @export(rest: true, graphql: true) {
    id: ID! @primaryKey
    authorId: ID! @indexed
    # "from" means: look up Author where Author.id == this.authorId
    author: Author @relationship(from: "authorId")
}
```

#### `to` -- One-to-Many (find children)

The field resolves to a list of related records by finding records in the other table that reference this record.

```graphql
type Author @table(database: "graphql-explorer") @export(rest: true, graphql: true) {
    id: ID! @primaryKey
    name: String!
    # "to" means: find all Books where Book.authorId == this.id
    books: [Book] @relationship(to: "authorId")
}
```

See [Relationships & Joins](relationships.md) for detailed query examples.

### @createdTime

Automatically sets the field value to the current timestamp when a record is first created. The field is not updated on subsequent writes.

```graphql
type Event @table @export {
    id: ID! @primaryKey
    name: String!
    createdAt: String @createdTime
}
```

### @updatedTime

Automatically sets the field value to the current timestamp on every write (both insert and update).

```graphql
type Post @table @export {
    id: ID! @primaryKey
    title: String!
    updatedAt: String @updatedTime
}
```

You can combine both on the same type:

```graphql
type Document @table @export {
    id: ID! @primaryKey
    content: String
    createdAt: String @createdTime
    modifiedAt: String @updatedTime
}
```

### @expiresAt

Marks a field as the TTL expiration timestamp. Records are automatically removed when the timestamp is reached.

```graphql
type Session @table @export {
    id: ID! @primaryKey
    userId: String!
    expiresAt: Int @expiresAt
}
```

## Field Types

| GraphQL Type | Description | Example |
|-------------|-------------|---------|
| `ID!` | Non-nullable identifier | Primary keys, foreign keys |
| `String` | Nullable string | Names, descriptions |
| `String!` | Non-nullable string | Required text fields |
| `Int` | Nullable integer | Counts, years |
| `Float` | Nullable floating-point | Prices, ratings |
| `Boolean` | Nullable boolean | Flags, toggles |
| `[Float]` | Float array | Vector embeddings |
| `[String]` | String array | Tags, categories |

## Complete Example

Here is the full `graphql-explorer` schema demonstrating all common patterns:

```graphql
type Author @table(database: "graphql-explorer") @export(rest: true, graphql: true) {
    id: ID! @primaryKey
    name: String!
    email: String @indexed
    bio: String
    country: String
    books: [Book] @relationship(to: "authorId")
}

type Publisher @table(database: "graphql-explorer") @export(rest: true, graphql: true) {
    id: ID! @primaryKey
    name: String!
    founded: Int
    headquarters: String
    books: [Book] @relationship(to: "publisherId")
}

type Book @table(database: "graphql-explorer") @export(rest: true, graphql: true) {
    id: ID! @primaryKey
    title: String!
    isbn: String! @indexed
    publishedYear: Int
    genre: String @indexed
    price: Float
    authorId: ID! @indexed
    publisherId: ID @indexed
    author: Author @relationship(from: "authorId")
    publisher: Publisher @relationship(from: "publisherId")
    reviews: [Review] @relationship(to: "bookId")
}

type Review @table(database: "graphql-explorer") @export(rest: true, graphql: true) {
    id: ID! @primaryKey
    bookId: ID! @indexed
    rating: Int!
    title: String
    content: String
    reviewer: String
    createdAt: String
    book: Book @relationship(from: "bookId")
}

type Category @table(database: "graphql-explorer") @export(rest: true, graphql: true) {
    id: ID! @primaryKey
    name: String! @indexed
    description: String
    parentId: ID @indexed
}
```
