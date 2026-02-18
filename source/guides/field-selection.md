# Field Selection

The `select` parameter lets you specify which fields to include in the response. This reduces payload size by excluding unnecessary data, which is especially valuable for tables with many fields or large text columns.

## Basic Usage

Add `select=field1,field2` to your query parameters:

```bash
# Return only title and price for each book
curl -sk 'https://localhost:9996/graphql-explorer/Book?select=title,price'
```

Response:

```json
[
  {"title": "Pride and Prejudice", "price": 12.99},
  {"title": "Sense and Sensibility", "price": 11.99},
  {"title": "Foundation", "price": 15.99},
  {"title": "I, Robot", "price": 14.99}
]
```

Without `select`, the full record is returned:

```json
[
  {
    "id": "book-1",
    "title": "Pride and Prejudice",
    "isbn": "978-0141439518",
    "publishedYear": 1813,
    "genre": "Romance",
    "price": 12.99,
    "authorId": "author-1",
    "publisherId": "pub-1"
  }
]
```

## Including the ID

The primary key is not automatically included when using `select`. Include it explicitly if you need it:

```bash
curl -sk 'https://localhost:9996/graphql-explorer/Book?select=id,title,price'
```

Response:

```json
[
  {"id": "book-1", "title": "Pride and Prejudice", "price": 12.99},
  {"id": "book-2", "title": "Sense and Sensibility", "price": 11.99}
]
```

## Function-Style Syntax

You can also use the function-style syntax:

```bash
curl -sk 'https://localhost:9996/graphql-explorer/Book?select(id,title,price)'
```

This is equivalent to `select=id,title,price`.

## Combining with Filters and Sorting

Field selection composes with all other query parameters:

```bash
# Mystery books, sorted by price, returning only title and price
curl -sk 'https://localhost:9996/graphql-explorer/Book?genre==Mystery&sort=-price&select=title,price'
```

Response:

```json
[
  {"title": "Murder on the Orient Express", "price": 13.99},
  {"title": "And Then There Were None", "price": 12.99}
]
```

### With Pagination

```bash
# Page 1 of books with only essential fields
curl -sk 'https://localhost:9996/graphql-explorer/Book?select=id,title,genre&limit=5&offset=0'
```

### Complex Query

```bash
# Electronics priced over $10, sorted by name, fields: id, name, price, page 2
curl -sk 'https://localhost:9996/my-app/Product?category==electronics&price=gt=10&sort=name&select=id,name,price&limit=10&offset=10'
```

## Use Cases

### API Efficiency

Select only the fields needed for a specific view:

```bash
# List view: just names
curl -sk 'https://localhost:9996/graphql-explorer/Author?select=id,name'

# Detail view: all fields (no select parameter)
curl -sk 'https://localhost:9996/graphql-explorer/Author/author-1'
```

### Excluding Large Fields

When a table has large text or blob fields, select the smaller fields for list views:

```bash
# Authors list without the full bio text
curl -sk 'https://localhost:9996/graphql-explorer/Author?select=id,name,email,country'
```

Response:

```json
[
  {"id": "author-1", "name": "Jane Austen", "email": "jane@austen.lit", "country": "England"},
  {"id": "author-2", "name": "Isaac Asimov", "email": "isaac@foundation.org", "country": "USA"}
]
```

### Dropdown / Autocomplete Data

For UI components that only need an ID and display label:

```bash
curl -sk 'https://localhost:9996/graphql-explorer/Category?select=id,name'
```

## Permissions Interaction

When an application uses role-based access control with attribute-level permissions, the `select` parameter interacts with the permission system:

- Fields the user is not authorized to read are excluded from the response regardless of the `select` parameter.
- Requesting a field you do not have permission to see does not produce an error; the field is simply omitted.

This means you can safely use the same `select` parameters for all users -- the permission system handles field-level filtering automatically.

## Single Record Requests

Field selection also works on single-record endpoints:

```bash
curl -sk 'https://localhost:9996/graphql-explorer/Book/book-1?select=title,price,genre'
```

Response:

```json
{"title": "Pride and Prejudice", "price": 12.99, "genre": "Romance"}
```

## Tips

- **No wildcard**: There is no `select=*` syntax. Omit the `select` parameter entirely to get all fields.
- **Case-sensitive**: Field names must match the schema exactly (typically camelCase).
- **Invalid fields**: Selecting a field that does not exist in the schema produces no error; it is simply not in the response.
- **Performance**: Field selection reduces response payload size but does not currently skip reading fields from storage. The filtering happens at the serialization layer.
