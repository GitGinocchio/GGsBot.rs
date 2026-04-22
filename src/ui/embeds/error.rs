use std::sync::LazyLock;

use crate::{
    discord::embed::EmbedBuilder, 
    ui::embeds::default::DEFAULT_EMBED
};



pub static ERROR_EMBED: LazyLock<EmbedBuilder> = LazyLock::new(|| {
    DEFAULT_EMBED
        .clone()
        .color("#ff0000")
        .title("Error!")
});