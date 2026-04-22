use async_trait::async_trait;
use twilight_model::{application::interaction::Interaction, channel::{ChannelType, message::{Component, EmojiReactionType, component::{ActionRow, Button, ButtonStyle, ComponentType, SelectDefaultValue, SelectMenu, SelectMenuOption, SelectMenuType}}}, http::interaction::InteractionResponse, id::{Id, marker::SkuMarker}};
use worker::RouteContext;

use crate::error::Error;

#[async_trait(?Send)]
#[allow(unused)]
pub trait CustomComponent {
    fn id(&self) -> String;

    fn build(&self) -> Component;

    async fn handle(
        &self,
        interaction: &Interaction, 
        ctx: &mut RouteContext<()>,
        target: String
    ) -> Option<Result<InteractionResponse, Error>> {
        None
    }

    fn button(
        &self, 
        id: impl Into<String>, 
        label: Option<impl Into<String>>, 
        emoji: Option<EmojiReactionType>,
        style: ButtonStyle,
        url: Option<impl Into<String>>,
        disabled: bool,
        sku_id: Option<Id<SkuMarker>>
    ) -> Component {
        Component::Button(Button {
            id: None,
            custom_id: Some(self.id() + ":" + &id.into()),
            emoji: emoji,
            disabled: disabled,
            label: label.map(|l| l.into()),
            style: style,
            url: url.map(|u| u.into()),
            sku_id: sku_id
        })
    }

    fn action_row(&self, components: Vec<Component>) -> Component {
        Component::ActionRow(ActionRow { id: None, components: components })
    }

    fn select_menu(
        &self, 
        id: impl Into<String>, 
        kind: SelectMenuType,
        disabled: bool,
        placeholder: Option<String>,
        options: Option<Vec<SelectMenuOption>>,
        default_values: Option<Vec<SelectDefaultValue>>,
        max_values: Option<u8>,
        min_values: Option<u8>,
        required: Option<bool>,
        channel_types: Option<Vec<ChannelType>>
    ) -> Component {
        Component::SelectMenu(SelectMenu {
            id: None,
            channel_types: channel_types,
            custom_id: self.id() + ":" + &id.into(),
            default_values: default_values,
            kind: kind,
            disabled: disabled,
            max_values: max_values,
            min_values: min_values,
            options: options,
            placeholder: placeholder,
            required: required
        })
    }
}