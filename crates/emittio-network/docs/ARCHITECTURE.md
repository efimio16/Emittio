# `emittio-network` Architecture

`emittio-network` crate is responsible of handling connections and establishing sessions between peers using hybrid-RTT handshake and post-quantum KEM.

Hybrid-RTT means that the first payload is sent using 0-RTT session, but before first reply, it upgrades to 1-RTT session.

## Message hierarchy

```mermaid
flowchart TB
    packet["Packet (a raw piece of bytes with a length prefix)"]
    handshake["Handshake"]
    frame["Frame (encrypted FrameData with anti-replay protection)"]
    frame_data["FrameData"]
    query["Query"]
    reply["Reply"]
    chunk["Chunk (raw bytes)"]

    packet --> handshake
    packet --> frame
    frame --> frame_data
    frame_data --> query
    frame_data --> reply
    frame_data --> chunk
```

## Session

Encrypts and decrypts frame data. Handles nonces and protects from replays.

## Flows

### Network service

![Network service flow](media/network-service-flow.svg)

### Alice - Bob communication example

![Alice-Bob communication flow](media/alice-bob-communication-flow.svg)