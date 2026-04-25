use twilight_model::{
    application::command::CommandOptionChoice,
    channel::message::{AllowedMentions, Component, Embed, MessageFlags},
    http::{
        attachment::Attachment,
        interaction::{InteractionResponse, InteractionResponseData, InteractionResponseType},
    },
    poll::Poll,
};

#[derive(serde::Serialize, Default)]
pub struct InteractionUpdateResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embeds: Option<Vec<Embed>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<Vec<Component>>,
}

#[allow(unused)]
pub trait InteractionResponseExt {
    fn new(kind: InteractionResponseType) -> Self;
    fn set_kind(&mut self, kind: InteractionResponseType);
    fn set_content(&mut self, content: impl Into<String>);
    fn set_components(&mut self, components: Vec<Component>);
    fn push_component(&mut self, component: Component);
    fn set_embeds(&mut self, embeds: Vec<Embed>);
    fn set_ephemeral(&mut self);
    fn set_tts(&mut self, tts: bool);
    fn set_allowed_mentions(&mut self, mentions: AllowedMentions);
    fn set_attachments(&mut self, attachments: Vec<Attachment>);
    fn set_choices(&mut self, choices: Vec<CommandOptionChoice>);
    fn set_custom_id(&mut self, id: impl Into<String>);
    fn set_title(&mut self, title: impl Into<String>);
    fn set_poll(&mut self, poll: Poll);

    fn empty() -> InteractionResponse;

    fn as_update(&self) -> InteractionUpdateResponse;
}

impl InteractionResponseExt for InteractionResponse {
    fn new(kind: InteractionResponseType) -> Self {
        Self {
            kind: kind,
            data: None,
        }
    }

    fn empty() -> InteractionResponse {
        InteractionResponse::new(InteractionResponseType::ChannelMessageWithSource)
    }

    fn as_update(&self) -> InteractionUpdateResponse {
        let Some(data) = self.data.clone() else {
            return InteractionUpdateResponse::default();
        };

        InteractionUpdateResponse {
            content: data.content,
            embeds: data.embeds,
            components: data.components,
        }
    }

    fn set_kind(&mut self, kind: InteractionResponseType) {
        self.kind = kind;
    }

    fn set_content(&mut self, content: impl Into<String>) {
        self.data
            .get_or_insert_with(InteractionResponseData::default)
            .content = Some(content.into());
    }

    fn set_components(&mut self, components: Vec<Component>) {
        self.data
            .get_or_insert_with(InteractionResponseData::default)
            .components = Some(components);
    }

    fn push_component(&mut self, component: Component) {
        let data = self.data.get_or_insert_with(InteractionResponseData::default);
        
        data.components.get_or_insert_with(Vec::new).push(component);
    }

    fn set_embeds(&mut self, embeds: Vec<Embed>) {
        self.data
            .get_or_insert_with(InteractionResponseData::default)
            .embeds = Some(embeds);
    }

    fn set_ephemeral(&mut self) {
        let data = self
            .data
            .get_or_insert_with(InteractionResponseData::default);
        let flags = data.flags.unwrap_or(MessageFlags::empty());
        data.flags = Some(flags | MessageFlags::EPHEMERAL);
    }

    fn set_tts(&mut self, tts: bool) {
        self.data
            .get_or_insert_with(InteractionResponseData::default)
            .tts = Some(tts);
    }

    fn set_allowed_mentions(&mut self, mentions: AllowedMentions) {
        self.data
            .get_or_insert_with(InteractionResponseData::default)
            .allowed_mentions = Some(mentions);
    }

    fn set_attachments(&mut self, attachments: Vec<Attachment>) {
        self.data
            .get_or_insert_with(InteractionResponseData::default)
            .attachments = Some(attachments);
    }

    fn set_choices(&mut self, choices: Vec<CommandOptionChoice>) {
        self.data
            .get_or_insert_with(InteractionResponseData::default)
            .choices = Some(choices);
    }

    fn set_custom_id(&mut self, id: impl Into<String>) {
        self.data
            .get_or_insert_with(InteractionResponseData::default)
            .custom_id = Some(id.into());
    }

    fn set_title(&mut self, title: impl Into<String>) {
        self.data
            .get_or_insert_with(InteractionResponseData::default)
            .title = Some(title.into());
    }

    fn set_poll(&mut self, poll: Poll) {
        self.data
            .get_or_insert_with(InteractionResponseData::default)
            .poll = Some(poll);
    }
}

pub struct ResponseBuilder {
    response: InteractionResponse,
}

#[allow(unused)]
impl ResponseBuilder {
    pub fn new(kind: InteractionResponseType) -> Self {
        Self {
            response: InteractionResponse::new(kind),
        }
    }

    pub fn from_response(response: InteractionResponse) -> Self {
        Self { response }
    }

    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.response.set_content(content);
        self
    }

    pub fn components(mut self, components: Vec<Component>) -> Self {
        self.response.set_components(components);
        self
    }

    pub fn push_component(mut self, component: Component) -> Self {
        self.response.push_component(component);
        self
    }

    pub fn embeds(mut self, embeds: Vec<Embed>) -> Self {
        self.response.set_embeds(embeds);
        self
    }

    pub fn ephemeral(mut self) -> Self {
        self.response.set_ephemeral();
        self
    }

    pub fn tts(mut self, tts: bool) -> Self {
        self.response.set_tts(tts);
        self
    }

    pub fn allowed_mentions(mut self, mentions: AllowedMentions) -> Self {
        self.response.set_allowed_mentions(mentions);
        self
    }

    pub fn attachments(mut self, attachments: Vec<Attachment>) -> Self {
        self.response.set_attachments(attachments);
        self
    }

    pub fn choices(mut self, choices: Vec<CommandOptionChoice>) -> Self {
        self.response.set_choices(choices);
        self
    }

    pub fn custom_id(mut self, id: impl Into<String>) -> Self {
        self.response.set_custom_id(id);
        self
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.response.set_title(title);
        self
    }

    pub fn poll(mut self, poll: Poll) -> Self {
        self.response.set_poll(poll);
        self
    }

    /// Restituisce la risposta costruita
    pub fn build(self) -> InteractionResponse {
        self.response
    }
}
