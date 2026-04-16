use serde_repr::{Deserialize_repr, Serialize_repr};



#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum ApplicationIntegrationType {
    GuildInstall = 0,
    UserInstall = 1
}