
CREATE TABLE IF NOT EXISTS gateway_session (
    shard_id INTEGER PRIMARY KEY,
    last_sequence INTEGER,
    heartbeat_interval INTEGER DEFAULT 0,
    heartbeat_acknowledged INTEGER DEFAULT 1
)