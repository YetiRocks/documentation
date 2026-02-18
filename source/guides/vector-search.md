# Vector Search

Yeti includes a built-in HNSW (Hierarchical Navigable Small World) vector index for approximate nearest-neighbor search. This enables semantic search, recommendation engines, and similarity matching directly in your schema without external vector databases.

## Defining a Vector Field

Add an `@indexed(type: "HNSW")` directive to a float array field in your schema:

```graphql
type Document @table(database: "my-app") @export(rest: true) {
    id: ID! @primaryKey
    title: String!
    content: String
    embedding: [Float!]! @indexed(type: "HNSW")
}
```

### HNSW Configuration Options

The `@indexed` directive accepts optional tuning parameters:

```graphql
type Document @table(database: "my-app") @export(rest: true) {
    id: ID! @primaryKey
    title: String!
    embedding: [Float!]! @indexed(
        type: "HNSW"
        distance: "cosine"
        optimizeRouting: 0.6
    )
}
```

| Parameter | Default | Description |
|-----------|---------|-------------|
| `type` | -- | Must be `"HNSW"` for vector indexing |
| `distance` | `"cosine"` | Distance function: `"cosine"` or `"euclidean"` |
| `optimizeRouting` | `0.0` | Routing optimization factor (0.0 to 1.0) |

## Inserting Vectors

Insert records with vector data through the standard REST API:

```bash
curl -sk -X POST https://localhost:9996/my-app/Document \
  -H "Content-Type: application/json" \
  -d '{
    "id": "doc-1",
    "title": "Introduction to Machine Learning",
    "content": "Machine learning is a subset of artificial intelligence...",
    "embedding": [0.12, -0.45, 0.78, 0.33, -0.21, 0.56, 0.91, -0.14]
  }'

curl -sk -X POST https://localhost:9996/my-app/Document \
  -H "Content-Type: application/json" \
  -d '{
    "id": "doc-2",
    "title": "Deep Learning Fundamentals",
    "content": "Deep learning uses neural networks with many layers...",
    "embedding": [0.15, -0.42, 0.81, 0.29, -0.18, 0.61, 0.88, -0.11]
  }'

curl -sk -X POST https://localhost:9996/my-app/Document \
  -H "Content-Type: application/json" \
  -d '{
    "id": "doc-3",
    "title": "Cooking Italian Pasta",
    "content": "The secret to great pasta is fresh ingredients...",
    "embedding": [-0.72, 0.31, -0.15, 0.88, 0.44, -0.63, 0.02, 0.55]
  }'
```

## Searching for Nearest Neighbors

### JSON Format

Pass the full search configuration as a single `vector_search` parameter:

```bash
curl -sk "https://localhost:9996/my-app/Document?vector_search=\
{\"attribute\":\"embedding\",\"target\":[0.13,-0.44,0.80,0.31,-0.19,0.58,0.90,-0.12],\"limit\":5}"
```

### Individual Parameters

Use separate query parameters for each search option:

```bash
curl -sk "https://localhost:9996/my-app/Document?\
vector_attr=embedding&\
vector_target=[0.13,-0.44,0.80,0.31,-0.19,0.58,0.90,-0.12]&\
limit=5&\
max_distance=0.3"
```

### Search Parameters

| Parameter | Required | Description |
|-----------|----------|-------------|
| `attribute` / `vector_attr` | Yes | Name of the HNSW-indexed field |
| `target` / `vector_target` | Yes | Query vector as JSON array of floats |
| `limit` | No | Max results to return (default: 10) |
| `max_distance` | No | Maximum distance threshold (filters distant results) |

## Distance Metrics

**Cosine distance** (default) measures the angle between vectors, ranging from 0 (identical) to 2 (opposite). Best for text embeddings and normalized vectors where magnitude does not matter.

**Euclidean distance** measures the straight-line distance in vector space. Best for spatial data and cases where vector magnitude is meaningful.

## Use Cases

- **Semantic search**: Embed documents with an LLM, then find similar content by embedding the search query and running nearest-neighbor search.
- **Recommendations**: Embed user preferences and item features, then find items closest to a user's preference vector.
- **Image similarity**: Store image feature vectors and find visually similar images.
- **Anomaly detection**: Identify data points that are far from any cluster center.

## Combining with FIQL Filters

Vector search can be combined with standard FIQL filters for hybrid queries. The FIQL filter runs on the vector search results:

```bash
curl -sk "https://localhost:9996/my-app/Document?\
vector_attr=embedding&\
vector_target=[0.13,-0.44,0.80,0.31,-0.19,0.58,0.90,-0.12]&\
limit=10&\
filter=title==*Learning*"
```

## Automatic Embedding with yeti-vectors

The **yeti-vectors** extension removes the need to pre-compute embeddings. It uses the `fastembed` crate for local ONNX-based inference, automatically generating embeddings on writes and converting text queries to vectors on reads.

### Setup

1. Enable the extension in your app's `config.yaml`:

```yaml
extensions:
  - yeti-vectors:
      fields:
        - source: content
          target: embedding
          model: "BAAI/bge-small-en-v1.5"
          field_type: text
```

2. Your schema still needs the HNSW index on the target field:

