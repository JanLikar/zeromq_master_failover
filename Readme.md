# zeromq_master_failover
An experiment in implementing master failover with ZeroMQ.

The system is designed to be highly available and only eventually consistent.

## Running
Run in separate terminals.

    cargo run --bin zeromq_master_failover -- 3001 4001 3002
    cargo run --bin zeromq_master_failover -- 4001 3001 4002
    cargo run --bin client -- 3002 4002

In this configuration, first node opens a PUB socket on 3001, a REP socket (for receiving client commands) on 3002 and listens port 4001 for replication updates.
