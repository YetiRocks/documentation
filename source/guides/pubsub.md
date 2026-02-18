# PubSub

PubSub is Yeti's internal publish/subscribe messaging system. It provides the foundation for real-time features by connecting data changes to SSE and WebSocket streams. PubSub is an in-process system -- it does not expose external endpoints but powers all real-time delivery.

## Architecture

```
                     PubSubManager
  ┌──────────────────────────────────────────┐
  │            Topics (HashMap)              │
  │                                          │
  │  "User"        -> broadcast::Sender      │
  │  "User/123"    -> broadcast::Sender      │
  │  "Message"     -> broadcast::Sender      │
  │  "Message/abc" -> broadcast::Sender      │
  └──────────────────────────────────────────┘
           |                    |
     subscribe("User")   notify_update("User", "123", data)
           |                    |
           v                    v
    Receiver<Message>    Broadcasts to all subscribers
```

## Topic Naming

PubSub uses a simple hierarchical topic naming scheme:

| Topic | Scope | Example |
|-------|-------|---------|
| `"{Table}"` | Table-level | `"User"` -- all User changes |
| `"{Table}/{id}"` | Record-level | `"User/123"` -- changes to User 123 |
| Custom string | Application-level | `"notifications"` -- custom events |

When a record is created, updated, or deleted, PubSub publishes to **both** the record-level and table-level topics. This means a table-level subscriber sees all changes, while a record-level subscriber only sees changes to that specific record.

## Message Types

Each PubSub message carries a type that indicates what happened:

| Type | Description | Triggered By |
|------|-------------|--------------|
| `Update` | Record created or updated | POST, PUT operations |
| `Delete` | Record deleted | DELETE operations |
| `Publish` | Custom application message | Application code |
| `Retained` | Historical data on connect | Initial subscription |

## Message Structure

```json
{
  "message_type": "Update",
  "data": {
    "id": "user-1",
    "name": "Alice",
    "email": "alice@example.com"
  },
  "id": "user-1",
  "timestamp": "2025-06-15T10:30:00Z"
}
```

## How PubSub Connects to SSE and WebSocket

PubSub is the internal engine. SSE and WebSocket are the delivery mechanisms:

1. A client opens an SSE or WebSocket connection to `/{app}/{table}`.
2. The connection handler calls `pubsub.subscribe("{Table}")`, receiving a `broadcast::Receiver`.
3. As the receiver yields messages, they are formatted as SSE events or WebSocket frames and sent to the client.
4. When the client disconnects, the receiver is dropped.

```
REST PUT /User/123 {"name":"Updated"}
        |
        v
  TableResource::update()
        |
        v
  pubsub.notify_update("User", "123", data)
        |
        +----> "User/123" topic -> record-level subscribers
        |
        +----> "User" topic -> table-level subscribers
                    |
                    +----> SSE stream -> EventSource client
                    |
                    +----> WebSocket -> WS client
```

## Channel Capacity

Each PubSub topic uses a `tokio::sync::broadcast` channel with a capacity of **256 messages**. If a subscriber falls behind by more than 256 messages, older messages are dropped. This prevents slow consumers from blocking the entire system.

For most applications, 256 messages provides ample buffer. If you have extremely high-throughput tables, consider using record-level subscriptions to reduce per-topic volume.

## Topic Lifecycle

- **Creation**: Topics are created lazily on the first `subscribe()` call.
- **Cleanup**: Topics can be removed with `remove_topic()` when no longer needed.
- **Thread safety**: All operations use `RwLock` -- reads (publish, subscribe to existing topic) are concurrent; writes (new topic creation) acquire exclusive access.

## Custom Publish

Extensions and custom resources can publish application-level messages directly:

```rust
// Publish a custom notification
pubsub.notify_publish("alerts", serde_json::json!({
    "severity": "warning",
    "message": "Disk usage above 80%"
})).await;
```

Clients subscribed to the `"alerts"` topic will receive this as a `Publish` event.

## PubSub Manager API

| Method | Description |
|--------|-------------|
| `subscribe(topic)` | Subscribe to a topic, returns a Receiver |
| `publish(topic, message)` | Send a message to all subscribers |
| `notify_update(table, id, data)` | Publish update to record and table topics |
| `notify_delete(table, id)` | Publish deletion to record and table topics |
| `notify_publish(topic, data)` | Publish a custom message |
| `topic_count()` | Number of active topics |
| `subscriber_count(topic)` | Number of subscribers for a topic |
| `remove_topic(topic)` | Remove a topic and drop subscribers |

## See Also

- [Real-Time Overview](realtime-overview.md) -- All real-time features
- [Server-Sent Events](sse.md) -- SSE delivery mechanism
- [WebSocket](websocket.md) -- WebSocket delivery mechanism
