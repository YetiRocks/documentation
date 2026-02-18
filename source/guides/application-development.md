# Application Development

This section covers everything you need to build applications on the Yeti platform. Yeti is a schema-driven application platform built in Rust that provides automatic REST and GraphQL APIs from GraphQL schema definitions, with support for custom Rust resources, extensions, and real-time features.

## Development Workflow

Building a Yeti application follows a straightforward sequence:

1. **Create the application directory** under `~/yeti/applications/{app-id}/`
2. **Write `config.yaml`** to define your application metadata and features
3. **Define your schema** in a `.graphql` file with Yeti directives
4. **Add custom resources** (optional) for business logic beyond CRUD
5. **Add seed data** (optional) as JSON files for initial table contents
6. **Restart the server** to compile and load your application

Once the server restarts, your application is live. Tables defined in your schema get automatic REST endpoints at `https://localhost:9996/{app-id}/{TableName}` and, if enabled, a GraphQL endpoint at `https://localhost:9996/{app-id}/graphql`.

## Quick Example

```bash
# Create a new application
mkdir -p ~/yeti/applications/my-app

# Minimal config.yaml
cat > ~/yeti/applications/my-app/config.yaml << 'EOF'
name: "My App"
app_id: "my-app"
version: "1.0.0"
enabled: true
rest: true
schemas:
  - schema.graphql
EOF

# Define a table
cat > ~/yeti/applications/my-app/schema.graphql << 'EOF'
type Product @table @export {
    id: ID! @primaryKey
    name: String!
    price: Float
}
EOF

# Restart Yeti, then test
curl -sk https://localhost:9996/my-app/Product
```

## Guides in This Section

- [Application Structure](app-structure.md) -- Directory layout and file conventions
- [Defining Schemas](defining-schemas.md) -- GraphQL schema directives for tables, indexes, and relationships
- [Custom Resources](custom-resources.md) -- Writing Rust resource handlers
- [Seed Data & Data Loading](seed-data.md) -- Populating tables with initial data
- [Static File Serving](static-files.md) -- Hosting frontend applications

## Key Concepts

**Schema-driven**: Your GraphQL schema is the source of truth. Yeti reads it at startup, creates tables in the storage engine, and generates REST and GraphQL endpoints automatically.

**Convention over configuration**: Table types become REST endpoints. The type name is the endpoint path. A `Product` type is served at `/{app-id}/Product`.

**Plugin compilation**: Custom resources are compiled as dynamic libraries. First compilation takes approximately 2 minutes; subsequent cached restarts take around 10 seconds.

**Self-signed TLS**: The development server uses HTTPS on port 9996 with self-signed certificates. Always use `curl -sk` (silent + insecure) when testing locally.

## Related Sections

- [Data Operations](data-operations.md) -- Querying, filtering, and manipulating your data
- [Authentication & Authorization](auth-overview.md) -- Securing your endpoints
- [Real-Time Features](realtime-overview.md) -- SSE, WebSocket, and PubSub
