// This file is auto generated by [`cg`] from [`schema`].
//
// **DO NOT EDIT THIS FILE**,
//
// Edit `cg` or `schema` instead.
//
// [cg]: https://github.com/teloxide/cg
// [`schema`]: https://github.com/WaffleLapkin/tg-methods-schema
use serde::Serialize;

use crate::types::UserProfilePhotos;

impl_payload! {
    /// Use this method to get a list of profile pictures for a user. Returns a [`UserProfilePhotos`] object.
    ///
    /// [`UserProfilePhotos`]: crate::types::UserProfilePhotos
    #[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize)]
    pub GetUserProfilePhotos (GetUserProfilePhotosSetters) => UserProfilePhotos {
        required {
            /// Unique identifier of the target user
            pub user_id: i64,
        }
        optional {
            /// Sequential number of the first photo to be returned. By default, all photos are returned.
            pub offset: u32,
            /// Limits the number of photos to be retrieved. Values between 1-100 are accepted. Defaults to 100.
            pub limit: u8,
        }
    }
}
