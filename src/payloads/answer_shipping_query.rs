// This file is auto generated by [`cg`] from [`schema`].
//
// **DO NOT EDIT THIS FILE**,
//
// Edit `cg` or `schema` instead.
//
// [cg]: https://github.com/teloxide/cg
// [`schema`]: https://github.com/WaffleLapkin/tg-methods-schema
use serde::Serialize;

use crate::types::{ShippingOption, True};

impl_payload! {
    /// If you sent an invoice requesting a shipping address and the parameter _is\_flexible_ was specified, the Bot API will send an [`Update`] with a shipping_query field to the bot. Use this method to reply to shipping queries. On success, True is returned.
    ///
    /// [`Update`]: crate::types::Update
    #[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize)]
    pub AnswerShippingQuery (AnswerShippingQuerySetters) => True {
        required {
            /// Unique identifier for the query to be answered
            pub shipping_query_id: String [into],
            /// Specify True if delivery to the specified address is possible and False if there are any problems (for example, if delivery to the specified address is not possible)
            pub ok: bool,
        }
        optional {
            /// Required if ok is True. A JSON-serialized array of available shipping options.
            pub shipping_options: Vec<ShippingOption> [collect],
            /// Required if ok is False. Error message in human readable form that explains why it is impossible to complete the order (e.g. 'Sorry, delivery to your desired address is unavailable'). Telegram will display this message to the user.
            pub error_message: String [into],
        }
    }
}
