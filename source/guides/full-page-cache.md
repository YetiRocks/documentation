# Full-Page Caching

The full-page cache pattern stores entire HTTP responses keyed by URL path. On cache hit, the stored content is returned immediately. On cache miss, the content is fetched from an origin server, stored in a table, and returned to the client.

This pattern is implemented by the `full-page-cache` example application.

---

## How It Works

1. A request arrives for a path like `/products/widget`
2. The cache resource looks up the path in the `PageCache` table
3. **Cache HIT**: Return stored content with `x-cache: HIT` header
4. **Cache MISS**: Fetch from the configured origin URL, store the response, and return it with `x-cache: MISS` header

---

## Schema

The cache table uses the URL path as its primary key and has a 1-hour TTL:

```graphql
type PageCache @table(database: "full-page-caching", expiration: 3600) {
  path: String! @primaryKey
  pageContents: String
  contentType: String
  httpStatus: Int
  cachedAt: Int
}
```

The `expiration: 3600` directive means cached entries are automatically evicted after one hour through RocksDB TTL compaction.

---

## Configuration

The application `config.yaml` specifies the origin URL:

```yaml
name: "Full Page Cache"
app_id: "full-page-cache"
version: "1.0.0"
enabled: true
rest: true
graphql: true

schemas:
  - schema.graphql

resources:
  - resources/*.rs

origin:
  url: "https://www.example.com/"
```

---

## Resource Implementation

The cache resource is a default handler that catches all paths. This is the core pattern from `page_cache.rs`:

```rust
pub struct PageCache;

impl Resource for PageCache {
    fn name(&self) -> &str { "PageCache" }
    fn is_default(&self) -> bool { true }

    fn get(&self, _request: Request<Vec<u8>>, ctx: ResourceParams) -> ResourceFuture {
        async_handler!({
            let path = ctx.id().unwrap_or("/");
            let cache = ctx.get_table("PageCache")?;

            // Check cache first
            if let Some(cached) = cache.get_by_id(&path).await? {
                ctx.response_headers().append("x-cache", "HIT");
                return ok_html(cached.as_str().unwrap_or_default());
            }

            // Fetch from origin
            let origin = ctx.config().get_str("origin.url", "https://example.com/");
            let url = format!("{}{}", origin.trim_end_matches('/'), &path);

            let (tx, rx) = std::sync::mpsc::channel();
            std::thread::spawn(move || {
                let result = reqwest::blocking::get(&url);
                let _ = tx.send(result);
            });
            let response = rx.recv()??;
            let html = response.text()?;

            // Store in cache
            cache.put(&path, json!(html)).await?;

            ctx.response_headers().append("x-cache", "MISS");
            ok_html(&html)
        })
    }
}

register_resource!(PageCache);
```

---

## Why reqwest::blocking?

Dynamic library plugins have their own separate Tokio runtime due to TLS (thread-local storage) isolation. The async `reqwest::Client` would attempt to use the dylib's tokio runtime, which does not have a running event loop. Using `reqwest::blocking::Client` on a dedicated `std::thread` avoids this issue entirely.

---

## Testing

```bash
# First request - cache miss, fetches from origin
curl -sk https://localhost:9996/full-page-cache/products
# Response headers: x-cache: MISS

# Second request - cache hit, served from table
curl -sk https://localhost:9996/full-page-cache/products
# Response headers: x-cache: HIT

# After 1 hour, the entry expires and the next request is a cache miss again
```

---

## Customization

- Change the TTL by modifying `expiration` in the schema directive
- Store additional metadata (content type, HTTP status) for richer cache behavior
- Add cache invalidation by exposing a DELETE endpoint on the PageCache table
- Use `@export(rest: true)` on the PageCache type to expose cache management endpoints

---

## See Also

- [Caching & Performance](caching.md) -- Caching overview
- [Table Expiration](table-expiration.md) -- TTL configuration details
- [Custom Resources](custom-resources.md) -- Building custom resource handlers
