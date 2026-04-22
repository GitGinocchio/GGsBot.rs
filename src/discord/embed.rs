use twilight_model::channel::message::embed::{
    Embed, EmbedAuthor, EmbedField, EmbedFooter, EmbedImage, EmbedProvider, EmbedThumbnail, EmbedVideo
};
use crate::traits::color::IntoColor;

#[allow(unused)]
pub trait EmbedExt {
    fn new() -> Self;
    fn set_type(&mut self, kind: impl Into<String>) -> &mut Self;
    fn set_title(&mut self, title: impl Into<String>) -> &mut Self;
    fn set_description(&mut self, description: impl Into<String>) -> &mut Self;
    fn set_color(&mut self, color: impl IntoColor) -> &mut Self;
    fn add_field(&mut self, name: impl Into<String>, value: impl Into<String>, inline: bool) -> &mut Self;
    fn set_footer(&mut self, text: impl Into<String>, icon_url: Option<String>) -> &mut Self;
    fn set_provider(&mut self, name: impl Into<String>, url: Option<String>) -> &mut Self;
    fn set_thumbnail(&mut self, url: impl Into<String>) -> &mut Self;
    fn set_author(&mut self, name: impl Into<String>, icon_url: Option<String>, url: Option<String>) -> &mut Self;
    fn set_url(&mut self, url: impl Into<String>) -> &mut Self;
    fn set_image(&mut self, url: impl Into<String>) -> &mut Self;
    fn set_video(&mut self, url: impl Into<String>) -> &mut Self;
    fn set_timestamp(&mut self, timestamp: twilight_model::util::Timestamp) -> &mut Self;
}

impl EmbedExt for Embed {
    fn new() -> Self {
        Self {
            author: None,
            color: None,
            description: None,
            fields: Vec::new(),
            footer: None,
            image: None,
            kind: "rich".into(),
            provider: None,
            thumbnail: None,
            timestamp: None,
            title: None,
            url: None,
            video: None,
        }
    }

    fn set_type(&mut self, kind: impl Into<String>) -> &mut Self {
        self.kind = kind.into();
        self
    }

    fn set_title(&mut self, title: impl Into<String>) -> &mut Self {
        self.title = Some(title.into());
        self
    }

    fn set_description(&mut self, description: impl Into<String>) -> &mut Self {
        self.description = Some(description.into());
        self
    }

    fn set_color(&mut self, color: impl IntoColor) -> &mut Self {
        if let Some(c) = color.into_u32() {
            self.color = Some(c);
        }
        self
    }

    fn add_field(&mut self, name: impl Into<String>, value: impl Into<String>, inline: bool) -> &mut Self {
        self.fields.push(EmbedField {
            name: name.into(),
            value: value.into(),
            inline,
        });
        self
    }

    fn set_footer(&mut self, text: impl Into<String>, icon_url: Option<String>) -> &mut Self {
        self.footer = Some(EmbedFooter {
            text: text.into(),
            icon_url,
            proxy_icon_url: None,
        });
        self
    }

    fn set_thumbnail(&mut self, url: impl Into<String>) -> &mut Self {
        self.thumbnail = Some(EmbedThumbnail {
            url: url.into(),
            height: None,
            width: None,
            proxy_url: None,
        });
        self
    }

    fn set_author(&mut self, name: impl Into<String>, icon_url: Option<String>, url: Option<String>) -> &mut Self {
        self.author = Some(EmbedAuthor {
            name: name.into(),
            icon_url,
            url,
            proxy_icon_url: None,
        });
        self
    }

    fn set_url(&mut self, url: impl Into<String>) -> &mut Self {
        self.url = Some(url.into());
        self
    }

    fn set_image(&mut self, url: impl Into<String>) -> &mut Self {
        self.image = Some(EmbedImage {
            url: url.into(),
            height: None,
            width: None,
            proxy_url: None,
        });
        self
    }

    fn set_video(&mut self, url: impl Into<String>) -> &mut Self {
        self.video = Some(EmbedVideo {
            height: None,
            proxy_url: None,
            url: Some(url.into()),
            width: None
        });
        self
    }

    fn set_provider(&mut self, name: impl Into<String>, url: Option<String>) -> &mut Self {
        self.provider = Some(EmbedProvider {
            name: Some(name.into()),
            url: url
        });
        self
    }

    fn set_timestamp(&mut self, timestamp: twilight_model::util::Timestamp) -> &mut Self {
        self.timestamp = Some(timestamp);
        self
    }
}

#[derive(Clone)]
pub struct EmbedBuilder(Embed);

#[allow(unused)]
impl EmbedBuilder {
    pub fn new() -> Self {
        Self(Embed {
            author: None,
            color: None,
            description: None,
            fields: Vec::new(),
            footer: None,
            image: None,
            kind: "rich".into(),
            provider: None,
            thumbnail: None,
            timestamp: None,
            title: None,
            url: None,
            video: None,
        })
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.0.set_title(title);
        self
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.0.set_description(description);
        self
    }

    pub fn color(mut self, color: impl IntoColor) -> Self {
        self.0.set_color(color);
        self
    }

    pub fn field(mut self, name: impl Into<String>, value: impl Into<String>, inline: bool) -> Self {
        self.0.add_field(name, value, inline);
        self
    }

    pub fn footer(mut self, text: impl Into<String>, icon_url: Option<String>) -> Self {
        self.0.set_footer(text, icon_url);
        self
    }

    pub fn thumbnail(mut self, url: impl Into<String>) -> Self {
        self.0.set_thumbnail(url);
        self
    }

    pub fn author(mut self, name: impl Into<String>, icon_url: Option<String>, url: Option<String>) -> Self {
        self.0.set_author(name, icon_url, url);
        self
    }

    pub fn image(mut self, url: impl Into<String>) -> Self {
        self.0.set_image(url);
        self
    }

    pub fn timestamp(mut self, timestamp: twilight_model::util::Timestamp) -> Self {
        self.0.set_timestamp(timestamp);
        self
    }

    pub fn build(self) -> Embed {
        self.0
    }
}