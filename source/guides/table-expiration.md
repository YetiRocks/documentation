# Table Expiration

Yeti supports automatic record expiration (TTL) on tables using RocksDB's built-in TTL compaction. Records are automatically removed after a configured time period, making this ideal for session storage, caches, and temporary data.

---

## Configuring Expiration

Set the `expiration` parameter on the `@table` directive in your schema. The value is in seconds:

```graphql
# Records expire after 1 hour (3600 seconds)
type Session @table(expiration: 3600) @export {
    id: ID! @primaryKey
    userId: String!
    token: String!
    createdAt: Int
}
```

```graphql
# Records expire after 24 hours
type CacheEntry @table(expiration: 86400) @export {
    key: String! @primaryKey
    value: String!
}
```

```graphql
# Records expire after 7 days
type PasswordReset @table(expiration: 604800) @export {
    id: ID! @primaryKey
    email: String! @indexed
    token: String!
}
```

---

## How It Works

When a table has an `expiration` value:

1. RocksDB opens the column family with TTL enabled
2. Each record's creation time is tracked internally
3. During compaction, RocksDB checks if records have exceeded the TTL
4. Expired records are removed during the next compaction cycle

Expiration is **not instantaneous**. Records may persist slightly beyond the configured TTL until the next compaction run. Do not rely on exact-second expiration for security-critical logic.

---

## Per-Record Expiration

For cases where different records need different lifetimes, use the `@expiresAt` field directive:

```graphql
type OAuthSession @table(database: "yeti-auth") @export {
    sessionId: String @primaryKey
    provider: String
    accessToken: String
    refreshToken: String
    tokenExpiresAt: Int     # When the OAuth access token expires
    createdAt: Int
    expiresAt: Int @expiresAt  # When this record should be deleted
}
```

The `@expiresAt` field must be a Unix timestamp (integer). Records are removed when the current time exceeds this value. This overrides the table-level expiration for individual records.

---

## Use Cases

### Session Storage

Short-lived sessions with automatic cleanup:

```graphql
type WebSession @table(expiration: 1800) @export {
    id: ID! @primaryKey
    userId: String! @indexed
    data: String
    lastAccess: Int
}
```

### Cache Tables

Temporary cached data that should not persist indefinitely:

```graphql
type PageCache @table(expiration: 3600) {
    path: String! @primaryKey
    content: String
    contentType: String
}
```

### Rate Limiting Counters

Short-lived counters for rate limiting:

```graphql
type RateLimit @table(expiration: 60) {
    key: String! @primaryKey
    count: Int!
}
```

---

## Tables Without Expiration

If no `expiration` is set, records persist indefinitely. This is the default and appropriate for primary data tables:

```graphql
type User @table @export {
    id: ID! @primaryKey
    name: String!
    email: String! @indexed
}
```

---

## Important Notes

- Expiration is based on record creation time, not last access time. Updating a record resets the TTL.
- Expired records may still appear in queries briefly until compaction runs.
- The `@expiresAt` per-record TTL takes precedence over table-level `expiration` when both are set.
- Expiration does not affect indexes -- expired records are removed from both primary storage and indexes during compaction.

---

## See Also

- [Caching & Performance](caching.md) -- Performance overview
- [Schema Directives](../reference/schema-directives.md) -- Complete directive reference
- [Full-Page Caching](full-page-cache.md) -- TTL in practice
