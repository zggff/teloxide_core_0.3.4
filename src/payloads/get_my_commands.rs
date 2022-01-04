// This file is auto generated by [`cg`] from [`schema`].
//
// **DO NOT EDIT THIS FILE**,
//
// Edit `cg` or `schema` instead.
//
// [cg]: https://github.com/teloxide/cg
// [`schema`]: https://github.com/WaffleLapkin/tg-methods-schema
use serde::Serialize;

use crate::types::{BotCommand, BotCommandScope};

impl_payload! {
    /// Use this method to get the current list of the bot's commands. Requires no parameters. Returns Array of [`BotCommand`] on success.
    ///
    /// [`BotCommand`]: crate::types::BotCommand
    #[derive(Debug, PartialEq, Eq, Hash, Default, Clone, Serialize)]
    pub GetMyCommands (GetMyCommandsSetters) => Vec<BotCommand> {

        optional {
            /// A JSON-serialized object, describing scope of users for which the commands are relevant. Defaults to BotCommandScopeDefault.
            pub scope: BotCommandScope,
            /// A two-letter ISO 639-1 language code. If empty, commands will be applied to all users from the given scope, for whose language there are no dedicated commands
            pub language_code: String [into],
        }
    }
}