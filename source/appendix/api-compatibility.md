# API Compatibility Matrix

**Harper API implementation status in Yeti**

This document tracks Yeti's compatibility with Harper's APIs. Our goal is 100% feature parity to enable drop-in migration from Harper to Yeti.

**Current Overall Progress**: ~95% Harper parity achieved

**See Also**:
- [Resource API Feature Tracking](../../tasks/RESOURCE_API.md) - Detailed feature-by-feature status
- [Operations API](operations-api.md) - Complete Operations API reference
- [Migration Guide](../getting-started/migration-from-harper.md) - How to migrate from Harper

---

## Resource API (REST)

| Endpoint | Harper | Yeti | Status |
|----------|--------|------|--------|
| `GET /schema/table` | ✅ | ✅ | Complete |
| `GET /schema/table/:id` | ✅ | ✅ | Complete |
| `POST /schema/table` | ✅ | ✅ | Complete |
| `PUT /schema/table/:id` | ✅ | ✅ | Complete |
| `DELETE /schema/table/:id` | ✅ | ✅ | Complete |
| `GET /schema/table?fiql` | ✅ | ✅ | Complete |

---

## FIQL Query Language

| Feature | Harper | Yeti | Status |
|---------|--------|------|--------|
| Equality (`==`) | ✅ | ✅ | Complete |
| Inequality (`!=`) | ✅ | ✅ | Complete |
| Strict equality (`===`, `!==`) | -- | ✅ | Yeti extension |
| Greater than (`>`, `>=`, `=gt=`, `=ge=`) | ✅ | ✅ | Complete |
| Less than (`<`, `<=`, `=lt=`, `=le=`) | ✅ | ✅ | Complete |
| Contains (`=ct=`) | -- | ✅ | Yeti extension |
| Starts with (`=sw=`) | -- | ✅ | Yeti extension |
| Ends with (`=ew=`) | -- | ✅ | Yeti extension |
| Wildcards (`*name*`, `name*`, `*name`) | ✅ | ✅ | Complete |
| Range operators (`=gele=`, `=gtlt=`, etc.) | -- | ✅ | Yeti extension |
| Regex (`=~=`) | ✅ | ✅ | Complete |
| AND (`&`) | ✅ | ✅ | Complete |
| OR (`\|`) | ✅ | ✅ | Complete |
| Grouping (`()`, `[]`) | ✅ | ✅ | Complete |
| NOT (`!`) | ✅ | ✅ | Complete |
| Null handling (`==null`, `!=null`) | ✅ | ✅ | Complete |
| Type prefixes (`number:`, `boolean:`, `date:`) | -- | ✅ | Yeti extension |
| Set membership (`=in=`, `=out=`) | ✅ | ✅ | Complete |
| Full-text search (`=ft=`) | ✅ | ✅ | Complete |

---

## Operations API

**See [Operations API Reference](operations-api.md) for complete documentation.**

### System Operations

| Operation | Harper | Yeti | Status |
|-----------|--------|------|--------|
| `system_information` | ✅ | ✅ | Complete |
| `health_check` | ✅ | ✅ | Complete |
| `get_configuration` | ✅ | ✅ | Complete |

### Component Operations (Harper terminology)

| Operation | Harper | Yeti | Status |
|-----------|--------|------|--------|
| `get_components` | ✅ | ✅ | Complete |
| `component_status` | ✅ | ✅ | Complete |
| `package_component` | ✅ | ✅ | Complete |
| `deploy_component` | ✅ | ✅ | Complete |
| `add_component` | ✅ | 📋 | Planned |
| `drop_component` | ✅ | 📋 | Planned |
| `get_component_file` | ✅ | 📋 | Planned |
| `set_component_file` | ✅ | 📋 | Planned |

### Application Operations (Yeti aliases)

| Operation | Description | Status |
|-----------|-------------|--------|
| `list_apps` | Alias for `get_components` | ✅ Complete |
| `app_status` | Alias for `component_status` | ✅ Complete |

