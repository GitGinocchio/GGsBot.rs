use async_trait::async_trait;
use twilight_model::channel::message::{Component, component::ButtonStyle};

use crate::framework::traits::component::CustomComponent;

pub struct SetupExtNavBar {
    parent_id: String,
    page: u8,
    max_page: Option<u8>,
    disable_confirm: bool
}

#[allow(unused)]
impl SetupExtNavBar {
    pub fn new(parent_id: impl Into<String>, start_page: Option<u8>, max_page: Option<u8>, disable_confirm: bool) -> Self {
        Self {
            parent_id: parent_id.into(),
            page: start_page.unwrap_or(0),
            max_page: max_page,
            disable_confirm: disable_confirm
        }
    }

    pub fn enable_confirm(&mut self) {
        self.disable_confirm = false;
    }

    pub fn disble_confirm(&mut self) {
        self.disable_confirm = true;
    }

    pub fn reset_page(&mut self) {
        self.page = 0;
    }

    pub fn set_page(&mut self, page: u8) {
        self.page = page;
    }

    pub fn get_page(&self) -> u8 {
        self.page
    }

    pub fn prev_page(&mut self) -> u8 {
        self.page = (self.page - 1).max(0);
        self.page
    }

    pub fn next_page(&mut self) -> u8 {
        self.page = self.page + 1;
        self.page
    }

    pub fn set_max_page(&mut self, max_page: Option<u8>) {
        self.max_page = max_page
    }
}

#[async_trait(?Send)]
impl CustomComponent for SetupExtNavBar {
    fn id(&self) -> String {
        format!("{}:nav:{}", self.parent_id, self.page)
    }

    fn build(&self) -> Component {
        let mut components = Vec::new();

        let render_back_btn = match self.max_page {
            Some(max) => max > 0,
            None => true
        };

        if render_back_btn {
            components.push(self.button(
                "back",
                Some("Back"),
                None,
                ButtonStyle::Secondary,
                None::<String>,
                if self.page == 0 { true } else { false },
                None,
            ));
        }

        components.push(self.button(
            "cancel",
            Some("Cancel"),
            None,
            ButtonStyle::Danger,
            None::<String>,
            false,
            None,
        ));

        let is_max_page = if let Some(max) = self.max_page && self.page >= max { true } else { false };

        components.push(self.button(
            if is_max_page { "confirm" } else { "next" },
            if is_max_page { Some("Confirm") } else { Some("Next") },
            None,
            if is_max_page {  ButtonStyle::Success } else { ButtonStyle::Primary },
            None::<String>,
            self.disable_confirm,
            None,
        ));

        self.action_row(components)
    }
}
