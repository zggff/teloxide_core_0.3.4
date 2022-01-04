use serde::{Deserialize, Serialize};

use crate::types::{InlineKeyboardMarkup, InputMessageContent};

/// Represents a link to an article or web page.
///
/// [The official docs](https://core.telegram.org/bots/api#inlinequeryresultarticle).
#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InlineQueryResultArticle {
    /// Unique identifier for this result, 1-64 Bytes.
    pub id: String,

    /// Title of the result.
    pub title: String,

    /// Content of the message to be sent.
    pub input_message_content: InputMessageContent,

    /// Inline keyboard attached to the message.
    pub reply_markup: Option<InlineKeyboardMarkup>,

    /// URL of the result.
    pub url: Option<String>,

    /// Pass `true`, if you don't want the URL to be shown in the
    /// message.
    pub hide_url: Option<bool>,

    /// Short description of the result.
    pub description: Option<String>,

    /// Url of the thumbnail for the result.
    pub thumb_url: Option<String>,

    /// Thumbnail width.
    pub thumb_width: Option<i32>,

    /// Thumbnail height.
    pub thumb_height: Option<i32>,
}

impl InlineQueryResultArticle {
    pub fn new<S1, S2>(id: S1, title: S2, input_message_content: InputMessageContent) -> Self
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        Self {
            id: id.into(),
            title: title.into(),
            input_message_content,
            reply_markup: None,
            url: None,
            hide_url: None,
            description: None,
            thumb_url: None,
            thumb_width: None,
            thumb_height: None,
        }
    }

    pub fn id<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.id = val.into();
        self
    }

    pub fn title<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.title = val.into();
        self
    }

    pub fn input_message_content(mut self, val: InputMessageContent) -> Self {
        self.input_message_content = val;
        self
    }

    pub fn reply_markup(mut self, val: InlineKeyboardMarkup) -> Self {
        self.reply_markup = Some(val);
        self
    }

    pub fn url<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.url = Some(val.into());
        self
    }

    pub fn hide_url(mut self, val: bool) -> Self {
        self.hide_url = Some(val);
        self
    }

    pub fn description<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.description = Some(val.into());
        self
    }

    pub fn thumb_url<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.thumb_url = Some(val.into());
        self
    }

    pub fn thumb_width(mut self, val: i32) -> Self {
        self.thumb_width = Some(val);
        self
    }

    pub fn thumb_height(mut self, val: i32) -> Self {
        self.thumb_height = Some(val);
        self
    }
}
