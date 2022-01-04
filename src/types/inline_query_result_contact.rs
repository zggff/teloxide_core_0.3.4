use serde::{Deserialize, Serialize};

use crate::types::{InlineKeyboardMarkup, InputMessageContent};

/// Represents a contact with a phone number.
///
/// By default, this contact will be sent by the user. Alternatively, you can
/// use `input_message_content` to send a message with the specified content
/// instead of the contact.
///
/// [The official docs](https://core.telegram.org/bots/api#inlinequeryresultcachedvideo).
#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InlineQueryResultContact {
    /// Unique identifier for this result, 1-64 Bytes.
    pub id: String,

    /// Contact's phone number.
    pub phone_number: String,

    /// Contact's first name.
    pub first_name: String,

    /// Contact's last name.
    pub last_name: Option<String>,

    /// Additional data about the contact in the form of a [vCard], 0-2048
    /// bytes.
    ///
    /// [VCard]: https://en.wikipedia.org/wiki/VCard
    pub vcard: Option<String>,

    /// [Inline keyboard] attached to the message.
    ///
    /// [Inline keyboard]: https://core.telegram.org/bots#inline-keyboards-and-on-the-fly-updating
    pub reply_markup: Option<InlineKeyboardMarkup>,

    /// Content of the message to be sent instead of the contact.
    pub input_message_content: Option<InputMessageContent>,

    /// Url of the thumbnail for the result.
    pub thumb_url: Option<String>,

    /// Thumbnail width.
    pub thumb_width: Option<i32>,

    /// Thumbnail height.
    pub thumb_height: Option<i32>,
}

impl InlineQueryResultContact {
    pub fn new<S1, S2, S3>(id: S1, phone_number: S2, first_name: S3) -> Self
    where
        S1: Into<String>,
        S2: Into<String>,
        S3: Into<String>,
    {
        Self {
            id: id.into(),
            phone_number: phone_number.into(),
            first_name: first_name.into(),
            last_name: None,
            vcard: None,
            reply_markup: None,
            input_message_content: None,
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

    pub fn phone_number<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.phone_number = val.into();
        self
    }

    pub fn first_name<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.first_name = val.into();
        self
    }

    pub fn last_name<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.last_name = Some(val.into());
        self
    }

    pub fn vcard<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.vcard = Some(val.into());
        self
    }

    pub fn reply_markup(mut self, val: InlineKeyboardMarkup) -> Self {
        self.reply_markup = Some(val);
        self
    }

    pub fn input_message_content(mut self, val: InputMessageContent) -> Self {
        self.input_message_content = Some(val);
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
