# Pagination & Sorting

Yeti provides query parameters for controlling the order and quantity of results returned from collection endpoints. These parameters work alongside FIQL filters and field selection.

## Pagination

### limit

Restricts the maximum number of records returned:

```bash
# Return at most 10 books
curl -sk 'https://localhost:9996/graphql-explorer/Book?limit=10'
```

### offset

Skips the first N records before returning results:

```bash
# Skip the first 20 books, then return up to the default limit
curl -sk 'https://localhost:9996/graphql-explorer/Book?offset=20'
```

### Combined Pagination

Use `limit` and `offset` together for page-based navigation:

```bash
# Page 1 (records 0-9)
curl -sk 'https://localhost:9996/graphql-explorer/Book?limit=10&offset=0'

# Page 2 (records 10-19)
curl -sk 'https://localhost:9996/graphql-explorer/Book?limit=10&offset=10'

# Page 3 (records 20-29)
curl -sk 'https://localhost:9996/graphql-explorer/Book?limit=10&offset=20'
```

### Function-Style Syntax

The `limit()` function-style parameter combines offset and limit:

```bash
# Equivalent to offset=20&limit=10
curl -sk 'https://localhost:9996/graphql-explorer/Book?limit(20,10)'

# Just limit (no offset)
curl -sk 'https://localhost:9996/graphql-explorer/Book?limit(10)'
```

## Sorting

### Ascending Sort

Use `sort=field` to sort results in ascending order:

```bash
# Sort books by title A-Z
curl -sk 'https://localhost:9996/graphql-explorer/Book?sort=title'

# Sort by published year (oldest first)
curl -sk 'https://localhost:9996/graphql-explorer/Book?sort=publishedYear'
```

### Descending Sort

Prefix the field name with `-` for descending order:

```bash
# Sort books by price, highest first
curl -sk 'https://localhost:9996/graphql-explorer/Book?sort=-price'

# Most recently published first
curl -sk 'https://localhost:9996/graphql-explorer/Book?sort=-publishedYear'
```

### Multi-Field Sort

Specify multiple fields separated by commas. Records are sorted by the first field, then by the second field for ties, and so on:

```bash
# Sort by genre ascending, then by price descending within each genre
curl -sk 'https://localhost:9996/graphql-explorer/Book?sort=genre,-price'
```

### Function-Style Sort

```bash
# Equivalent to sort=-price
curl -sk 'https://localhost:9996/graphql-explorer/Book?sort(-price)'

# Multi-field
curl -sk 'https://localhost:9996/graphql-explorer/Book?sort(genre,-price)'
```

## Combining Everything

Pagination, sorting, and FIQL filters compose naturally:

```bash
# Science fiction books, sorted by price descending, page 2 (10 per page)
curl -sk 'https://localhost:9996/graphql-explorer/Book?genre==Science%20Fiction&sort=-price&limit=10&offset=10'
```

Or with function-style syntax:

```bash
curl -sk 'https://localhost:9996/graphql-explorer/Book?genre==Science%20Fiction&sort(-price)&limit(10,10)'
```

## Response Format

Collection endpoints always return a JSON array. An empty result set returns an empty array:

```json
[]
```

A paginated result set:

```json
[
  {"id": "book-3", "title": "Foundation", "price": 15.99, "genre": "Science Fiction"},
  {"id": "book-4", "title": "I, Robot", "price": 14.99, "genre": "Science Fiction"}
]
```

## Pagination Strategy

Yeti uses offset-based pagination. For typical use cases:

| Use Case | Parameters |
|----------|-----------|
| First page | `?limit=25` |
| Next page | `?limit=25&offset=25` |
| Specific page N | `?limit=25&offset={(N-1)*25}` |
| Top 5 most expensive | `?sort=-price&limit=5` |
| Newest 10 | `?sort=-createdAt&limit=10` |

## Example: Building a Paginated List

Here is a sequence of requests to browse all books, 5 at a time, sorted by title:

```bash
# Page 1
curl -sk 'https://localhost:9996/graphql-explorer/Book?sort=title&limit=5&offset=0'

# Page 2
curl -sk 'https://localhost:9996/graphql-explorer/Book?sort=title&limit=5&offset=5'

# Page 3
curl -sk 'https://localhost:9996/graphql-explorer/Book?sort=title&limit=5&offset=10'
```

When the response returns fewer records than the `limit` (or an empty array), you have reached the last page.

## Tips

- **Default ordering**: Without a `sort` parameter, records are returned in storage order (typically insertion order).
- **Null values in sort**: Records with null values for the sort field are typically sorted last in ascending order and first in descending order.
- **Large offsets**: For very large datasets, high offset values may be slow. Consider using a cursor-based approach by filtering on the last seen ID or timestamp.
