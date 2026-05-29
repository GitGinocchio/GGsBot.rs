use worker::{Result, Storage};

const SCHEMA_SQL : &str = include_str!("../assets/gateway_storage.sql");

pub async fn setup_storage(storage: &Storage) -> Result<()> {
    let backend = storage.sql();

    for stmt in SCHEMA_SQL.split(";") {
        let trimmed = stmt.trim();
        if !trimmed.is_empty() && !trimmed.starts_with("--") {
            backend
                .exec(trimmed, None)
                .map_err(|e| worker::Error::from(format!("Errore esecuzione schema: {}", e)))?;
        }
    }

    Ok(())
}

pub const UPDATE_SEQ_QUERY: &str = r#"
    INSERT INTO gateway_session (id, last_sequence) 
    VALUES (1, ?) 
    ON CONFLICT(id) 
    DO UPDATE SET last_sequence = excluded.last_sequence;
"#;

pub const GET_SEQ_QUERY: &str = r#"
    SELECT last_sequence FROM gateway_session WHERE id = 1;
"#;

pub fn save_last_sequence(storage: &Storage, seq: u64) -> Result<()> {
    let seq_value = seq as i64;

    storage.sql().exec(UPDATE_SEQ_QUERY, vec![seq_value.into()])
        .map_err(|e| worker::Error::from(format!("Errore esecuzione SQL: {}", e)))?;

    Ok(())
}

pub fn get_last_sequence(storage: &Storage) -> Result<u64> {
    let cursor = storage.sql().exec(GET_SEQ_QUERY, None)
        .map_err(|e| worker::Error::from(format!("Errore esecuzione SQL (GET): {}", e)))?;

    Ok(cursor.one::<u64>().unwrap_or_default())
}