use bitflags::bitflags;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::discord::{attachment::Attachment, interaction::User};

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct MessageFlags: u32 {
        const CROSSPOSTED = 1 << 0;
        const IS_CROSSPOST = 1 << 1;
        const SUPPRESS_EMBEDS = 1 << 2;
        const SOURCE_MESSAGE_DELETED = 1 << 3;
        const URGENT = 1 << 4;
        const HAS_THREAD = 1 << 5;
        const EPHEMERAL = 1 << 6;
        const LOADING = 1 << 7;
        const FAILED_TO_MENTION_SOME_ROLES_IN_THREAD = 1 << 8;
        const SUPPRESS_NOTIFICATIONS = 1 << 12;
        const IS_VOICE_MESSAGE = 1 << 13;
        const HAS_SNAPSHOT = 1 << 14;
        const IS_COMPONENTS_V2 = 1 << 15;
    }
}

impl Serialize for MessageFlags {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u32(self.bits())
    }
}

impl<'de> Deserialize<'de> for MessageFlags {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let bits = u32::deserialize(deserializer)?;
        Ok(MessageFlags::from_bits_retain(bits))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    /// ID del messaggio (Snowflake)
    pub id: String,
    
    /// ID del canale in cui è stato inviato il messaggio
    pub channel_id: String,
    
    /// L'autore di questo messaggio (User object)
    pub author: User,
    
    /// Contenuto del messaggio
    pub content: String,
    
    /// Quando il messaggio è stato inviato (ISO8601 timestamp)
    pub timestamp: DateTime<Utc>,
    
    /// Quando il messaggio è stato modificato (null se mai modificato)
    pub edited_timestamp: Option<String>,
    
    /// Se è un messaggio Text-To-Speech
    pub tts: bool,
    
    /// Se il messaggio menziona @everyone
    pub mention_everyone: bool,
    
    /// Utenti specificamente menzionati nel messaggio
    pub mentions: Vec<User>,
    
    /// ID dei ruoli menzionati nel messaggio
    pub mention_roles: Vec<String>,
    
    /// Canali menzionati nel messaggio
    pub mention_channels: Option<Vec<serde_json::Value>>,
    
    /// File allegati
    pub attachments: Vec<Attachment>,
    
    /// Embed contenuti nel messaggio
    pub embeds: Vec<serde_json::Value>, // Sostituisci con Vec<Embed> se hai la struct
    
    /// Reazioni al messaggio
    pub reactions: Option<Vec<serde_json::Value>>,
    
    /// Usato per validare che un messaggio sia stato inviato
    pub nonce: Option<serde_json::Value>, // Può essere integer o string
    
    /// Se il messaggio è fissato (pinned)
    pub pinned: bool,
    
    /// Se generato da un webhook, il suo ID
    pub webhook_id: Option<String>,
    
    /// Tipo di messaggio (Integer)
    #[serde(rename = "type")]
    pub message_type: u8,
    
    /// Attività associata (Rich Presence)
    pub activity: Option<serde_json::Value>,
    
    /// Applicazione associata
    pub application: Option<serde_json::Value>,
    
    /// ID dell'applicazione (se è un'interazione)
    pub application_id: Option<String>,
    
    /// Flag del messaggio (bitfield)
    pub flags: Option<MessageFlags>,
    
    /// Riferimento a un altro messaggio (reply, crosspost, ecc.)
    pub message_reference: Option<serde_json::Value>,
    
    /// Il messaggio a cui si riferisce la reply
    pub referenced_message: Option<Box<Message>>,
    
    /// Metadati dell'interazione (nuovo standard)
    pub interaction_metadata: Option<serde_json::Value>,
    
    /// Il thread avviato da questo messaggio
    pub thread: Option<serde_json::Value>,
    
    /// Componenti interattivi (Bottoni, ecc.)
    pub components: Option<Vec<serde_json::Value>>,
    
    /// Sticker associati
    pub sticker_items: Option<Vec<serde_json::Value>>,
    
    /// Posizione approssimativa nel thread
    pub position: Option<u64>,
    
    /// Sondaggi
    pub poll: Option<serde_json::Value>,
}