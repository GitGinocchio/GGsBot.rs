use std::collections::HashMap;

use twilight_model::{application::command::{CommandOptionChoice, CommandOptionChoiceValue}, http::interaction::{InteractionResponse, InteractionResponseType}};

use crate::framework::discord::response::InteractionResponseExt;

pub struct Autocomplete {
    response: InteractionResponse
}

#[allow(unused)]
impl Autocomplete {
    pub fn new() -> Self {
        Self {
            response: InteractionResponse::new(InteractionResponseType::ApplicationCommandAutocompleteResult)
        }
    }

    pub fn get_choices(&self) -> &[CommandOptionChoice] {
        self.response.data
            .as_ref()
            .and_then(|d| d.choices.as_ref())
            .map(|c| c.as_slice())
            .unwrap_or(&[])
    }

    pub fn add_choice(&mut self, 
        name: impl Into<String>, 
        value: CommandOptionChoiceValue, 
        localitazions: Option<HashMap<String, String>>
    ) -> bool {
        let choice = CommandOptionChoice {
            name: name.into(),
            value: value,
            name_localizations: localitazions,
        };

        self.response.add_choice(choice)
    }

    pub fn build(self) -> InteractionResponse {
        self.response
    }
}