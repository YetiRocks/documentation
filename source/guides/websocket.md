# WebSocket

WebSocket connections provide bidirectional, real-time communication between clients and Yeti. Unlike SSE (server-to-client only), WebSocket allows both the client and server to send messages at any time, making it suitable for interactive applications like chat, collaborative editors, and live dashboards with user input.

## Schema Configuration

Enable WebSocket on a table with the `@export(ws: true)` directive:

```graphql
type Message @table(database: "realtime-demo") @export(
    rest: true
    ws: true
) {
    id: ID! @primaryKey
    title: String!
    content: String!
}
```

Also set `ws: true` in your `config.yaml`:

```yaml
name: "My App"
app_id: "my-app"
rest: true
ws: true
schemas:
  - schema.graphql
```

## Connecting

WebSocket connections upgrade from HTTP/1.1. The server URL uses the `wss://` scheme (TLS) on port 9996:

```
wss://localhost:9996/my-app/Message
```

## JavaScript Client

```javascript
const ws = new WebSocket('wss://localhost:9996/my-app/Message');

ws.onopen = () => {
  console.log('Connected');

  // Send a message to the server
  ws.send(JSON.stringify({
    action: 'subscribe',
    topic: 'Message'
  }));
};

ws.onmessage = (event) => {
  const data = JSON.parse(event.data);
  console.log('Received:', data);
};

ws.onerror = (error) => {
  console.error('WebSocket error:', error);
};

ws.onclose = (event) => {
  console.log('Disconnected:', event.code, event.reason);
};
```

## Bidirectional Communication

WebSocket connections support both directions:

**Server to client**: When a record in the subscribed table is created, updated, or deleted, the server pushes a JSON message to all connected WebSocket clients.

**Client to server**: The client can send JSON messages which are forwarded to the resource's incoming message handler. This enables interactive patterns where client messages influence the data stream.

```javascript
// Receive server-pushed updates
ws.onmessage = (event) => {
  const update = JSON.parse(event.data);
  if (update.message_type === 'Update') {
    updateUI(update.data);
  } else if (update.message_type === 'Delete') {
    removeFromUI(update.id);
  }
};

// Send data to the server
ws.send(JSON.stringify({
  id: 'msg-new',
  title: 'From WebSocket',
  content: 'Sent via WS'
}));
```

## Heartbeat

The WebSocket handler maintains connection health through a ping/pong mechanism:

| Setting | Value |
|---------|-------|
| Heartbeat interval | 5 seconds |
| Client timeout | 10 seconds |

The server sends a `Ping` frame every 5 seconds. If no `Pong` response is received within 10 seconds, the connection is considered dead and closed. Clients must respond to pings (most WebSocket libraries handle this automatically).

## Message Format

All messages are JSON-encoded. Server-pushed messages have this structure:

```json
{
  "message_type": "Update",
  "data": {
    "id": "msg-1",
    "title": "Hello",
    "content": "World"
  },
  "id": "msg-1",
  "timestamp": "2025-01-15T10:30:00Z"
}
```

Message types: `Update`, `Delete`, `Publish`, `Retained`.

## Record-Level Subscriptions

Connect to a specific record by including the ID in the URL:

```javascript
// Subscribe to changes for a specific record
const ws = new WebSocket('wss://localhost:9996/my-app/Message/msg-1');
```

## Reconnection

WebSocket connections can drop due to network issues. Implement reconnection logic in your client:

```javascript
function connect() {
  const ws = new WebSocket('wss://localhost:9996/my-app/Message');

  ws.onclose = () => {
    console.log('Disconnected, reconnecting in 3s...');
    setTimeout(connect, 3000);
  };

  ws.onmessage = (event) => {
    const data = JSON.parse(event.data);
    handleUpdate(data);
  };
}

connect();
```

## SSE vs WebSocket

| Feature | SSE | WebSocket |
|---------|-----|-----------|
| Direction | Server to client | Bidirectional |
| Protocol | HTTP/1.1, HTTP/2 | WS/WSS |
| Auto-reconnect | Built-in (EventSource) | Manual |
| Binary data | No (text only) | Yes |
| Proxy support | Excellent | Varies |
| Use case | Dashboards, feeds | Chat, collaboration |

Use SSE when you only need server-to-client updates. Use WebSocket when the client needs to send data back to the server through the same connection.

## See Also

- [Real-Time Overview](realtime-overview.md) -- All real-time features
- [Server-Sent Events](sse.md) -- One-way streaming alternative
- [PubSub](pubsub.md) -- Underlying messaging system