### Describe Operations

| Operation | Harper | Yeti | Status |
|-----------|--------|------|--------|
| `describe_all` | ✅ | ✅ | Complete |
| `describe_database` | ✅ | ✅ | Complete |
| `describe_table` | ✅ | ✅ | Complete |

### Schema Operations (Legacy)

| Operation | Harper | Yeti | Status |
|-----------|--------|------|--------|
| `create_schema` | ✅ | 📋 | Planned (via schema.graphql) |
| `drop_schema` | ✅ | 📋 | Planned |
| `create_table` | ✅ | 📋 | Planned (via schema.graphql) |
| `drop_table` | ✅ | 📋 | Planned |
| `create_attribute` | ✅ | 📋 | Planned |
| `drop_attribute` | ✅ | 📋 | Planned |

---

## Secondary Indexes

| Feature | Harper | Yeti | Status |
|---------|--------|------|--------|
| Hash index (equality) | ✅ | ✅ | Complete |
| Range index (comparisons) | ✅ | ✅ | Complete |
| Full-text search | ✅ | ✅ | Complete |
| Composite indexes | ✅ | ✅ | Complete |
| HNSW vector index | ✅ | ✅ | Complete |
| Auto-embedding (yeti-vectors) | -- | ✅ | Yeti extension |

---

## Custom Resources

| Feature | Harper | Yeti | Status |
|---------|--------|------|--------|
| Resource class trait | ✅ | ✅ | Complete |
| GET handler | ✅ | ✅ | Complete |
| POST handler | ✅ | ✅ | Complete |
| PUT handler | ✅ | ✅ | Complete |
| DELETE handler | ✅ | ✅ | Complete |
| PATCH handler | ✅ | ✅ | Complete |
| Dynamic loading | ✅ | ❌ | By design (compile-time) |

---

## Static Files

| Feature | Harper | Yeti | Status |
|---------|--------|------|--------|
| File serving | ✅ | ✅ | Complete |
| Directory routing | ✅ | ✅ | Complete |
| MIME types | ✅ | ✅ | Complete |

---

## Legend

- ✅ **Complete** - Fully implemented and tested
- 🚧 **In Progress** - Partially implemented
- 📋 **Planned** - Scheduled for implementation
- ❌ **Not Planned** - Intentionally different design

---

## Try It Examples

### Resource API (Complete Features)

**Create a record**:
```bash
curl -X POST http://localhost:9996/User \
  -H "Content-Type: application/json" \
  -d '{"id":"user-1","name":"Alice","email":"alice@example.com"}'
```

**Get a record**:
```bash
curl http://localhost:9996/User/user-1
```

**List records with FIQL filter**:
```bash
# Equality filter
curl "http://localhost:9996/User?filter=name==Alice"

# AND filter
curl "http://localhost:9996/User?filter=name==Alice%26email==alice@example.com"
```

### Custom Resources (Complete)

See [Custom Resources Guide](../developers/custom-resources.md) for implementing custom business logic in Rust.

---

## Migration Impact

| Feature Status | Migration Impact | Action Required |
|----------------|------------------|-----------------|
| ✅ Complete | Drop-in replacement | No changes needed |
| 🚧 In Progress | Partial compatibility | May need workarounds |
| 📋 Planned | Not yet available | Wait for implementation |
| ❌ Not Planned | Different by design | Adapt to Yeti's approach |

**See**: [Migration Guide](../getting-started/migration-from-harper.md) for detailed migration strategies.

---

## Contributing

Help us achieve 100% parity! See:
- **[Task Tracking](../../tasks/)** - Pick a feature to implement
- **[Contributing Guide](../contributing/)** - Development setup
- **[ROADMAP](../../tasks/ROADMAP.md)** - Implementation priorities

---

**Overall Progress**: ~90% Harper parity achieved
**Last Updated**: 2026-02-12
