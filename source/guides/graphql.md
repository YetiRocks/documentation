# GraphQL

Yeti auto-generates a full GraphQL API from your schema definitions. Every table with `graphql: true` in its config gets query, mutation, and subscription support at the `POST /{app-id}/graphql` endpoint.

## Enabling GraphQL

Set `graphql: true` in your application's `config.yaml`:

```yaml
name: "My App"
app_id: "my-app"
graphql: true
schemas:
  - schema.graphql
```

## Schema with Relationships

The `graphql-explorer` application demonstrates a full relational model:

```graphql
type Author @table(database: "graphql-explorer") @export(rest: true, graphql: true) {
    id: ID! @primaryKey
    name: String!
    email: String @indexed
    bio: String
    books: [Book] @relationship(to: "authorId")
}

type Book @table(database: "graphql-explorer") @export(rest: true, graphql: true) {
    id: ID! @primaryKey
    title: String!
    isbn: String! @indexed
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
    content: String
    book: Book @relationship(from: "bookId")
}
```

## Querying Data

Send a POST request with a JSON body containing the `query` field:

```bash
curl -sk https://localhost:9996/graphql-explorer/graphql \
  -H "Content-Type: application/json" \
  -d '{"query": "{ Book { id title price genre } }"}'
```

### Field Selection

Request only the fields you need. The server returns exactly what you ask for:

```bash
curl -sk https://localhost:9996/graphql-explorer/graphql \
  -H "Content-Type: application/json" \
  -d '{"query": "{ Author { name email } }"}'
```

### Nested Relationship Queries

Fetch a Book with its Author and Reviews in a single query:

```bash
curl -sk https://localhost:9996/graphql-explorer/graphql \
  -H "Content-Type: application/json" \
  -d '{"query": "{ Book { id title price author { name email } reviews { rating content reviewer } } }"}'
```

## Mutations

Create, update, and delete records through GraphQL mutations:

```bash
# Create a record
curl -sk https://localhost:9996/graphql-explorer/graphql \
  -H "Content-Type: application/json" \
  -d '{"query": "mutation { createAuthor(data: {id: \"author-new\", name: \"New Author\", email: \"new@example.com\"}) { id name } }"}'

# Update a record
curl -sk https://localhost:9996/graphql-explorer/graphql \
  -H "Content-Type: application/json" \
  -d '{"query": "mutation { updateAuthor(id: \"author-new\", data: {name: \"Updated Name\"}) { id name } }"}'

# Delete a record
curl -sk https://localhost:9996/graphql-explorer/graphql \
  -H "Content-Type: application/json" \
  -d '{"query": "mutation { deleteAuthor(id: \"author-new\") { id } }"}'
```

## Variables

Pass variables separately from the query for cleaner code:

```bash
curl -sk https://localhost:9996/graphql-explorer/graphql \
  -H "Content-Type: application/json" \
  -d '{
    "query": "mutation CreateBook($data: BookInput!) { createBook(data: $data) { id title } }",
    "variables": {
      "data": {
        "id": "book-new",
        "title": "New Book",
        "isbn": "978-0000000000",
        "authorId": "author-1"
      }
    }
  }'
```

## Introspection

Yeti supports GraphQL introspection for schema discovery. Tools like the GraphQL Explorer UI use this automatically:

```bash
curl -sk https://localhost:9996/graphql-explorer/graphql \
  -H "Content-Type: application/json" \
  -d '{"query": "{ __schema { queryType { name } types { name kind } } }"}'
```

## Subscriptions via SSE

GraphQL subscriptions are delivered as Server-Sent Events. Set the `Accept: text/event-stream` header:

```bash
curl -sk -H "Accept: text/event-stream" \
  -H "Content-Type: application/json" \
  -d '{"query": "subscription { Book { id title price } }"}' \
  https://localhost:9996/graphql-explorer/graphql
```

## GraphQL Explorer UI

The `graphql-explorer` application includes a built-in Apollo-style explorer UI. After starting Yeti, visit `https://localhost:9996/graphql-explorer/` in your browser to interactively build and test queries against the Book/Author/Review schema.

## See Also

- [FIQL Queries](fiql.md) -- REST-based filtering alternative
- [Relationships & Joins](relationships.md) -- Relationship directive reference
- [Server-Sent Events](sse.md) -- SSE subscription details
