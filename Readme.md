# zeromq_master_failover
An experiment in implementing a multi-master distributed key-value store with failover using ZeroMQ.

The system is designed to be highly available and only eventually consistent.

## Semantics
Each of the entries has an associated TTL and is evicted once the TTL is reached.

This enables a simplified approach to dealing with conflict resolution -- the entry with a validity that is the furthest in the future always wins.

## Protocol
Nodes currently implement three commands:

    SET key val ttl
    GET key
    KEYS

On success, they return messages prepended with `OK`, `ERR` on error and `NO` if key is not present.

## Running
Run in separate terminals.

    cargo run --bin zeromq_master_failover -- 3001 4001 3002
    cargo run --bin zeromq_master_failover -- 4001 3001 4002
    cargo run --bin client -- 3002 4002

In this configuration, first node opens a PUB socket on 3001, a REP socket (for receiving client commands) on 3002 and listens port 4001 for replication updates.
