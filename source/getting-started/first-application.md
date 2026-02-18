# Your First Application

This tutorial walks through building a complete Yeti application from scratch. By the end, you will have a working task tracker with two related tables, seed data, a custom resource, and full REST/GraphQL/SSE support.

## What We Are Building

A **task tracker** with:

- A `Task` table for tracking work items with status, priority, and due dates
- A `Tag` table for categorizing tasks
- A custom `Summary` resource that returns task counts by status
- Seed data so the app starts with useful sample records
- FIQL filtering, sorting, pagination, and real-time SSE streaming

## Step 1: Create the Application Directory

Every Yeti application lives in its own directory under the runtime root:

```bash
mkdir ~/yeti/applications/task-tracker
mkdir ~/yeti/applications/task-tracker/resources
mkdir ~/yeti/applications/task-tracker/data
```

## Step 2: Write the Configuration

Create `~/yeti/applications/task-tracker/config.yaml`:

```yaml
# Application metadata
name: "Task Tracker"
app_id: "task-tracker"
version: "1.0.0"
description: "A simple task tracking application"

# Application state
enabled: true

# Interfaces
rest: true
graphql: true
sse: true

# GraphQL schemas to define tables
schemas:
  - schema.graphql

# Custom resource files - compiled plugin libraries
resources:
  - resources/*.rs

# Seed data loaded on startup
dataLoader: data/*.json
```

Key fields:

- **app_id** -- Determines the URL prefix. All endpoints will be under `/task-tracker/`.
- **schemas** -- Points to GraphQL files that define your tables.
- **resources** -- Glob pattern for custom Rust resource files. These compile to dynamic libraries automatically.
- **dataLoader** -- Glob pattern for JSON seed data files loaded on first startup.
- **rest/graphql/sse** -- Enable or disable protocol interfaces per application.

## Step 3: Define the Schema

Create `~/yeti/applications/task-tracker/schema.graphql`:

```graphql
# Task table - work items with status tracking
type Task @table(database: "task-tracker") @export(
    rest: true
    graphql: true
    sse: true
) {
    id: ID! @primaryKey
    title: String!
    description: String
    status: String! @indexed
    priority: String! @indexed
    assignee: String @indexed
    dueDate: String
    tagId: ID @indexed
    tag: Tag @relationship(from: "tagId")
    __createdAt__: String
    __updatedAt__: String
}

# Tag table - categories for organizing tasks
type Tag @table(database: "task-tracker") @export(
    rest: true
    graphql: true
) {
    id: ID! @primaryKey
    name: String! @indexed
    color: String
    tasks: [Task] @relationship(to: "tagId")
}
```

Schema directives explained:

