# Example Applications

Yeti ships with a collection of example applications and extensions that demonstrate platform features. Each is self-contained in its own Git repository and can be installed by cloning into `~/yeti/applications/`.

For full documentation on each application, see the README in its repository.

## Core Extensions

| Application | Description |
|-------------|-------------|
| [yeti-auth](https://github.com/yetirocks/yeti-auth) | Authentication and authorization (Basic, JWT, OAuth, RBAC) |
| [yeti-vectors](https://github.com/yetirocks/yeti-vectors) | Automatic text/image embedding with persistent cache |
| [yeti-telemetry](https://github.com/yetirocks/yeti-telemetry) | Log/Span/Metric collection with dashboard and OTLP export |

## Starter

| Application | Description |
|-------------|-------------|
| [Application Template](https://github.com/yetirocks/application-template) | Minimal starter app with a single table and custom resource |

## Data & Queries

| Application | Description |
|-------------|-------------|
| [GraphQL Explorer](https://github.com/yetirocks/graphql-explorer) | Multi-table relationships with Apollo-style GraphQL explorer UI |
| [Example Queries](https://github.com/yetirocks/example-queries) | FIQL filtering, sorting, pagination, field selection, joins |

## AI & Search

| Application | Description |
|-------------|-------------|
| [Vector Search Demo](https://github.com/yetirocks/vector-search-demo) | Automatic text embedding and semantic similarity search |

## Real-Time

| Application | Description |
|-------------|-------------|
| [Real-Time Demo](https://github.com/yetirocks/realtime-demo) | SSE streaming with a React UI for live message updates |

## Caching & Routing

| Application | Description |
|-------------|-------------|
| [Full-Page Cache](https://github.com/yetirocks/full-page-cache) | HTTP caching proxy with origin fallback and auto-expiration |
| [Redirect Manager](https://github.com/yetirocks/redirect-manager) | URL redirects with pattern matching and version-controlled cutover |

## Auth Demo

| Application | Description |
|-------------|-------------|
| [Web Auth Demo](https://github.com/yetirocks/web-auth-demo) | Interactive demo of all auth methods with RBAC visualization |

## Management

| Application | Description |
|-------------|-------------|
| [App Manager](https://github.com/yetirocks/yeti-applications) | Web UI for viewing, editing, and managing all Yeti applications |

## Testing

| Application | Description |
|-------------|-------------|
| [Load Testing](https://github.com/yetirocks/load-test) | Performance testing with multiple table types and relationship chains |

## Installing an Application

```bash
# Clone the repository into your applications folder
cd ~/yeti/applications
git clone https://github.com/yetirocks/<app-name>.git

# Restart Yeti to load the application
# The server auto-detects new directories and loads them
```

## Common Structure

All applications share a common directory structure:

```
~/yeti/applications/{app-id}/
├── README.md            # Full documentation
├── config.yaml          # Application configuration
├── schema.graphql       # Table definitions (if any)
├── resources/           # Custom Rust resources (if any)
│   └── *.rs
├── data/                # Seed data (if any)
│   └── *.json
└── web/                 # Static files (if any)
    └── index.html
```

## Creating a New Application

The fastest way to create a new application is to clone the template:

```bash
cd ~/yeti/applications
git clone https://github.com/yetirocks/application-template.git my-app
cd my-app
# Edit config.yaml, schema.graphql, and resources as needed
```

See the [Application Template README](https://github.com/yetirocks/application-template) for detailed instructions.
