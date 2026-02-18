# Data Operations

Yeti provides a rich set of data operations through its REST and GraphQL interfaces. Every table defined with `@export` in your schema automatically gets full CRUD endpoints with support for filtering, pagination, sorting, field selection, and relationship traversal.

## REST Endpoint Pattern

All table endpoints follow this URL structure:

```
https://localhost:9996/{app-id}/{TableName}           # Collection
https://localhost:9996/{app-id}/{TableName}/{id}      # Single record
```

For example, with the `graphql-explorer` application:

```
https://localhost:9996/graphql-explorer/Author         # All authors
https://localhost:9996/graphql-explorer/Author/author-1 # Single author
https://localhost:9996/graphql-explorer/Book            # All books
```

## Development Note

The Yeti development server uses HTTPS with self-signed certificates on port 9996. All `curl` examples in these guides use the `-sk` flags:

- `-s` -- Silent mode (no progress bar)
- `-k` -- Allow insecure connections (accept self-signed certs)

## Guides in This Section

### Core Operations

- [CRUD Operations](crud.md) -- Create, Read, Update, and Delete records with complete curl examples

### Query Features

- [FIQL Queries](fiql.md) -- Filter records using Feed Item Query Language syntax
- [Pagination & Sorting](pagination.md) -- Control result order and page through large datasets
- [Field Selection](field-selection.md) -- Request only the fields you need

### Advanced Features

- [Relationships & Joins](relationships.md) -- Traverse foreign key relationships in REST and GraphQL
- [GraphQL](graphql.md) -- Query and mutate data using the GraphQL endpoint
- [Vector Search](vector-search.md) -- Approximate nearest-neighbor search on vector fields

## Quick Reference

Here is a summary of the query parameters available on all collection endpoints:

| Parameter | Example | Description |
|-----------|---------|-------------|
| FIQL filter | `?genre==Mystery` | Filter by field values |
| `limit` | `?limit=10` | Maximum records to return |
| `offset` | `?offset=20` | Skip N records (for pagination) |
| `sort` | `?sort=name` | Sort ascending by field |
| `sort` | `?sort=-price` | Sort descending (prefix with `-`) |
| `select` | `?select=name,price` | Return only specified fields |

Parameters can be combined:

```bash
curl -sk 'https://localhost:9996/graphql-explorer/Book?genre==Mystery&sort=-price&limit=5&select=title,price'
```

## Function-Style Parameters

Yeti also supports Harper-compatible function-style parameter syntax as an alternative:

```bash
# These are equivalent:
curl -sk 'https://localhost:9996/app/Product?select=name,price&sort=-price&limit=10&offset=5'
curl -sk 'https://localhost:9996/app/Product?select(name,price)&sort(-price)&limit(5,10)'
```

The function-style `limit(offset, count)` combines offset and limit into a single parameter.

## Response Format

Collection endpoints return a JSON array:

```json
[
  {"id": "1", "name": "Widget", "price": 9.99},
  {"id": "2", "name": "Gadget", "price": 19.99}
]
```

Single-record endpoints return a JSON object:

```json
{"id": "1", "name": "Widget", "price": 9.99}
```

Error responses return a JSON object with an `error` field:

```json
{"error": "Record not found: xyz"}
```

## HTTP Methods Summary

| Method | URL | Description |
|--------|-----|-------------|
| `GET` | `/{app}/{Table}` | List records (with optional filters) |
| `GET` | `/{app}/{Table}/{id}` | Get single record by ID |
| `POST` | `/{app}/{Table}` | Create record(s) |
| `PUT` | `/{app}/{Table}/{id}` | Replace entire record |
| `PATCH` | `/{app}/{Table}/{id}` | Partial update (merge fields) |
| `DELETE` | `/{app}/{Table}/{id}` | Delete single record |

See [CRUD Operations](crud.md) for detailed examples of each method.
