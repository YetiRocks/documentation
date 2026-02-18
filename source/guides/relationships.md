# Relationships & Joins

Yeti supports declaring foreign key relationships in your GraphQL schema using the `@relationship` directive. Relationships enable nested data retrieval through both the GraphQL and REST interfaces.

## Relationship Patterns

There are two patterns, defined by the `from` and `to` arguments.

### from -- Many-to-One (Lookup Parent)

A `from` relationship resolves a foreign key field on the current record to the referenced record in another table. It answers the question "who is the parent of this record?"

```graphql
type Book @table(database: "graphql-explorer") @export(rest: true, graphql: true) {
    id: ID! @primaryKey
    title: String!
    authorId: ID! @indexed
    # Resolve authorId to the matching Author record
    author: Author @relationship(from: "authorId")
}
```

When you query a Book, the `author` field resolves by looking up the Author whose `id` matches the Book's `authorId`.

**Data flow**: `Book.authorId` --> find `Author` where `Author.id == Book.authorId`

### to -- One-to-Many (Find Children)

A `to` relationship finds all records in another table that reference the current record. It answers "which records point to me?"

```graphql
type Author @table(database: "graphql-explorer") @export(rest: true, graphql: true) {
    id: ID! @primaryKey
    name: String!
    # Find all Books where Book.authorId == this Author's id
    books: [Book] @relationship(to: "authorId")
}
```

When you query an Author, the `books` field resolves by finding all Books whose `authorId` matches the Author's `id`.

**Data flow**: find all `Book` records where `Book.authorId == Author.id`

## Complete Schema Example

The `graphql-explorer` application demonstrates both patterns across multiple types:

```graphql
type Author @table(database: "graphql-explorer") @export(rest: true, graphql: true) {
    id: ID! @primaryKey
    name: String!
    email: String @indexed
    bio: String
    country: String
    books: [Book] @relationship(to: "authorId")         # one-to-many
}

type Publisher @table(database: "graphql-explorer") @export(rest: true, graphql: true) {
    id: ID! @primaryKey
    name: String!
    founded: Int
    headquarters: String
    books: [Book] @relationship(to: "publisherId")      # one-to-many
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
    author: Author @relationship(from: "authorId")       # many-to-one
    publisher: Publisher @relationship(from: "publisherId") # many-to-one
    reviews: [Review] @relationship(to: "bookId")        # one-to-many
}

type Review @table(database: "graphql-explorer") @export(rest: true, graphql: true) {
    id: ID! @primaryKey
    bookId: ID! @indexed
    rating: Int!
    title: String
    content: String
    reviewer: String
    book: Book @relationship(from: "bookId")             # many-to-one
}
```

This creates a relationship graph:

```
Author --[1:N]--> Book --[1:N]--> Review
Publisher --[1:N]--> Book
```

## GraphQL Nested Queries

The primary way to traverse relationships is through GraphQL. Relationships map directly to nested fields:

### Many-to-One (fetch parent)

```graphql
{
  Book {
    title
    price
    author {
      name
      country
    }
    publisher {
      name
    }
  }
}
```

```bash
curl -sk https://localhost:9996/graphql-explorer/graphql \
  -X POST \
  -H "Content-Type: application/json" \
  -d '{"query": "{ Book { title price author { name country } publisher { name } } }"}'
```

Response:

```json
{
  "data": {
    "Book": [
      {
        "title": "Pride and Prejudice",
        "price": 12.99,
        "author": {"name": "Jane Austen", "country": "England"},
        "publisher": {"name": "Penguin Classics"}
      },
      {
        "title": "Foundation",
        "price": 15.99,
        "author": {"name": "Isaac Asimov", "country": "USA"},
        "publisher": {"name": "Bantam Books"}
      }
    ]
  }
}
```

### One-to-Many (fetch children)

```graphql
{
  Author {
    name
    books {
      title
      genre
      price
    }
  }
}
```

```bash
curl -sk https://localhost:9996/graphql-explorer/graphql \
  -X POST \
  -H "Content-Type: application/json" \
  -d '{"query": "{ Author { name books { title genre price } } }"}'
```

Response:

```json
{
  "data": {
    "Author": [
      {
        "name": "Jane Austen",
        "books": [
          {"title": "Pride and Prejudice", "genre": "Romance", "price": 12.99},
          {"title": "Sense and Sensibility", "genre": "Romance", "price": 11.99}
        ]
      },
      {
        "name": "Isaac Asimov",
        "books": [
          {"title": "Foundation", "genre": "Science Fiction", "price": 15.99},
          {"title": "I, Robot", "genre": "Science Fiction", "price": 14.99}
        ]
      }
    ]
  }
}
```

### Deep Nesting

Relationships can be nested to any depth:

```graphql
{
  Author {
    name
    books {
      title
      reviews {
        rating
        reviewer
        content
      }
    }
  }
}
```

This fetches authors, their books, and each book's reviews in a single query.

## REST API Relationships

In the REST API, relationship fields are typically not resolved by default on collection endpoints (to avoid N+1 query overhead). The foreign key fields (e.g., `authorId`) are always present.

To get related data via REST, make a follow-up request using the foreign key:

```bash
# Get a book
curl -sk https://localhost:9996/graphql-explorer/Book/book-1
# Response: {"id": "book-1", "title": "Pride and Prejudice", "authorId": "author-1", ...}

# Follow the relationship manually
curl -sk https://localhost:9996/graphql-explorer/Author/author-1
# Response: {"id": "author-1", "name": "Jane Austen", ...}
```

For one-to-many from REST, filter by the foreign key:

```bash
# Get all books by author-1
curl -sk 'https://localhost:9996/graphql-explorer/Book?authorId==author-1'

# Get all reviews for book-1
curl -sk 'https://localhost:9996/graphql-explorer/Review?bookId==book-1'
```

## Best Practices

- **Index foreign keys**: Always add `@indexed` to foreign key fields (e.g., `authorId: ID! @indexed`). Without an index, `to` relationships require a full table scan.
- **Use GraphQL for nested data**: When you need related data in a single request, prefer the GraphQL endpoint over multiple REST calls.
- **Avoid circular depth**: While deep nesting is supported, queries like `Author -> books -> author -> books` can be expensive. Keep nesting practical.
- **Nullable foreign keys**: Use `ID` (nullable) for optional relationships and `ID!` (non-nullable) for required ones.
