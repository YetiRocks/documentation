# FIQL Queries

FIQL (Feed Item Query Language) is a URI-friendly query syntax used for filtering records in Yeti REST endpoints. FIQL filters are applied as query parameters on collection endpoints.

## Basic Syntax

Filters are appended to the URL as query parameters in the form `field==value`:

```
GET /{app-id}/{Table}?field==value
```

Note the double equals (`==`). A single `=` is used internally by the query string parser; the double `==` is the FIQL equality operator.

## Comparison Operators

| Operator | Meaning | Example |
|----------|---------|---------|
| `==` | Equals (with type coercion) | `name==Widget` |
| `===` | Strict equals (no coercion) | `id===prod-001` |
| `!=` | Not equals | `category!=hardware` |
| `!==` | Strict not equals | `count!==5` |
| `=gt=` or `>` | Greater than | `price=gt=10` |
| `=ge=` or `>=` | Greater than or equal | `price=ge=10` |
| `=lt=` or `<` | Less than | `price=lt=20` |
| `=le=` or `<=` | Less than or equal | `price=le=20` |
| `=ct=` | Contains substring | `name=ct=Widget` |
| `=sw=` | Starts with | `name=sw=Ultra` |
| `=ew=` | Ends with | `name=ew=Pro` |
| `=~=` | Regex match | `name=~=^Ultra.*` |
| `=in=` | In set | `category=in=hardware,electronics` |
| `=out=` | Not in set | `category=out=food,clothing` |
| `=ft=` | Full-text search | `name=ft=ultra widget` |
| `=gele=` | Range (inclusive both) | `price=gele=10,100` |
| `=gtlt=` | Range (exclusive both) | `price=gtlt=10,100` |

## Wildcard Patterns

Wildcard patterns use `*` within the equality operator:

| Pattern | Meaning | Example |
|---------|---------|---------|
| `==value*` | Starts with | `name==Ultra*` |
| `==*value` | Ends with | `name==*Pro` |
| `==*value*` | Contains | `name==*Widget*` |

## Logical Operators

| Operator | Meaning | Example |
|----------|---------|---------|
| `&` | AND | `category==hardware&price=lt=20` |
| `\|` | OR | `category==hardware\|category==electronics` |
| `!` | NOT (prefix) | `!(price=gt=100)` |

AND has higher precedence than OR. Use parentheses for grouping.

## Examples

### Equality Filter

```bash
# Find all books in the Mystery genre
curl -sk 'https://localhost:9996/graphql-explorer/Book?genre==Mystery'
```

### Not Equals

```bash
# All books except Romance genre
curl -sk 'https://localhost:9996/graphql-explorer/Book?genre!=Romance'
```

### Range Queries

```bash
# Books published after 1950
curl -sk 'https://localhost:9996/graphql-explorer/Book?publishedYear=gt=1950'

# Books priced between $10 and $15
curl -sk 'https://localhost:9996/graphql-explorer/Book?price=ge=10&price=le=15'

# Range operator shorthand (inclusive both ends)
curl -sk 'https://localhost:9996/graphql-explorer/Book?price=gele=10,15'
```

### Set Membership

```bash
# Books in either Mystery or Romance genres
curl -sk 'https://localhost:9996/graphql-explorer/Book?genre=in=Mystery,Romance'

# Books NOT in Science Fiction or Magical Realism
curl -sk 'https://localhost:9996/graphql-explorer/Book?genre=out=Science%20Fiction,Magical%20Realism'
```

### Regex Match

```bash
# Books with titles starting with "The"
curl -sk 'https://localhost:9996/graphql-explorer/Book?title=~=^The'

# Case-insensitive match for "great"
curl -sk 'https://localhost:9996/graphql-explorer/Book?title=~=(?i)great'
```

### NOT (Negation)

```bash
# All books where price is NOT greater than 15
curl -sk 'https://localhost:9996/graphql-explorer/Book?!(price=gt=15)'

# Not (Mystery AND cheap) — exclude cheap mysteries
curl -sk 'https://localhost:9996/graphql-explorer/Book?!(genre==Mystery&price=lt=10)'
```

### Full-Text Search

The `=ft=` operator performs full-text search with AND semantics (all terms must match):

```bash
# Products with both "ultra" and "monitor" in the name
curl -sk 'https://localhost:9996/example-queries/Products?name=ft=ultra%20monitor'

# Products with "programming" in the description
curl -sk 'https://localhost:9996/example-queries/Products?description=ft=programming'
```

Full-text search tokenizes text by splitting on non-alphanumeric characters, lowercasing, and filtering tokens shorter than 2 characters. Fields must be annotated with `@indexed(type: "fulltext")` in the schema for index-accelerated queries.

### AND Conditions

Multiple conditions joined by `&` must all be true:

```bash
# Mystery books priced under $14
curl -sk 'https://localhost:9996/graphql-explorer/Book?genre==Mystery&price=lt=14'
```

### OR Conditions

Conditions joined by `|` match if any is true:

```bash
# Books by author-1 OR author-2
curl -sk 'https://localhost:9996/graphql-explorer/Book?authorId==author-1|authorId==author-2'
```

### Grouped Expressions

Use parentheses to control precedence:

```bash
# (Mystery OR Romance) AND price under $14
curl -sk 'https://localhost:9996/graphql-explorer/Book?(genre==Mystery|genre==Romance)&price=lt=14'
```

## Combining with Other Parameters

FIQL filters work alongside pagination, sorting, and field selection:

```bash
# Mystery books, sorted by price descending, first 5 results, only title and price
curl -sk 'https://localhost:9996/graphql-explorer/Book?genre==Mystery&sort=-price&limit=5&select=title,price'
```

## URL Encoding

Special characters in filter values must be URL-encoded:

| Character | Encoded |
|-----------|---------|
| Space | `%20` |
| `/` | `%2F` |
| `+` | `%2B` |
| `@` | `%40` |

```bash
# Filter by genre containing a space
curl -sk 'https://localhost:9996/graphql-explorer/Book?genre==Science%20Fiction'
```

Most command-line tools and HTTP libraries handle URL encoding automatically when you pass parameters properly.

## Indexed vs Non-Indexed Fields

FIQL queries work on all fields, but performance differs:

- **@indexed fields**: Queries use the secondary index for fast lookups. Ideal for fields used frequently in filters.
- **Non-indexed fields**: Queries require a full table scan. Acceptable for small tables but may be slow for large datasets.

For the best performance, add `@indexed` to fields that are commonly used in FIQL filters:

```graphql
type Book @table @export {
    id: ID! @primaryKey
    genre: String @indexed        # Fast FIQL filtering
    publishedYear: Int            # Scan-based filtering (slower on large tables)
}
```

## Function-Style Alternative

Yeti also accepts function-style syntax for sort, select, and limit parameters (not for FIQL comparisons):

```bash
# These are equivalent
curl -sk 'https://localhost:9996/app/Product?category==electronics&select=name,price&sort=-price&limit=10'
curl -sk 'https://localhost:9996/app/Product?category==electronics&select(name,price)&sort(-price)&limit(10)'
```