```graphql
type Document @table(database: "my-app") @export(rest: true) {
    id: ID! @primaryKey
    title: String!
    content: String
    embedding: [Float!]! @indexed(type: "HNSW")
}
```

3. Insert records **without** manually providing embeddings -- the extension generates them:

```bash
curl -sk -X POST https://localhost:9996/my-app/Document \
  -H "Content-Type: application/json" \
  -d '{
    "id": "doc-1",
    "title": "Introduction to Machine Learning",
    "content": "Machine learning is a subset of artificial intelligence..."
  }'
```

The `embedding` field is automatically populated from the `content` field.

### Text-Based Search

With yeti-vectors, you can search with natural language instead of raw vectors:

```bash
# JSON format
curl -sk "https://localhost:9996/my-app/Document?vector_search_text=\
{\"attribute\":\"embedding\",\"text\":\"how does deep learning work\",\"model\":\"BAAI/bge-small-en-v1.5\"}"

# Individual parameters
curl -sk "https://localhost:9996/my-app/Document?\
vector_attr=embedding&\
vector_text=how+does+deep+learning+work&\
vector_model=BAAI/bge-small-en-v1.5&\
limit=5"
```

| Parameter | Required | Description |
|-----------|----------|-------------|
| `vector_text` | Yes | Natural language query text |
| `vector_model` | Yes | Embedding model (must match the field mapping model) |
| `vector_attr` | No | Target attribute (default: `embedding`) |
| `limit` | No | Max results (default: 10) |
| `max_distance` | No | Maximum distance threshold |

### Image Embedding

yeti-vectors also supports image embeddings via CLIP models. Use `field_type: image` for base64-encoded `Bytes` fields:

```yaml
extensions:
  - yeti-vectors:
      fields:
        - source: description
          target: textEmbedding
          model: "BAAI/bge-small-en-v1.5"
          field_type: text
        - source: thumbnail
          target: imageEmbedding
          model: "clip-ViT-B-32"
          field_type: image
```

```graphql
type Product @table(database: "my-app") @export(rest: true) {
    id: ID! @primaryKey
    description: String
    textEmbedding: [Float!]! @indexed(type: "HNSW")
    thumbnail: Bytes
    imageEmbedding: [Float!]! @indexed(type: "HNSW")
}
```

### Backfill on Schema Change

When you add the yeti-vectors extension to an app with existing records, the extension automatically backfills embeddings on the next restart. Records with missing or null target vector fields are detected and processed in the background.

- **Idempotent**: Only records without embeddings are processed. Restarting again skips already-embedded records.
- **Non-blocking**: The server starts immediately; backfill runs concurrently.
- **Progress logged**: Watch for `Backfilling` messages in the server log.

### Supported Models

| Model | Type | Dimensions | Size |
|-------|------|------------|------|
| `BAAI/bge-small-en-v1.5` | Text | 384 | ~130 MB |
| `BAAI/bge-base-en-v1.5` | Text | 768 | ~440 MB |
| `BAAI/bge-large-en-v1.5` | Text | 1024 | ~1.3 GB |
| `all-MiniLM-L6-v2` | Text | 384 | ~80 MB |
| `clip-ViT-B-32` | Image | 512 | ~300 MB |

Models are downloaded automatically on first use and cached locally.

## Embedding Cache

By default, yeti-vectors caches the result of every text-to-vector embedding. When the same text and model are searched again, the cached vector is returned instantly instead of re-running the ONNX model (~50-200ms per embedding).

### How It Works

The cache is stored in an `EmbeddingCache` table inside the yeti-vectors extension's own RocksDB database. Cache keys are a SHA-256 hash of the model name and query text, so entries are shared across all applications that use the same model.

On a text-based vector search:
1. Yeti computes `sha256(model + "\0" + text)` as the cache key
2. If a matching entry exists in `EmbeddingCache`, the stored vector is used directly
3. On a cache miss, the embedding is computed via `fastembed`, the search executes, and the result is stored in `EmbeddingCache` for future queries

Since embeddings for a given (text, model) pair are deterministic, cached entries never go stale. There is no TTL.

### Per-App Configuration

Caching is enabled by default. To disable it for a specific application, set `cache: false` in the extension config:

```yaml
extensions:
  - yeti-vectors:
      cache: false
      fields:
        - source: content
          target: embedding
          model: "BAAI/bge-small-en-v1.5"
          field_type: text
```

### Managing the Cache

Cached embeddings are accessible through the standard REST API on the yeti-vectors application:

```bash
# List all cached embeddings
curl -sk https://localhost:9996/yeti-vectors/EmbeddingCache

# View a specific cache entry
curl -sk https://localhost:9996/yeti-vectors/EmbeddingCache/{id}

# Delete a specific cache entry
curl -sk -X DELETE https://localhost:9996/yeti-vectors/EmbeddingCache/{id}
```

Each cache entry contains:

| Field | Description |
|-------|-------------|
| `id` | SHA-256 hash key (64-character hex string) |
| `model` | The embedding model name |
| `embedding` | The cached vector (array of floats) |
| `createdAt` | Unix timestamp when the entry was created |

## See Also

- [FIQL Queries](fiql.md) -- Text-based filtering
- [Schema Directives](../reference/schema-directives.md) -- Full directive reference
- [Vector Search Demo](../examples/vector-search-demo.md) -- Interactive demo application
