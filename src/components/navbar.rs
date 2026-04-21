use async_trait::async_trait;
use twilight_model::channel::message::{Component, component::ButtonStyle};

use crate::discord::component::CustomComponent;

pub struct NavBar {
    parent_id: String,
    page: u8,
    max_page: Option<u8>
}

#[allow(unused)]
impl NavBar {
    pub fn new(parent_id: impl Into<String>, start_page: Option<u8>, max_page: Option<u8>) -> Self {
        Self {
            parent_id: parent_id.into(),
            page: start_page.unwrap_or(0),
            max_page: max_page
        }
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
impl CustomComponent for NavBar {
    fn id(&self) -> String {
        format!("{}:nav:{}", self.parent_id, self.page)
    }

    fn build(&self) -> Component {
        let mut components = Vec::new();

        components.push(self.button(
            "back", 
            Some("Back"), 
            None, 
            ButtonStyle::Secondary,
            None::<String>, 
            if self.page == 0 { true } else { false }, 
            None
        ));

        components.push(self.button(
            "cancel", 
            Some("Cancel"), 
            None, 
            ButtonStyle::Danger,
            None::<String>, 
            false, 
            None
        ));

        components.push(self.button(
            "next", 
            Some("Next"), 
            None, 
            ButtonStyle::Primary,
            None::<String>, 
            if let Some(max) = self.max_page && self.page >= max { true } else { false }, 
            None
        ));

        self.action_row(components)
    }
}