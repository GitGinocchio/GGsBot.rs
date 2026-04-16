use twilight_model::application::command::{CommandOption, CommandOptionChoice, CommandOptionChoiceValue, CommandOptionType};

pub trait CommandOptionExt {
    fn required(self, required: bool) -> Self;
    #[allow(dead_code)]
    fn autocomplete(self, autocomplete: bool) -> Self;
    #[allow(dead_code)]
    fn add_choice(self, name: impl Into<String>, value: CommandOptionChoiceValue) -> Self;
}

impl CommandOptionExt for CommandOption {
    fn required(mut self, required: bool) -> Self {
        self.required = Some(required);
        self
    }

    fn autocomplete(mut self, autocomplete: bool) -> Self {
        self.autocomplete = Some(autocomplete);
        self
    }

    fn add_choice(mut self, name: impl Into<String>, value: CommandOptionChoiceValue) -> Self {
        let choice = CommandOptionChoice {
            name: name.into(),
            value: value,
            name_localizations: None,
        };
        
        if let Some(ref mut choices) = self.choices {
            choices.push(choice);
        } else {
            self.choices = Some(vec![choice]);
        }
        self
    }
}

pub fn create_option(kind: CommandOptionType, name: &str, desc: &str) -> CommandOption {
    CommandOption {
        kind,
        name: name.to_string(),
        description: desc.to_string(),
        required: Some(false),
        autocomplete: Some(false),
        channel_types: None,
        choices: None,
        description_localizations: None,
        max_length: None,
        max_value: None,
        min_length: None,
        min_value: None,
        name_localizations: None,
        options: None,
    }
}