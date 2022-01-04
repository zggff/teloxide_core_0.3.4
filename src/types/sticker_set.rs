use serde::{Deserialize, Serialize};

use crate::types::{PhotoSize, Sticker};

/// This object represents a sticker set.
///
/// [The official docs](https://core.telegram.org/bots/api#stickerset).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StickerSet {
    /// Sticker set name.
    pub name: String,

    /// Sticker set title.
    pub title: String,

    /// `true`, if the sticker set contains [animated stickers].
    ///
    /// [animates stickers]: https://telegram.org/blog/animated-stickers
    pub is_animated: bool,

    /// `true`, if the sticker set contains masks.
    pub contains_masks: bool,

    /// List of all set stickers.
    pub stickers: Vec<Sticker>,

    /// Sticker set thumbnail in the .WEBP or .TGS format.
    pub thumb: Option<PhotoSize>,
}
