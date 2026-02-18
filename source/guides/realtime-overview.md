# Real-Time Features

Yeti provides built-in real-time capabilities for streaming data changes to connected clients. Three mechanisms work together to deliver live updates: Server-Sent Events (SSE) for one-way streaming, WebSocket for bidirectional communication, and PubSub as the internal messaging backbone.

## Overview

| Feature | Direction | Protocol | Use Case |
|---------|-----------|----------|----------|
| [SSE](sse.md) | Server to client | HTTP/1.1, HTTP/2 | Live dashboards, notifications, feeds |
| [WebSocket](websocket.md) | Bidirectional | WS/WSS | Chat, collaborative editing, gaming |
| [PubSub](pubsub.md) | Internal | In-process channels | Connects data changes to SSE/WS streams |

## Enabling Real-Time

Enable SSE and/or WebSocket in your schema using the `@export` directive:

```graphql
type Message @table(database: "realtime-demo") @export(
    rest: true
    sse: true
    ws: true
) {
    id: ID! @primaryKey
    title: String!
    content: String!
    __createdAt__: String
}
```

Also set the corresponding flags in `config.yaml`:

```yaml
name: "Real-time Demo"
app_id: "realtime-demo"
rest: true
sse: true
schemas:
  - schema.graphql
```

## How It Works

When a record is created, updated, or deleted in a table with real-time enabled:

1. The table operation triggers the **PubSub** system, which publishes a notification to both the table-level topic (e.g., `"Message"`) and the record-level topic (e.g., `"Message/msg-1"`).
2. **SSE connections** subscribed to the table receive the update as an SSE event.
3. **WebSocket connections** subscribed to the table receive the update as a JSON message.

```
Client A: POST /Message {"id":"msg-1","title":"Hello","content":"World"}
                |
                v
         +-----------+
         |  PubSub   |  notify_update("Message", "msg-1", data)
         +-----------+
           /        \
          v          v
   SSE clients    WS clients
   (EventSource)  (WebSocket)
```

## Quick Example

Open two terminal windows:

**Terminal 1** -- Subscribe to SSE updates:
```bash
curl -sk "https://localhost:9996/realtime-demo/message?stream=sse"
```

**Terminal 2** -- Create a record:
```bash
curl -sk -X POST https://localhost:9996/realtime-demo/message \
  -H "Content-Type: application/json" \
  -d '{"id": "msg-1", "title": "Hello", "content": "Real-time works!"}'
```

Terminal 1 will immediately display:
```
event: message
data: {"type":"connected","status":"ok"}

event: update
id: msg-1
data: {"id":"msg-1","title":"Hello","content":"Real-time works!"}
```

## Real-Time Demo Application

The `realtime-demo` application provides a complete working example with a React UI that demonstrates live message streaming. Start Yeti and visit `https://localhost:9996/realtime-demo/` to see real-time updates in action.

## Topic Granularity

PubSub supports two levels of subscription:

- **Table-level** (`"Message"`): Receives all changes to any record in the table.
- **Record-level** (`"Message/msg-1"`): Receives changes only to a specific record.

SSE and WebSocket subscriptions to `/{table}` use table-level topics. Subscriptions to `/{table}/{id}` use record-level topics.

## Sub-Guides

- [Server-Sent Events](sse.md) -- One-way streaming protocol
- [WebSocket](websocket.md) -- Bidirectional communication
- [PubSub](pubsub.md) -- Internal messaging system
