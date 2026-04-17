use std::sync::LazyLock;

use crate::discord::embed::EmbedBuilder;



pub static ERROR_EMBED: LazyLock<EmbedBuilder> = LazyLock::new(|| {
    EmbedBuilder::new()
        .color("#ff0000")
        .author("", None::<String>, None)
});