| Directive | Purpose |
|-----------|---------|
| `@table(database: "...")` | Declares a persistent table with its database name |
| `@export(rest: true, ...)` | Exposes the table as REST, GraphQL, and/or SSE endpoints |
| `@primaryKey` | Marks the field used as the record identifier |
| `@indexed` | Creates a secondary index for fast FIQL filtering |
| `@relationship(from: "field")` | Defines a foreign-key join (this record's field points to another table) |
| `@relationship(to: "field")` | Defines a reverse join (other table's field points to this table) |
| `__createdAt__` | Automatically populated with ISO 8601 timestamp on creation |
| `__updatedAt__` | Automatically populated with ISO 8601 timestamp on every update |

From this schema, Yeti generates the following endpoints automatically:

| Method | Endpoint | Description |
|--------|----------|-------------|
| `POST` | `/task-tracker/Task` | Create a task |
| `GET` | `/task-tracker/Task` | List tasks (with filtering, sorting, pagination) |
| `GET` | `/task-tracker/Task/:id` | Get a single task by ID |
| `PUT` | `/task-tracker/Task/:id` | Replace a task |
| `PATCH` | `/task-tracker/Task/:id` | Partial update a task |
| `DELETE` | `/task-tracker/Task/:id` | Delete a task |
| `GET` | `/task-tracker/Task?stream=sse` | Real-time SSE stream of task changes |

The same set of endpoints is generated for `Tag`.

## Step 4: Create a Custom Resource

Custom resources let you add business logic beyond basic CRUD. Create `~/yeti/applications/task-tracker/resources/summary.rs`:

```rust
use yeti_core::prelude::*;
use std::collections::HashMap;

resource!(Summary {
    get(request, ctx) => {
        let tasks = ctx.get_table("Task")?;
        let records: Vec<serde_json::Value> = tasks.scan_all().await?;

        let total = records.len();
        let mut by_status: HashMap<String, usize> = HashMap::new();
        let mut by_priority: HashMap<String, usize> = HashMap::new();
        let mut overdue = 0usize;

        for record in &records {
            if let Some(status) = record["status"].as_str() {
                *by_status.entry(status.to_string()).or_insert(0) += 1;
            }
            if let Some(priority) = record["priority"].as_str() {
                *by_priority.entry(priority.to_string()).or_insert(0) += 1;
            }
            if let Some(due) = record["dueDate"].as_str() {
                if record["status"].as_str() != Some("done") && due < "2026-02-12" {
                    overdue += 1;
                }
            }
        }

        reply().json(json!({
            "total": total,
            "byStatus": by_status,
            "byPriority": by_priority,
            "overdue": overdue
        }))
    }
});
```

This resource is available at `GET /task-tracker/summary` and returns an aggregated view of all tasks.

The `resource!` macro handles struct definition, `Resource` trait implementation, and plugin registration. The two parameters `request` and `ctx` give you access to the HTTP request and the resource context (tables, query parameters, configuration).

## Step 5: Add Seed Data

Create `~/yeti/applications/task-tracker/data/tags.json`:

```json
{
  "database": "task-tracker",
  "table": "Tag",
  "records": [
    { "id": "tag-bug", "name": "Bug", "color": "#e53e3e" },
    { "id": "tag-feature", "name": "Feature", "color": "#3182ce" },
    { "id": "tag-docs", "name": "Documentation", "color": "#38a169" },
    { "id": "tag-infra", "name": "Infrastructure", "color": "#d69e2e" }
  ]
}
```

Create `~/yeti/applications/task-tracker/data/tasks.json`:

```json
{
  "database": "task-tracker",
  "table": "Task",
  "records": [
    {
      "id": "task-001",
      "title": "Fix login redirect loop",
      "description": "Users are stuck in a redirect loop after OAuth callback",
      "status": "in-progress",
      "priority": "high",
      "assignee": "alice",
      "dueDate": "2026-02-15",
      "tagId": "tag-bug"
    },
    {
      "id": "task-002",
      "title": "Add dark mode support",
      "description": "Implement theme switching with system preference detection",
      "status": "todo",
      "priority": "medium",
      "assignee": "bob",
      "dueDate": "2026-03-01",
      "tagId": "tag-feature"
    },
    {
      "id": "task-003",
      "title": "Write API reference docs",
      "description": "Document all REST endpoints with examples",
      "status": "todo",
      "priority": "medium",
      "assignee": "alice",
      "dueDate": "2026-02-20",
      "tagId": "tag-docs"
    },
    {
      "id": "task-004",
      "title": "Upgrade CI pipeline",
      "description": "Migrate from legacy CI to GitHub Actions",
      "status": "done",
      "priority": "low",
      "assignee": "charlie",
      "dueDate": "2026-02-01",
      "tagId": "tag-infra"
    },
    {
      "id": "task-005",
      "title": "Fix memory leak in SSE handler",
      "description": "Connections are not being cleaned up on client disconnect",
      "status": "todo",
      "priority": "high",
      "assignee": "bob",
      "dueDate": "2026-02-10",
      "tagId": "tag-bug"
    }
  ]
}
```

Seed data format requires three fields: `database` (must match the `@table(database: ...)` directive), `table` (the type name), and `records` (array of objects). Records are loaded once on first startup when the table is empty.

## Step 6: Start the Server

From the Yeti source directory, build and run with your application enabled:

```bash
cd ~/Developer/yeti
cargo run --release -- --root-dir ~/yeti --apps task-tracker
```

You can also run multiple applications simultaneously:

```bash
cargo run --release -- --root-dir ~/yeti --apps task-tracker,realtime-demo,example-queries
```

The first startup compiles the custom resource plugin (this takes approximately two minutes). Subsequent restarts use the cached build and start in about ten seconds.

The server starts on HTTPS port 9996 with a self-signed certificate.

## Step 7: Test with curl

All examples use `-sk` because the development server uses self-signed TLS certificates.

### List All Tasks

```bash
curl -sk https://localhost:9996/task-tracker/Task
```

### Get a Single Task

```bash
curl -sk https://localhost:9996/task-tracker/Task/task-001
```

### Create a Task

```bash
curl -sk -X POST https://localhost:9996/task-tracker/Task \
  -H "Content-Type: application/json" \
  -d '{
    "id": "task-006",
    "title": "Add search autocomplete",
    "description": "Implement typeahead suggestions in the search bar",
    "status": "todo",
    "priority": "medium",
    "assignee": "alice",
    "dueDate": "2026-03-15",
    "tagId": "tag-feature"
  }'
```

### Update a Task

```bash
curl -sk -X PUT https://localhost:9996/task-tracker/Task/task-006 \
  -H "Content-Type: application/json" \
  -d '{
    "id": "task-006",
    "title": "Add search autocomplete",
    "status": "in-progress",
    "priority": "high",
    "assignee": "alice",
    "dueDate": "2026-03-15",
    "tagId": "tag-feature"
  }'
```

### Partial Update

```bash
curl -sk -X PATCH https://localhost:9996/task-tracker/Task/task-006 \
  -H "Content-Type: application/json" \
  -d '{"status": "done"}'
```

### Delete a Task

```bash
curl -sk -X DELETE https://localhost:9996/task-tracker/Task/task-006
```

### FIQL Filtering

Query parameters on indexed fields use FIQL syntax for filtering:

```bash
# Tasks with high priority
curl -sk "https://localhost:9996/task-tracker/Task?priority==high"

# Tasks assigned to alice
curl -sk "https://localhost:9996/task-tracker/Task?assignee==alice"

# Tasks that are not done
curl -sk "https://localhost:9996/task-tracker/Task?status!=done"

# Combine filters: high priority AND assigned to bob
curl -sk "https://localhost:9996/task-tracker/Task?priority==high&assignee==bob"
```

### Sorting

```bash
# Sort by priority ascending
curl -sk "https://localhost:9996/task-tracker/Task?sort=priority"

# Sort by due date descending (newest first)
curl -sk "https://localhost:9996/task-tracker/Task?sort=-dueDate"
```

### Pagination

```bash
# First page of 2 results
curl -sk "https://localhost:9996/task-tracker/Task?limit=2&offset=0"

# Second page
curl -sk "https://localhost:9996/task-tracker/Task?limit=2&offset=2"
```

### Field Selection with Relationships

Use the `select` parameter to choose which fields to return. Nested braces traverse relationships:

```bash
# Only return id and title
curl -sk "https://localhost:9996/task-tracker/Task/task-001?select=id,title,status"

# Include the related tag data
curl -sk "https://localhost:9996/task-tracker/Task/task-001?select=id,title,tag%7Bname,color%7D"
```

The `%7B` and `%7D` are URL-encoded `{` and `}`. The above resolves the `tagId` foreign key and returns the related Tag's `name` and `color` inline.

### SSE Streaming

Open a real-time stream that emits events whenever tasks are created, updated, or deleted:

```bash
curl -sk --max-time 30 "https://localhost:9996/task-tracker/Task?stream=sse"
```

In a second terminal, create or update a task and watch the event appear in the SSE stream.

### Custom Resource

```bash
curl -sk https://localhost:9996/task-tracker/summary
```

Returns something like:

```json
{
  "total": 5,
  "byStatus": { "todo": 3, "in-progress": 1, "done": 1 },
  "byPriority": { "high": 2, "medium": 2, "low": 1 },
  "overdue": 1
}
```

## Application Structure Summary

Your completed application looks like this:

```
~/yeti/applications/task-tracker/
  config.yaml          # App configuration
  schema.graphql       # Table definitions
  resources/
    summary.rs         # Custom resource
  data/
    tags.json          # Tag seed data
    tasks.json         # Task seed data
```

Five files. No boilerplate HTTP server code, no routing configuration, no ORM setup. The schema drives everything.

## Next Steps

- **[Authentication](../guides/auth-overview.md)** -- Add Basic, JWT, or OAuth authentication with role-based access control
- **[Real-Time Features](../guides/realtime-overview.md)** -- SSE streaming, WebSocket connections, and PubSub patterns
- **[Building Extensions](../guides/building-extensions.md)** -- Create shared services that multiple applications can use
- **[FIQL Queries](../guides/fiql.md)** -- Advanced filtering with comparison operators and logical expressions
- **[Relationships & Joins](../guides/relationships.md)** -- Deep nested field selection across related tables
- **[Example Applications](../examples/overview.md)** -- Explore 11 working applications with source code
