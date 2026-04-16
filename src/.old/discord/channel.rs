use serde_repr::{Deserialize_repr, Serialize_repr};


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum ChannelType {
    GuildText = 0,
    Dm = 1,
    GuildVoice = 2,
    GroupDM = 3,
    GuildCategory = 4,
    GuildAnnouncement = 5,
    AnnouncementThread = 10,
    PublicThread = 11,
    PrivateThread = 12,
    GuildStageVoice = 13,
    GuildDirectory = 14,
    GuildForum = 15,
    GuildMedia = 16,
}