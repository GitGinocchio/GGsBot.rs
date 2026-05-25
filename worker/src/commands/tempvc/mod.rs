use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use twilight_model::{application::interaction::Interaction, gateway::{event::EventType, payload::incoming::VoiceStateUpdate}, guild::Permissions, http::{interaction::InteractionResponse, permission_overwrite::PermissionOverwriteType}, id::{Id, marker::ChannelMarker}};
use worker::RouteContext;

use crate::{
    bindings::{KV_BINDING, QueueBinding}, build_commands, commands::tempvc, error::Error, framework::{discord::command::{Command, CommandMap}, structs::{config::extension::ExtensionConfig, kv::NamespacedKv, queue::QueueMessage}, traits::{command::{CommandController, CommandEvents}, namespaces::KvExt}}, queues::tempvc::TempvcDeleteChannelMessage, services::discord::DiscordService
};

mod new;
mod del;

#[derive(Default)]
pub(crate) struct Tempvc {}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct TempvcExtConfig {
    generators: Vec<String>,
    channels: Vec<String>
}

#[async_trait(?Send)]
impl CommandController for Tempvc {
    async fn get_default_config(&self, _interaction: &Interaction, _ctx: &mut RouteContext<()>) -> Option<Value> {
        serde_json::to_value(TempvcExtConfig::default()).ok()
    }

    async fn on_setup(
        &self,
        interaction: &Interaction,
        ctx: &mut RouteContext<()>,
    ) -> Option<Result<InteractionResponse, Error>> {
        let guild_kv = match interaction.guild_kv(&ctx.env) {
            Ok(kv) => kv,
            Err(e) => return Some(Err(e))
        };

        let outcome = guild_kv.duplicate(
            &format!("extensions:{}:config:pending", self.name()),
            &format!("extensions:{}:config", self.name()),
            None
        ).await;

        match outcome {
            Ok(_) => {},
            Err(e) => return Some(Err(Error::KvError(e)))
        };

        None
    }
}

#[async_trait(?Send)]
impl CommandEvents for Tempvc {
    fn responds_to(&self, event_type: EventType) -> bool {
        matches!(event_type, EventType::VoiceStateUpdate)
    }

    async fn on_voice_state_update(&self, ctx: &RouteContext<()>, state: VoiceStateUpdate, metadata: Value) -> Result<Value, Error> {
        worker::console_debug!("[tempvc-events] Received voice_state_update: {state:?}");
        let Some(guild_id) = state.guild_id else { 
            return Err(Error::Generic("Missing guild_id".into())) 
        };

        let kv = ctx.kv(KV_BINDING)?;
        let guild_key = format!("guilds:{}", guild_id);
        let guild_kv = NamespacedKv::new(kv, guild_key.clone());

        let config_key = format!("extensions:{}:config", self.name());

        let maybe_ext = guild_kv
            .get_json::<ExtensionConfig<TempvcExtConfig>>(&config_key).await?;

        let Some(mut extension) = maybe_ext else {
            return Ok(Value::String(format!("No tempvc config for this server!")))
        };

        let discord = DiscordService::new(&ctx.env)?;

        let Some(channel_id) = state.channel_id else {
            let before_channel_id = metadata
                .get("before_channel_id")
                .and_then(|v| v.as_str())
                .ok_or_else(|| Error::InvalidPayload("Missing before_channel_id".to_string()))?
                .parse::<Id<ChannelMarker>>()
                .map_err(|_| Error::InvalidPayload("Malformed before_channel_id".to_string()))?;

            let channels = &mut extension.config
                .get_or_insert_default()
                .channels;

            if !channels.contains(&before_channel_id.to_string()) {
                return Ok(Value::String(format!("before channel is not a custom voice channel!")));
            }

            /*
            let task_queue = ctx.env.queue(&QueueBinding::Tasks.to_string())?;

            task_queue.send(QueueMessage::Tempvc(TempvcDeleteChannelMessage {
                channel_id: before_channel_id.into()
            })).await?;
            */

            discord.delete_channel(before_channel_id).await?;

            channels.retain(|channel| channel != &before_channel_id.to_string());
            guild_kv.put(&config_key, serde_json::to_string(&extension)?, None).await?;

            worker::console_log!("[tempvc-events] User left channel. Previous: {}", before_channel_id);

            // L'utente e' uscito dal canale...
            return Ok(Value::String(format!("User left the channel!")))
        };

        let Some(member) = &state.member else {
            return Err(Error::Generic("Missing member".into())) 
        };

        {
            let generators = &extension.config
                .get_or_insert_default()
                .generators;

            if !generators.contains(&channel_id.get().to_string()) {
                return Ok(Value::String(format!("channel not present in config: {channel_id} not in {generators:?}")));
            }
        }

        let current_channel = discord.get_channel(channel_id).await?;
        
        let new_channel_name = member.nick.clone()
            .unwrap_or_else(|| member.user.name.clone())
            + "'s Vocal Channel";

        let new_channel = discord.create_channel(
            guild_id, 
            new_channel_name.clone(),
            current_channel.kind,
            current_channel.parent_id.map(|id| id.cast()),
            None
        ).await?;

        let allow = Permissions::empty()
            .union(Permissions::CONNECT)
            .union(Permissions::SPEAK)
            .union(Permissions::STREAM)
            .union(Permissions::MANAGE_CHANNELS)
            .union(Permissions::MOVE_MEMBERS)
            .union(Permissions::DEAFEN_MEMBERS)
            .union(Permissions::PRIORITY_SPEAKER)
            .union(Permissions::VIEW_CHANNEL);

        discord.set_permissions(
            new_channel.id, 
            member.user.id, 
            Some(allow), 
            None, 
            PermissionOverwriteType::Member
        ).await?;

        discord.move_member(guild_id, member.user.id, new_channel.id).await?;

        let channels = &mut extension.config
            .get_or_insert_default()
            .channels;

        channels.push(new_channel.id.to_string());

        guild_kv.put(&config_key, serde_json::to_string(&extension)?, None).await?;

        Ok(Value::String(format!("Channel '{new_channel_name}' created successfully!")))
    }
}

#[async_trait(?Send)]
impl Command for Tempvc {
    fn name(&self) -> String {
        "tempvc".into()
    }

    fn description(&self) -> String {
        "Crea canali vocali personalizzati per te!".into()
    }

    fn subcommands(&self) -> CommandMap {
        build_commands![
            tempvc::new::New, 
            tempvc::del::Del
        ]
    }

    fn get_controller(&self) -> Option<&dyn CommandController> {
        Some(self)
    }

    fn get_events(&self) -> Option<&dyn CommandEvents> {
        Some(self)
    }
}
