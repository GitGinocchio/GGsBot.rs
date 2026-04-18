use twilight_model::application::command::{
    CommandOption, CommandOptionChoice, CommandOptionChoiceValue, CommandOptionType
};

#[allow(unused)]
pub trait CommandOptionExt {
    fn new(kind: CommandOptionType, name: impl Into<String>, desc: impl Into<String>) -> Self;
    fn set_required(&mut self, required: bool) -> &mut Self;
    fn set_autocomplete(&mut self, autocomplete: bool) -> &mut Self;
    fn add_choice(&mut self, name: impl Into<String>, value: impl Into<CommandOptionChoiceValue>) -> &mut Self;
}

impl CommandOptionExt for CommandOption {
    fn new(kind: CommandOptionType, name: impl Into<String>, desc: impl Into<String>) -> Self {
        Self {
            kind,
            name: name.into(),
            description: desc.into(),
            required: None,
            autocomplete: None,
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

    fn set_required(&mut self, required: bool) -> &mut Self {
        self.required = Some(required);
        self
    }

    fn set_autocomplete(&mut self, autocomplete: bool) -> &mut Self {
        self.autocomplete = Some(autocomplete);
        self
    }

    fn add_choice(&mut self, name: impl Into<String>, value: impl Into<CommandOptionChoiceValue>) -> &mut Self {
        let choice = CommandOptionChoice {
            name: name.into(),
            value: value.into(),
            name_localizations: None,
        };
        self.choices.get_or_insert_with(Vec::new).push(choice);
        self
    }
}

#[derive(Clone)]
pub struct OptionBuilder(CommandOption);

#[allow(unused)]
impl OptionBuilder {
    pub fn new(kind: CommandOptionType, name: impl Into<String>, desc: impl Into<String>) -> Self {
        Self(CommandOption {
            kind,
            name: name.into(),
            description: desc.into(),
            required: None,
            autocomplete: None,
            channel_types: None,
            choices: None,
            description_localizations: None,
            max_length: None,
            max_value: None,
            min_length: None,
            min_value: None,
            name_localizations: None,
            options: None,
        })
    }

    // Helper statici per i tipi più comuni
    pub fn string(name: impl Into<String>, desc: impl Into<String>) -> Self {
        Self::new(CommandOptionType::String, name, desc)
    }

    pub fn integer(name: impl Into<String>, desc: impl Into<String>) -> Self {
        Self::new(CommandOptionType::Integer, name, desc)
    }

    pub fn required(mut self, required: bool) -> Self {
        self.0.set_required(required);
        self
    }

    pub fn autocomplete(mut self, autocomplete: bool) -> Self {
        self.0.set_autocomplete(autocomplete);
        self
    }

    pub fn choice(mut self, name: impl Into<String>, value: impl Into<CommandOptionChoiceValue>) -> Self {
        self.0.add_choice(name, value);
        self
    }

    pub fn build(self) -> CommandOption {
        self.0
    }
}