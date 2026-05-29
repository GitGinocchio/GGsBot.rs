
CREATE TABLE IF NOT EXISTS gateway_session (
    shard_id INTEGER PRIMARY KEY,
    last_sequence INTEGER,
    session_id TEXT,
    resume_url TEXT
)