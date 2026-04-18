use std::collections::HashMap;

use async_trait::async_trait;
use serde_json::Value;
use twilight_model::{application::{command::{CommandOption, CommandOptionChoiceValue, CommandOptionType}, interaction::{Interaction, application_command::CommandData}}, channel::message::Embed, http::interaction::{InteractionResponse, InteractionResponseData, InteractionResponseType}};
use worker::RouteContext;

use crate::{
    map,
    COMMANDS, 
    commands::ext::REQUIRED_EXTENSIONS, 
    discord::{
        command::Command, 
        embed::EmbedExt, 
        response::ResponseBuilder
    }, 
    embeds::default::DEFAULT_EMBED, 
    error::InteractionError, 
    structs::config::extension::ExtensionConfig, 
    traits::namespaces::InteractionKvExt, 
    utils::capitalize
};

#[derive(Default)]
pub(crate) struct Show {
}

#[async_trait(?Send)]
impl Command for Show {
    fn name(&self) -> String { "show".into() }

    fn description(&self) -> String { "Mostra le estensioni abilitate/disabilitate o disponibili!".into() }

    async fn respond(
        &self, 
        interaction: &Interaction,
        _data: &CommandData,
        ctx: &mut RouteContext<()>
    ) -> Result<InteractionResponse, InteractionError> {
        let guild_kv = interaction.guild_kv(ctx)?;
        let extensions_list = guild_kv.list(Some("extensions".into()), Some(COMMANDS.len() as u64), None)
            .await
            .map_err(|e| InteractionError::KvError(e))?;

        let extensions_keys: Vec<String> = extensions_list.keys
            .into_iter()
            .map(|k| k.name)
            .collect();

        worker::console_debug!("{extensions_keys:?}");

        let extensions_config = if extensions_keys.len() > 0 {
            guild_kv.get_bulk(&extensions_keys)
                .await
                .map_err(|e| InteractionError::KvError(e))?
        } else {
            map!()
        };

        let status_map: HashMap<String, bool> = extensions_config
            .iter()
            .filter_map(|(key, maybe_val)| {
                let ext_name = key.split(':').nth(5)?.to_string();

                // TODO: capire perche' ritorna maybe_val: None anche se e' presente
                if let Some(val) = maybe_val { 
                    let config: ExtensionConfig<Value> = serde_json::from_str(val).ok()?;
                    Some((ext_name, config.enabled))
                }
                else {
                    Some((ext_name, true))
                }
            })
            .collect();

        worker::console_debug!("{status_map:?}");

        let mut configured_field = String::new();
        for (name, status) in status_map.iter() {
            configured_field.push_str(&format!(
                "- *{}*: **{}**\n", 
                capitalize(name),
                if *status == true { "enabled" } else { "disabled" }
            ))
        }

        let mut unused_field = String::new();
        for (name, _) in COMMANDS.iter() {
            if REQUIRED_EXTENSIONS.contains(&name.as_str()) { continue };
            if status_map.contains_key(name) { continue };
            unused_field.push_str(&format!("- *{}*\n", capitalize(name)))
        }

        let mut embed = DEFAULT_EMBED.clone()
            .title("GGsBot Extensions")
            .description("Here is a list of enabled, disabled and available extensions")
            .build();

        if !configured_field.is_empty() {
            embed.add_field("Configured", configured_field, false);
        }
        
        if !unused_field.is_empty() {
            embed.add_field("Unused", unused_field, false);
        }

        Ok(ResponseBuilder::new(InteractionResponseType::ChannelMessageWithSource)
            .embeds(vec![embed])
            .ephemeral()
            .build())
    }
}