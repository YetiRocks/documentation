# Server-Sent Events

Server-Sent Events (SSE) provide a one-way streaming connection from the Yeti server to clients. When data changes in a table, connected SSE clients receive the update in real time. SSE is built on standard HTTP and works through proxies, load balancers, and firewalls without special configuration.

## Schema Configuration

Enable SSE on a table with the `@export(sse: true)` directive:

```graphql
type Message @table(database: "realtime-demo") @export(
    name: "message"
    rest: true
    sse: true
) {
    id: ID! @primaryKey
    title: String!
    content: String!
    __createdAt__: String
}
```

## Connecting to an SSE Stream

Add `?stream=sse` to any table endpoint to open an SSE connection:

```bash
curl -sk "https://localhost:9996/realtime-demo/message?stream=sse"
```

The server responds with the SSE content type and begins streaming events:

```
HTTP/1.1 200 OK
Content-Type: text/event-stream
Cache-Control: no-cache
Connection: keep-alive
X-Accel-Buffering: no

event: message
id: 0
data: {"type":"connected","status":"ok"}

```

## Event Format

Each SSE event follows the standard format:

```
event: <type>
id: <optional-id>
data: <json-payload>

```

### Event Types

| Event Type | Description | Triggered By |
|------------|-------------|--------------|
| `message` | Generic message (includes the initial "connected" event) | Connection established, retained data |
| `update` | A record was created or updated | POST, PUT operations |
| `delete` | A record was deleted | DELETE operations |
| `publish` | A custom application-level message | PubSub publish |
| `ping` | Heartbeat keepalive | Internal timer |
| `error` | An error occurred | Stream errors |

### Example Event Sequence

```
event: message
id: 0
data: {"type":"connected","status":"ok"}

event: update
id: msg-1
data: {"id":"msg-1","title":"Hello","content":"World"}

event: update
id: msg-2
data: {"id":"msg-2","title":"Second","content":"Message"}

event: delete
id: msg-1
data: {"id":"msg-1"}

```

## JavaScript Client

```javascript
const source = new EventSource(
  'https://localhost:9996/realtime-demo/message?stream=sse'
);

// Listen for connection confirmation
source.onmessage = (event) => {
  const data = JSON.parse(event.data);
  console.log('Message:', data);
};

// Listen for specific event types
source.addEventListener('update', (event) => {
  const record = JSON.parse(event.data);
  console.log('Record updated:', record.id, record);
});

source.addEventListener('delete', (event) => {
  const data = JSON.parse(event.data);
  console.log('Record deleted:', data.id);
});

source.addEventListener('error', (event) => {
  console.error('SSE error:', event);
});

// Close when done
source.close();
```

## Record-Level Subscriptions

Subscribe to changes for a specific record by including the record ID:

```bash
curl -sk "https://localhost:9996/realtime-demo/message/msg-1?stream=sse"
```

This stream will only receive events for the record with ID `msg-1`.

## Testing with curl

Open two terminals to test SSE:

**Terminal 1** -- Listen for events:
```bash
curl -sk --max-time 60 \
  "https://localhost:9996/realtime-demo/message?stream=sse"
```

**Terminal 2** -- Create and modify data:
```bash
# Create a record (triggers "update" event)
curl -sk -X POST https://localhost:9996/realtime-demo/message \
  -H "Content-Type: application/json" \
  -d '{"id":"test-1","title":"Live","content":"SSE is working"}'

# Delete the record (triggers "delete" event)
curl -sk -X DELETE https://localhost:9996/realtime-demo/message/test-1
```

## GraphQL Subscriptions

SSE also powers GraphQL subscriptions. Send a subscription query with the `Accept: text/event-stream` header:

```bash
curl -sk -H "Accept: text/event-stream" \
  -H "Content-Type: application/json" \
  -d '{"query": "subscription { message { id title content } }"}' \
  https://localhost:9996/realtime-demo/graphql
```

## Response Headers

SSE responses include headers for proper streaming behavior:

| Header | Value | Purpose |
|--------|-------|---------|
| `Content-Type` | `text/event-stream` | SSE protocol identifier |
| `Cache-Control` | `no-cache` | Prevents caching of the stream |
| `Connection` | `keep-alive` | Maintains the persistent connection |
| `X-Accel-Buffering` | `no` | Disables nginx buffering |

## See Also

- [Real-Time Overview](realtime-overview.md) -- All real-time features
- [WebSocket](websocket.md) -- Bidirectional alternative
- [PubSub](pubsub.md) -- Underlying messaging system
