# Vector Search

Yeti includes built-in HNSW vector indexing for approximate nearest-neighbor search.

## Schema

```graphql
type Document @table(database: "my-app") @export(rest: true) {
    id: ID! @primaryKey
    title: String!
    content: String
    embedding: [Float!]! @indexed(type: "HNSW")
}
```

### HNSW Options

```graphql
embedding: [Float!]! @indexed(type: "HNSW", distance: "cosine", optimizeRouting: 0.6)
```

| Parameter | Default | Description |
|-----------|---------|-------------|
| `distance` | `"cosine"` | `"cosine"` or `"euclidean"` |
| `optimizeRouting` | `0.0` | Routing optimization (0.0-1.0) |

## Inserting Vectors

```bash
curl -sk -X POST https://localhost:9996/my-app/Document \
  -H "Content-Type: application/json" \
  -d '{"id":"doc-1","title":"Intro to ML","content":"Machine learning is...","embedding":[0.12,-0.45,0.78,0.33]}'
```

## Searching

```bash
# JSON format
curl -sk "https://localhost:9996/my-app/Document?vector_search=\
{\"attribute\":\"embedding\",\"target\":[0.13,-0.44,0.80,0.31],\"limit\":5}"

# Individual parameters
curl -sk "https://localhost:9996/my-app/Document?\
vector_attr=embedding&vector_target=[0.13,-0.44,0.80,0.31]&limit=5&max_distance=0.3"
```

| Parameter | Required | Description |
|-----------|----------|-------------|
| `attribute` / `vector_attr` | Yes | HNSW-indexed field name |
| `target` / `vector_target` | Yes | Query vector (JSON array) |
| `limit` | No | Max results (default: 10) |
| `max_distance` | No | Distance threshold |

Combine with FIQL: `&filter=title==*Learning*`

## Automatic Embedding with yeti-vectors

The **yeti-vectors** extension generates embeddings automatically using local ONNX inference.

### Setup

```yaml
extensions:
  - yeti-vectors:
      fields:
        - source: content
          target: embedding
          model: "BAAI/bge-small-en-v1.5"
          field_type: text
```

Insert records **without** embeddings - the extension generates them from the source field:

```bash
curl -sk -X POST https://localhost:9996/my-app/Document \
  -H "Content-Type: application/json" \
  -d '{"id":"doc-1","title":"Intro to ML","content":"Machine learning is..."}'
```

### Text-Based Search

Search with natural language instead of raw vectors:

```bash
curl -sk "https://localhost:9996/my-app/Document?\
vector_attr=embedding&vector_text=how+does+deep+learning+work&\
vector_model=BAAI/bge-small-en-v1.5&limit=5"
```

### Image Embedding

```yaml
extensions:
  - yeti-vectors:
      fields:
        - source: thumbnail
          target: imageEmbedding
          model: "clip-ViT-B-32"
          field_type: image
```

### Backfill

Adding yeti-vectors to an existing app auto-backfills embeddings on next restart. Non-blocking, idempotent, progress logged.

### Supported Models

| Model | Type | Dimensions | Size |
|-------|------|------------|------|
| `BAAI/bge-small-en-v1.5` | Text | 384 | ~130 MB |
| `BAAI/bge-base-en-v1.5` | Text | 768 | ~440 MB |
| `BAAI/bge-large-en-v1.5` | Text | 1024 | ~1.3 GB |
| `all-MiniLM-L6-v2` | Text | 384 | ~80 MB |
| `clip-ViT-B-32` | Image | 512 | ~300 MB |

Models download automatically on first use.

## Embedding Cache

yeti-vectors caches embeddings keyed by `sha256(model + text)`. Deterministic, no TTL.

Disable per-app:

```yaml
extensions:
  - yeti-vectors:
      cache: false
```

Manage via REST:

```bash
curl -sk https://localhost:9996/yeti-vectors/EmbeddingCache
curl -sk -X DELETE https://localhost:9996/yeti-vectors/EmbeddingCache/{id}
```

## See Also

- [FIQL Queries](fiql.md) - Text-based filtering
- [Schema Directives](../reference/schema-directives.md) - Full directive reference
