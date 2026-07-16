use flarecord::{error::{Error, BotResult}, models::{command::{CommandOptions, Subcommand, context::CommandContext, interaction::CommandInteraction, option::CommandOption, response::CommandResponse}}};


pub struct Clear;

impl Subcommand for Clear {
    fn name(&self) -> String {
        "clear".into()
    }

    fn description(&self) -> String {
        "Delete messages from a chat!".into()
    }

    fn options(&self) -> BotResult<CommandOptions> {
        let amount = CommandOption::integer("amount", "The amount of messages to delete")?;
        Ok(Some(vec![amount]))
    }

    async fn execute(
        &self,
        interaction: CommandInteraction,
        ctx: CommandContext
    ) -> BotResult<CommandResponse> {
        interaction.defer(true).await?;

        if !interaction.is_guild() {
            return interaction.edit(CommandResponse::builder()
                .content("⚠️ This command can only be used inside a guild")
                .build())
                .await;
        }

        let amount = interaction.data
            .get_option_integer("amount")
            .unwrap_or(10);

        if amount < 1 || amount > 100 {
            return interaction.edit(CommandResponse::builder()
                .content("⚠️ You can delete 1 to 100 messages...")
                .build())
                .await;
        }

        let channel_id = interaction
            .channel
            .as_ref()
            .map(|c| c.id)
            .ok_or_else(|| Error::InvalidInteraction("Missing interaction channel".into()))?;

        let content = match ctx.discord.delete_messages(channel_id, amount as u8).await {
            Err(e) => e.to_string(),
            Ok(num) => format!("🗑️ Deleted **{}** messages", num),
        };

        let response = CommandResponse::builder()
            .content(content)
            .build();

        interaction.edit(response).await
    }
}