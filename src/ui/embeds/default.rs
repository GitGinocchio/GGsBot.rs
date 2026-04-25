use std::sync::LazyLock;

use crate::{framework::discord::embed::EmbedBuilder, ui::embeds::BOT_ICON_URL};

pub static DEFAULT_EMBED: LazyLock<EmbedBuilder> = LazyLock::new(|| {
    EmbedBuilder::new().color("#2ecc71").author(
        "GGsBot",
        Some(BOT_ICON_URL.into()),
        Some("https://discord.com/oauth2/authorize?client_id=1493638725488476311".into()),
    )
});
