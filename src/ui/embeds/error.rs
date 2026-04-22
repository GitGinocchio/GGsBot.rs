use std::sync::LazyLock;

use crate::{
    discord::embed::EmbedBuilder, 
    ui::embeds::{
        default::DEFAULT_EMBED
    }
};

pub const ERROR_ICON: &str = "https://img.icons8.com/?size=100&id=59754&format=png&color=FA5252";
pub const BUG_ICON: &str = "https://img.icons8.com/?size=100&id=ldqGGT31WTA2&format=png&color=FA5252";
pub const HELP_ICON: &str = "https://img.icons8.com/?size=100&id=foEg0x6MA0FE&format=png&color=FA5252";

pub static ERROR_EMBED: LazyLock<EmbedBuilder> = LazyLock::new(|| {
    DEFAULT_EMBED
        .clone()
        .color("#ff0000")
        .title("Ops!")
        .description("An unexpected error occurred!")
        .author(
            "GGsBot",
            Some(ERROR_ICON.into()),
            Some("https://discord.com/oauth2/authorize?client_id=1493638725488476311".into())
        )
        .footer(
            "If you don't understand this error, feel free to contact a moderator for help.",
            Some(HELP_ICON.into())
        )
});