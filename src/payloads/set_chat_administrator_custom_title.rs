// This file is auto generated by [`cg`] from [`schema`].
//
// **DO NOT EDIT THIS FILE**,
//
// Edit `cg` or `schema` instead.
//
// [cg]: https://github.com/teloxide/cg
// [`schema`]: https://github.com/WaffleLapkin/tg-methods-schema
use serde::Serialize;

use crate::types::{ChatId, True};

impl_payload! {
    /// Use this method to set a custom title for an administrator in a supergroup promoted by the bot. Returns _True_on success.
    #[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize)]
    pub SetChatAdministratorCustomTitle (SetChatAdministratorCustomTitleSetters) => True {
        required {
            /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
            pub chat_id: ChatId [into],
            /// Unique identifier of the target user
            pub user_id: i64,
            /// New custom title for the administrator; 0-16 characters, emoji are not allowed
            pub custom_title: String [into],
        }
    }
}