use std::sync::LazyLock;

use crate::discord::embed::EmbedBuilder;

pub static BOT_ICON_URL: &'static str = "https://cdn.discordapp.com/app-icons/1493638725488476311/d7ff4726f7cf04698b155a1460fedcab.png?size=256&quot";

pub static DEFAULT_EMBED: LazyLock<EmbedBuilder> = LazyLock::new(|| {
    EmbedBuilder::new()
        .color("#2ecc71")
        .author(
            "GGsBot", 
            Some(BOT_ICON_URL.into()), 
            None
        )
        .footer(
            format!("Message sent from GGsBot!"), 
            Some(BOT_ICON_URL.into())
        )
});