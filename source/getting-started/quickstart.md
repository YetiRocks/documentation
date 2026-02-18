# Quickstart

Build a REST API in 5 minutes.

## What You'll Build

A simple REST API with automatic CRUD endpoints, FIQL filtering, and real-time subscriptions — all from a GraphQL schema.

## Step 1: Create an Application

```bash
# Copy the template to a new app
cp -r ~/yeti/applications/application-template ~/yeti/applications/my-app
```

## Step 2: Define Your Schema

Edit `~/yeti/applications/my-app/schema.graphql`:

```graphql
type User @table @export(rest: true, sse: true) {
    id: ID! @primaryKey
    name: String!
    email: String! @indexed
    role: String @indexed
    active: Boolean
    createdAt: Date @createdTime
    updatedAt: Date @updatedTime
}
```

This single file generates:
- `POST /my-app/User` -- Create a user
- `GET /my-app/User` -- List users (with FIQL filtering, pagination, sorting)
- `GET /my-app/User/{id}` -- Get a user by ID
- `PUT /my-app/User/{id}` -- Replace a user
- `PATCH /my-app/User/{id}` -- Partial update
- `DELETE /my-app/User/{id}` -- Delete a user
- `GET /my-app/User?stream=sse` -- Real-time updates

## Step 3: Update the Config

Edit `~/yeti/applications/my-app/config.yaml`:

```yaml
name: "My App"
app_id: "my-app"
version: "1.0.0"
description: "My first Yeti application"
enabled: true
rest: true
graphql: true
sse: true
schemas:
  - schema.graphql
```

## Step 4: Restart the Server

```bash
# From the source directory
cd ~/Developer/yeti
cargo run --release
```

Wait for plugin compilation (~2 minutes on first run). You'll see:

```
[INFO] Registered my-app (1 table, 0 resources)
```

## Step 5: Test Your API

### Create a user

```bash
curl -sk -X POST https://localhost:9996/my-app/User \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Alice",
    "email": "alice@example.com",
    "role": "admin",
    "active": true
  }'
```

The `id`, `createdAt`, and `updatedAt` fields are generated automatically.

### Get the user

```bash
curl -sk https://localhost:9996/my-app/User/USER_ID
```

### List all users

```bash
curl -sk https://localhost:9996/my-app/User
```

### Filter with FIQL

```bash
# Users with role "admin"
curl -sk "https://localhost:9996/my-app/User?role==admin"

# Active users, sorted by name
curl -sk "https://localhost:9996/my-app/User?active==true&sort=name"

# Pagination
curl -sk "https://localhost:9996/my-app/User?limit=10&offset=0"
```

### Update a user

```bash
curl -sk -X PATCH https://localhost:9996/my-app/User/USER_ID \
  -H "Content-Type: application/json" \
  -d '{"role": "viewer"}'
```

### Delete a user

```bash
curl -sk -X DELETE https://localhost:9996/my-app/User/USER_ID
```

### Stream real-time updates

```bash
# In one terminal, listen for updates
curl -sk "https://localhost:9996/my-app/User?stream=sse"

# In another terminal, create a user — it appears in the stream
curl -sk -X POST https://localhost:9996/my-app/User \
  -H "Content-Type: application/json" \
  -d '{"name": "Bob", "email": "bob@example.com"}'
```

## What Just Happened

1. **Schema-driven** -- The GraphQL schema generated REST endpoints, database tables, and real-time subscriptions
2. **Zero boilerplate** -- No route definitions, no ORM setup, no controller classes
3. **Storage** -- RocksDB handled all persistence automatically
4. **Indexes** -- The `@indexed` fields enable fast FIQL filtering
5. **Timestamps** -- `@createdTime` and `@updatedTime` are managed automatically

## Next Steps

- [Your First Application](first-application.md) -- Build a complete app with custom resources and seed data
- [FIQL Queries](../guides/fiql.md) -- Advanced filtering syntax
- [Authentication](../guides/auth-overview.md) -- Add auth to your app
- [Custom Resources](../guides/custom-resources.md) -- Add business logic in Rust
- [Example Applications](../examples/overview.md) -- 11 working apps with source code
