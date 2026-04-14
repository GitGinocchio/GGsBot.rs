use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attachment {
    /// ID dell'allegato (Snowflake)
    pub id: String,
    
    /// Nome del file allegato
    pub filename: String,
    
    /// Titolo del file
    pub title: Option<String>,
    
    /// Descrizione (alt text) per il file (max 1024 caratteri)
    pub description: Option<String>,
    
    /// Media type dell'allegato (MIME type)
    pub content_type: Option<String>,
    
    /// Dimensione del file in byte
    pub size: u64,
    
    /// URL sorgente del file
    pub url: String,
    
    /// URL proxato del file
    pub proxy_url: String,
    
    /// Altezza del file (se immagine o video)
    pub height: Option<u32>,
    
    /// Larghezza del file (se immagine o video)
    pub width: Option<u32>,
    
    /// Placeholder per thumbhash (se immagine o video)
    pub placeholder: Option<String>,
    
    /// Versione del placeholder (se immagine o video)
    pub placeholder_version: Option<u32>,
    
    /// Se l'allegato è effimero (viene rimosso dopo un po')
    pub ephemeral: Option<bool>,
    
    /// Durata del file audio (per i messaggi vocali)
    pub duration_secs: Option<f32>,
    
    /// Waveform codificata in base64 (per i messaggi vocali)
    pub waveform: Option<String>,
    
    /// Flag dell'allegato (bitfield)
    pub flags: Option<u32>,
    
    /// Per i Clip: array di utenti nello stream
    pub clip_participants: Option<Vec<serde_json::Value>>, // Puoi sostituire con la tua struct User
    
    /// Per i Clip: quando è stato creato (ISO8601)
    pub clip_created_at: Option<String>,
    
    /// Per i Clip: l'applicazione nello stream
    pub application: Option<serde_json::Value>, // Puoi sostituire con la tua struct Application
}