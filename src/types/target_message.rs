use crate::types::ChatId;

use serde::{Deserialize, Serialize};

/// A message in chat or inline message.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TargetMessage {
    Common { chat_id: ChatId, message_id: i32 },
    Inline { inline_message_id: String },
}

impl From<String> for TargetMessage {
    fn from(inline_message_id: String) -> Self {
        Self::Inline { inline_message_id }
    }
}
