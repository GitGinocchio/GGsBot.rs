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

// Questa query ora gestisce l'aggiornamento parziale di qualsiasi colonna in modo sicuro
pub const UPSERT_SESSION_QUERY: &str = r#"
    INSERT INTO gateway_session (shard_id, last_sequence, heartbeat_interval, heartbeat_acknowledged) 
    VALUES (?, ?, ?, ?) 
    ON CONFLICT(shard_id) 
    DO UPDATE SET 
        last_sequence = COALESCE(excluded.last_sequence, gateway_session.last_sequence),
        heartbeat_interval = COALESCE(excluded.heartbeat_interval, gateway_session.heartbeat_interval),
        heartbeat_acknowledged = COALESCE(excluded.heartbeat_acknowledged, gateway_session.heartbeat_acknowledged);
"#;

pub const GET_SESSION_DATA_QUERY: &str = r#"
    SELECT last_sequence, heartbeat_interval, heartbeat_acknowledged FROM gateway_session WHERE shard_id = ?;
"#;

pub const DELETE_SESSION_QUERY: &str = r#"
    DELETE FROM gateway_session WHERE shard_id = ?;
"#;

/// Elimina completamente la riga dello shard dal database
pub fn delete_session(storage: &Storage, shard_id: u32) -> Result<()> {
    let shard_value = shard_id as i64;

    storage.sql().exec(DELETE_SESSION_QUERY, vec![shard_value.into()])
        .map_err(|e| worker::Error::from(format!("Errore durante l'eliminazione fisica della sessione: {}", e)))?;

    worker::console_debug!("[SQL] Sessione dello Shard {} eliminata dal DB.", shard_id);
    Ok(())
}

/// Salva l'ultima sequenza ricevuta da Discord (lascia invariati gli altri campi)
pub fn save_last_sequence(storage: &Storage, shard_id: u32, seq: u64) -> Result<()> {
    let shard_value = shard_id as i64;
    let seq_value = seq as i64;

    // Passiamo NULL (None) per gli altri parametri, così il COALESCE terrà i valori attuali nel DB
    storage.sql().exec(UPSERT_SESSION_QUERY, vec![
        shard_value.into(),
        seq_value.into(),
        None::<i64>.into(),
        None::<i64>.into(),
    ]).map_err(|e| worker::Error::from(format!("Errore salvataggio sequenza: {}", e)))?;

    Ok(())
}

/// Recupera l'ultima sequenza salvata
pub fn get_last_sequence(storage: &Storage, shard_id: u32) -> Result<u64> {
    let shard_value = shard_id as i64;
    let cursor = storage.sql().exec("SELECT last_sequence FROM gateway_session WHERE shard_id = ?;", vec![shard_value.into()])?;
    
    Ok(cursor.one::<u64>().unwrap_or_default())
}

/// Salva l'intervallo dell'heartbeat ricevuto dall'evento HELLO e resetta l'ACK a True (1)
pub fn save_heartbeat_interval(storage: &Storage, shard_id: u32, interval_ms: u64) -> Result<()> {
    let shard_value = shard_id as i64;
    let interval_value = interval_ms as i64;

    storage.sql().exec(UPSERT_SESSION_QUERY, vec![
        shard_value.into(),
        None::<i64>.into(),      // Non tocchiamo la sequenza
        interval_value.into(),   // Impostiamo l'intervallo
        1_i64.into(),            // Resettiamo l'ACK a 1 (true) visto che siamo appena partiti
    ]).map_err(|e| worker::Error::from(format!("Errore salvataggio intervallo heartbeat: {}", e)))?;

    Ok(())
}

/// Recupera l'intervallo dell'heartbeat (restituisce 0 se non configurato)
pub fn get_heartbeat_interval(storage: &Storage, shard_id: u32) -> Result<u64> {
    let shard_value = shard_id as i64;
    let cursor = storage.sql().exec("SELECT heartbeat_interval FROM gateway_session WHERE shard_id = ?;", vec![shard_value.into()])?;
    
    Ok(cursor.one::<u64>().unwrap_or_default())
}

/// Imposta lo stato dell'ACK dell'heartbeat (1 per confermato, 0 per in attesa)
pub fn set_heartbeat_ack(storage: &Storage, shard_id: u32, acknowledged: bool) -> Result<()> {
    let shard_value = shard_id as i64;
    let ack_value = if acknowledged { 1_i64 } else { 0_i64 };

    storage.sql().exec(UPSERT_SESSION_QUERY, vec![
        shard_value.into(),
        None::<i64>.into(),
        None::<i64>.into(),
        ack_value.into(),
    ]).map_err(|e| worker::Error::from(format!("Errore aggiornamento ACK heartbeat: {}", e)))?;

    Ok(())
}

/// Controlla se l'ultimo heartbeat ha ricevuto risposta (Default: true se la riga non esiste)
pub fn get_heartbeat_ack(storage: &Storage, shard_id: u32) -> Result<bool> {
    let shard_value = shard_id as i64;
    let cursor = storage.sql().exec("SELECT heartbeat_acknowledged FROM gateway_session WHERE shard_id = ?;", vec![shard_value.into()])?;
    
    let raw_ack = cursor.one::<i64>().unwrap_or(1);
    Ok(raw_ack == 1)
}