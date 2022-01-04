// This file is auto generated by [`cg`] from [`schema`].
//
// **DO NOT EDIT THIS FILE**,
//
// Edit `cg` or `schema` instead.
//
// [cg]: https://github.com/teloxide/cg
// [`schema`]: https://github.com/WaffleLapkin/tg-methods-schema
use serde::Serialize;

use crate::types::{File, InputFile};

impl_payload! {
    @[multipart]
    /// Use this method to upload a .PNG file with a sticker for later use in _createNewStickerSet_ and _addStickerToSet_ methods (can be used multiple times). Returns the uploaded File on success.
    #[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize)]
    pub UploadStickerFile (UploadStickerFileSetters) => File {
        required {
            /// User identifier of sticker file owner
            pub user_id: i64,
            /// PNG image with the sticker, must be up to 512 kilobytes in size, dimensions must not exceed 512px, and either width or height must be exactly 512px. [More info on Sending Files »]
            ///
            /// [More info on Sending Files »]: crate::types::InputFile
            pub png_sticker: InputFile,
        }
    }
}
