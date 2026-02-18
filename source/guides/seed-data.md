# Seed Data & Data Loading

Yeti supports loading initial data into tables at startup from JSON files. This is useful for development, demos, and bootstrapping applications with reference data.

## JSON Format

Each seed data file is a JSON object with three fields:

```json
{
  "database": "graphql-explorer",
  "table": "Author",
  "records": [
    {
      "id": "author-1",
      "name": "Jane Austen",
      "email": "jane@austen.lit",
      "bio": "English novelist known for her six major novels.",
      "country": "England"
    },
    {
      "id": "author-2",
      "name": "Isaac Asimov",
      "email": "isaac@foundation.org",
      "bio": "American writer and professor of biochemistry.",
      "country": "USA"
    }
  ]
}
```

| Field | Description |
|-------|-------------|
| `database` | The database name as defined in `@table(database: "...")` |
| `table` | The type name from your schema (PascalCase) |
| `records` | Array of JSON objects matching your schema fields |

Each record must include the primary key field. Other fields are optional and follow the schema definition.

## Configuration

Reference seed data files in your `config.yaml` using the `dataLoader` field with a glob pattern:

```yaml
dataLoader: data/*.json
```

This loads all `.json` files in the `data/` directory at startup. Files are processed in filesystem order.

## File Organization

A common pattern is one file per table, named after the table in lowercase:

```
~/yeti/applications/my-app/
  data/
    authors.json
    books.json
    categories.json
    publishers.json
    reviews.json
```

The `graphql-explorer` application uses exactly this pattern to populate its five tables.

## Change Detection

Yeti uses change detection on seed data files. Records are only written to storage when the data file has changed since the last load. This means:

- First startup: all records are inserted
- Subsequent startups: records are only re-loaded if the JSON file was modified
- Unchanged files are skipped for faster startup

## Example: Complete Seed Data File

Here is a real seed data file from the `graphql-explorer` application (`data/books.json`):

```json
{
  "database": "data",
  "table": "Book",
  "records": [
    {
      "id": "book-1",
      "title": "Pride and Prejudice",
      "isbn": "978-0141439518",
      "publishedYear": 1813,
      "genre": "Romance",
      "price": 12.99,
      "authorId": "author-1",
      "publisherId": "pub-1"
    },
    {
      "id": "book-2",
      "title": "Foundation",
      "isbn": "978-0553293357",
      "publishedYear": 1951,
      "genre": "Science Fiction",
      "price": 15.99,
      "authorId": "author-2",
      "publisherId": "pub-2"
    }
  ]
}
```

## Bulk Loading via REST API

Beyond seed data files, you can load data at runtime using the REST API:

### Single Record

```bash
curl -sk https://localhost:9996/my-app/Product \
  -X POST \
  -H "Content-Type: application/json" \
  -d '{"id": "prod-1", "name": "Widget", "price": 9.99}'
```

### Multiple Records (Batch)

POST an array of records to insert multiple items in a single request:

```bash
curl -sk https://localhost:9996/my-app/Product \
  -X POST \
  -H "Content-Type: application/json" \
  -d '[
    {"id": "prod-1", "name": "Widget", "price": 9.99},
    {"id": "prod-2", "name": "Gadget", "price": 19.99},
    {"id": "prod-3", "name": "Doohickey", "price": 4.99}
  ]'
```

## CSV Loading

Yeti also supports CSV data loading through the REST endpoint. POST CSV data with the appropriate content type:

```bash
curl -sk https://localhost:9996/my-app/Product \
  -X POST \
  -H "Content-Type: text/csv" \
  -d 'id,name,price
prod-1,Widget,9.99
prod-2,Gadget,19.99
prod-3,Doohickey,4.99'
```

## Tips

- **Primary keys are required**: Every record must include the primary key field defined in your schema.
- **Database name must match**: The `database` field in the JSON file must match the `database` parameter in your `@table` directive. If you omit the `database` parameter in the schema, it defaults to the app_id.
- **Field names are case-sensitive**: Use the exact casing from your GraphQL schema (typically camelCase).
- **Relationships are by ID**: For related records, include the foreign key value (e.g., `"authorId": "author-1"`), not the full related object.
- **Order matters for foreign keys**: If table B references table A, load A's data file first (alphabetical file naming helps).
