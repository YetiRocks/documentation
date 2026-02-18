# Error Codes

Yeti uses structured error types that map to standard HTTP status codes. All error responses are returned as JSON.

---

## Error Response Format

All errors follow this JSON structure:

```json
{
  "error": "Human-readable error message"
}
```

For `NotFound` errors, the format includes the resource type and identifier:

```json
{
  "error": "Resource not found: User with id 'user-999'"
}
```

---

## HTTP Status Codes

| Status | Name | YetiError Variant | Description |
|--------|------|-------------------|-------------|
| 400 | Bad Request | `Validation(msg)` | Invalid input, malformed request, missing required fields |
| 400 | Bad Request | `Query(ParseError)` | Invalid FIQL filter syntax |
| 400 | Bad Request | `Query(InvalidSort)` | Invalid sort parameter |
| 400 | Bad Request | `Query(InvalidPagination)` | Invalid limit or offset value |
| 400 | Bad Request | `Query(TooComplex)` | Query exceeds complexity limits |
| 400 | Bad Request | `Schema(...)` | Invalid schema or type definition |
| 400 | Bad Request | `Encoding(...)` | JSON serialization or encoding error |
| 401 | Unauthorized | `Unauthorized(msg)` | Authentication required or credentials invalid |
| 403 | Forbidden | `Forbidden(msg)` | Authenticated but insufficient permissions |
| 404 | Not Found | `NotFound { resource_type, id }` | Requested resource does not exist |
| 409 | Conflict | `Storage(WriteConflict)` | Optimistic locking failure (concurrent write) |
| 500 | Internal Server Error | `Storage(...)` | Database error, I/O failure |
| 500 | Internal Server Error | `Internal(msg)` | Unexpected internal error |
| 503 | Service Unavailable | -- | Server overloaded (backpressure) |

---

## Error Types in Detail

### Validation (400)

Returned when request data fails validation. Common causes:

- Missing required fields in POST/PUT body
- Invalid field types (string where integer expected)
- Empty primary key
- Invalid URL parameters

```json
{
  "error": "Validation error: 'email' is required"
}
```

**Convenience types for resource handlers:**

```rust
// Static message
return Err(BadRequest("Email is required"))?;

// Dynamic message
return Err(BadRequestOwned(format!("Invalid field: {}", name)))?;
```

Both convert to `YetiError::Validation` and produce HTTP 400.

### Unauthorized (401)

Returned when authentication is required but not provided, or credentials are invalid.

```json
{
  "error": "Unauthorized: Invalid credentials"
}
```

```rust
return Err(Unauthorized("Authentication required"))?;
```

### Forbidden (403)

Returned when the user is authenticated but lacks permission for the requested operation.

```json
{
  "error": "Forbidden: Admin access required"
}
```

```rust
return Err(Forbidden("Admin access required"))?;
```

### Not Found (404)

Returned when a requested record or resource does not exist.

```json
{
  "error": "Resource not found: Product with id 'prod-999'"
}
```

```rust
return Err(NotFoundError("Product not found"))?;
```

### Write Conflict (409)

Returned when a concurrent write conflicts with the current operation (optimistic locking).

```json
{
  "error": "Storage error: Write conflict for key: prod-1"
}
```

This error is retryable. Clients should retry the operation after a brief delay.

### Internal Error (500)

Returned for unexpected failures: database errors, I/O issues, or programming errors. The error message is logged server-side; the client receives a generic message.

```json
{
  "error": "Internal error: Storage error: RocksDB error: ..."
}
```

### Service Unavailable (503)

Returned by the backpressure layer when `maxInFlightRequests` is exceeded. This is not a YetiError variant but an HTTP-level response.

```json
{
  "error": "Server overloaded. Please retry later."
}
```

---

## Error Properties

All `YetiError` variants expose:

| Method | Return Type | Description |
|--------|------------|-------------|
| `status_code()` | `u16` | HTTP status code |
| `error_type()` | `&str` | Category string for metrics (e.g., `"validation"`, `"not_found"`) |
| `is_retryable()` | `bool` | Whether the client should retry |

Retryable errors:
- `Storage(Io(...))` -- Transient I/O failures
- `Storage(WriteConflict(...))` -- Concurrent write conflicts
- `Backend(NotAvailable(...))` -- Backend temporarily unavailable

---

## Sub-Error Types

### StorageError

| Variant | Description |
|---------|-------------|
| `KeyNotFound(key)` | Record not found in storage |
| `WriteConflict(key)` | Optimistic locking failure |
| `Corruption(msg)` | Data corruption detected |
| `Io(err)` | File system I/O error |
| `RocksDb(msg)` | RocksDB-specific error |
| `InitializationFailed(msg)` | Database startup failure |

### QueryError

| Variant | Description |
|---------|-------------|
| `ParseError(msg)` | FIQL syntax error |
| `InvalidSelectField(msg)` | Unknown field in select |
| `InvalidSort(msg)` | Invalid sort expression |
| `InvalidPagination(msg)` | Invalid limit/offset |
| `TooComplex { reason }` | Query exceeds limits |

### SchemaError

| Variant | Description |
|---------|-------------|
| `ParseError(msg)` | GraphQL schema syntax error |
| `TableNotFound(name)` | Referenced table not defined |
| `FieldNotFound { table, field }` | Referenced field not defined |
| `InvalidDirective(msg)` | Invalid directive usage |
| `Duplicate(name)` | Duplicate type or field definition |

---

## Client Best Practices

1. Check the HTTP status code first, then parse the error JSON body.
2. Implement exponential backoff for 503 and 409 responses.
3. Do not retry 400, 401, 403, or 404 errors -- fix the request instead.
4. Log the full error response for 500 errors and report to the server operator.

---

## See Also

- [REST API](rest.md) -- REST endpoint reference
- [Operations API](operations.md) -- Administrative API
- [GraphQL API](graphql.md) -- GraphQL error format
