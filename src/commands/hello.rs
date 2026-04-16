use crate::discord::interaction::{
    InteractionApplicationCommandCallbackData
};
use crate::discord::error::InteractionError;
use crate::discord::command::{Command, CommandContext};
use crate::discord::locale::Localization;
use crate::discord::option::{ApplicationCommandOption, ApplicationCommandOptionType};

use async_trait::async_trait;

#[derive(Default)]
pub(crate) struct Hello {}

#[async_trait(?Send)]
impl Command for Hello {
    fn name(&self) -> Localization { "hello".into() }
    fn description(&self) -> Localization { "Say Hello to someone on this server!".into() }

    fn options(&self) -> Vec<ApplicationCommandOption> {
        vec![ApplicationCommandOption{
            name: "user".into(), 
            autocomplete: Some(true),
            description: "The user you want to greet".into(), 
            required: Some(false), 
            ty: ApplicationCommandOptionType::User,
            ..Default::default()
        }]
    }

    async fn respond(&self, ctx: &mut CommandContext) -> Result<InteractionApplicationCommandCallbackData, InteractionError>{
        let maybe_user = ctx.get_option("user")
            .map(|v| v.as_str())
            .flatten()
            .map(|s| format!("<@{}>", s))
            .unwrap_or("World".into());
        
        Ok(InteractionApplicationCommandCallbackData {
            content: Some(format!("Hello, {}!", maybe_user)),
            ..Default::default()
        })
    }
}