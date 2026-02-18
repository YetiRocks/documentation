# CRUD Operations

This guide covers the four core data operations -- Create, Read, Update, Delete -- using the REST API. All examples use a `Product` table in an application with `app_id: "my-app"`.

Schema for reference:

```graphql
type Product @table @export {
    id: ID! @primaryKey
    name: String!
    price: Float
    category: String @indexed
    inStock: Boolean
}
```

## Create (POST)

### Single Record

```bash
curl -sk https://localhost:9996/my-app/Product \
  -X POST \
  -H "Content-Type: application/json" \
  -d '{
    "id": "prod-1",
    "name": "Widget",
    "price": 9.99,
    "category": "hardware",
    "inStock": true
  }'
```

Response (201 Created):

```json
{
  "id": "prod-1",
  "name": "Widget",
  "price": 9.99,
  "category": "hardware",
  "inStock": true
}
```

### Auto-Generated ID

Omit the `id` field and Yeti generates a UUID:

```bash
curl -sk https://localhost:9996/my-app/Product \
  -X POST \
  -H "Content-Type: application/json" \
  -d '{"name": "Gadget", "price": 19.99, "category": "electronics"}'
```

### Batch Create

POST an array to insert multiple records:

```bash
curl -sk https://localhost:9996/my-app/Product \
  -X POST \
  -H "Content-Type: application/json" \
  -d '[
    {"id": "prod-2", "name": "Gadget", "price": 19.99, "category": "electronics"},
    {"id": "prod-3", "name": "Doohickey", "price": 4.99, "category": "hardware"}
  ]'
```

## Read (GET)

### List All Records

```bash
curl -sk https://localhost:9996/my-app/Product
```

Response (200 OK):

```json
[
  {"id": "prod-1", "name": "Widget", "price": 9.99, "category": "hardware", "inStock": true},
  {"id": "prod-2", "name": "Gadget", "price": 19.99, "category": "electronics", "inStock": null},
  {"id": "prod-3", "name": "Doohickey", "price": 4.99, "category": "hardware", "inStock": null}
]
```

### Get Single Record

Append the record ID to the URL:

```bash
curl -sk https://localhost:9996/my-app/Product/prod-1
```

Response (200 OK):

```json
{
  "id": "prod-1",
  "name": "Widget",
  "price": 9.99,
  "category": "hardware",
  "inStock": true
}
```

If the record does not exist, you get a 404:

```json
{"error": "Record not found: prod-999"}
```

### Filtered List

Use FIQL query syntax to filter (see [FIQL Queries](fiql.md) for full syntax):

```bash
# All hardware products
curl -sk 'https://localhost:9996/my-app/Product?category==hardware'

# Products over $10
curl -sk 'https://localhost:9996/my-app/Product?price=gt=10'
```

## Update (PUT) -- Full Replace

PUT replaces the entire record. All fields must be provided.

```bash
curl -sk https://localhost:9996/my-app/Product/prod-1 \
  -X PUT \
  -H "Content-Type: application/json" \
  -d '{
    "id": "prod-1",
    "name": "Widget Pro",
    "price": 14.99,
    "category": "hardware",
    "inStock": true
  }'
```

Response (200 OK):

```json
{
  "id": "prod-1",
  "name": "Widget Pro",
  "price": 14.99,
  "category": "hardware",
  "inStock": true
}
```

Fields omitted in a PUT request will be removed from the record (set to null).

## Update (PATCH) -- Partial Update

PATCH merges the provided fields into the existing record. Only the fields you send are updated; all other fields are preserved.

```bash
curl -sk https://localhost:9996/my-app/Product/prod-1 \
  -X PATCH \
  -H "Content-Type: application/json" \
  -d '{"price": 12.99, "inStock": false}'
```

Response (200 OK):

```json
{
  "id": "prod-1",
  "name": "Widget Pro",
  "price": 12.99,
  "category": "hardware",
  "inStock": false
}
```

The `name` and `category` fields are unchanged because they were not included in the PATCH body.

## Delete (DELETE)

### Single Record

```bash
curl -sk https://localhost:9996/my-app/Product/prod-3 -X DELETE
```

Response (204 No Content) -- empty body on success.

If the record does not exist:

```json
{"error": "Record not found: prod-3"}
```

## HTTP Status Codes

| Status | Meaning |
|--------|---------|
| 200 | Success (GET, PUT, PATCH) |
| 201 | Created (POST) |
| 204 | No Content (DELETE success) |
| 400 | Bad Request (invalid JSON, missing required fields) |
| 404 | Not Found (record does not exist) |
| 405 | Method Not Allowed |
| 500 | Internal Server Error |

## Computed Fields

If your schema uses `@createdTime` or `@updatedTime` directives, these fields are set automatically:

- **@createdTime**: Set on POST only. Not modified on PUT or PATCH.
- **@updatedTime**: Set on POST, PUT, and PATCH. Updated on every write.

```graphql
type Article @table @export {
    id: ID! @primaryKey
    title: String!
    createdAt: String @createdTime
    updatedAt: String @updatedTime
}
```

```bash
# POST sets both createdAt and updatedAt
curl -sk https://localhost:9996/my-app/Article \
  -X POST \
  -H "Content-Type: application/json" \
  -d '{"title": "Hello World"}'

# PATCH updates only updatedAt
curl -sk https://localhost:9996/my-app/Article/abc-123 \
  -X PATCH \
  -H "Content-Type: application/json" \
  -d '{"title": "Hello World (edited)"}'
```
