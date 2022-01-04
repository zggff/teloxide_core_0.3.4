use mime::Mime;
use serde::{Deserialize, Serialize};

/// This object represents a voice note.
///
/// [The official docs](https://core.telegram.org/bots/api#voice).
#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct Voice {
    /// Identifier for this file.
    pub file_id: String,

    /// Unique identifier for this file, which is supposed to be the same over
    /// time and for different bots. Can't be used to download or reuse the
    /// file.
    pub file_unique_id: String,

    /// Duration of the audio in seconds as defined by sender.
    pub duration: u32,

    /// MIME type of the file as defined by sender.
    #[serde(with = "crate::types::non_telegram_types::mime::opt_deser")]
    pub mime_type: Option<Mime>,

    /// File size.
    pub file_size: Option<u64>,
}
