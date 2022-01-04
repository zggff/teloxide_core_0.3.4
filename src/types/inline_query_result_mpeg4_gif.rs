use serde::{Deserialize, Serialize};

use crate::types::{InlineKeyboardMarkup, InputMessageContent, MessageEntity, ParseMode};

/// Represents a link to a video animation (H.264/MPEG-4 AVC video without
/// sound).
///
/// By default, this animated MPEG-4 file will be sent by the user with optional
/// caption. Alternatively, you can use `input_message_content` to send
/// a message with the specified content instead of the animation.
///
/// [The official docs](https://core.telegram.org/bots/api#inlinequeryresultmpeg4gif).
#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InlineQueryResultMpeg4Gif {
    /// Unique identifier for this result, 1-64 bytes.
    pub id: String,

    /// A valid URL for the MP4 file. File size must not exceed 1MB.
    pub mpeg4_url: String,

    /// Video width.
    pub mpeg4_width: Option<i32>,

    /// Video height.
    pub mpeg4_height: Option<i32>,

    /// Video duration.
    pub mpeg4_duration: Option<i32>,

    /// URL of the static thumbnail (jpeg or gif) for the result.
    pub thumb_url: String,

    /// Title for the result.
    pub title: Option<String>,

    /// Caption of the MPEG-4 file to be sent, 0-1024 characters.
    pub caption: Option<String>,

    /// Send [Markdown] or [HTML], if you want Telegram apps to show [bold,
    /// italic, fixed-width text or inline URLs] in the media caption.
    ///
    /// [Markdown]: https://core.telegram.org/bots/api#markdown-style
    /// [HTML]: https://core.telegram.org/bots/api#html-style
    /// [bold, italic, fixed-width text or inline URLs]: https://core.telegram.org/bots/api#formatting-options
    pub parse_mode: Option<ParseMode>,

    /// List of special entities that appear in the caption, which can be
    /// specified instead of `parse_mode`.
    pub caption_entities: Option<Vec<MessageEntity>>,

    /// [Inline keyboard] attached to the message.
    ///
    /// [Inline keyboard]: https://core.telegram.org/bots#inline-keyboards-and-on-the-fly-updating
    pub reply_markup: Option<InlineKeyboardMarkup>,

    /// Content of the message to be sent instead of the video animation.
    pub input_message_content: Option<InputMessageContent>,
}

impl InlineQueryResultMpeg4Gif {
    pub fn new<S1, S2, S3>(id: S1, mpeg4_url: S2, thumb_url: S3) -> Self
    where
        S1: Into<String>,
        S2: Into<String>,
        S3: Into<String>,
    {
        Self {
            id: id.into(),
            mpeg4_url: mpeg4_url.into(),
            thumb_url: thumb_url.into(),
            mpeg4_width: None,
            mpeg4_height: None,
            mpeg4_duration: None,
            title: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            reply_markup: None,
            input_message_content: None,
        }
    }

    pub fn id<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.id = val.into();
        self
    }

    pub fn mpeg4_url<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.mpeg4_url = val.into();
        self
    }

    pub fn mpeg4_width(mut self, val: i32) -> Self {
        self.mpeg4_width = Some(val);
        self
    }

    pub fn mpeg4_height(mut self, val: i32) -> Self {
        self.mpeg4_height = Some(val);
        self
    }

    pub fn mpeg4_duration(mut self, val: i32) -> Self {
        self.mpeg4_duration = Some(val);
        self
    }

    pub fn thumb_url<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.thumb_url = val.into();
        self
    }

    pub fn title<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.title = Some(val.into());
        self
    }

    pub fn caption<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.caption = Some(val.into());
        self
    }

    pub fn parse_mode<S>(mut self, val: ParseMode) -> Self {
        self.parse_mode = Some(val);
        self
    }

    pub fn caption_entities<C>(mut self, val: C) -> Self
    where
        C: IntoIterator<Item = MessageEntity>,
    {
        self.caption_entities = Some(val.into_iter().collect());
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
}
