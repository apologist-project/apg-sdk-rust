pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum BenchmarkRunRequestContentOneItem {
    String(String),

    ChatMessage(ChatMessage),
}

impl BenchmarkRunRequestContentOneItem {
    pub fn is_string(&self) -> bool {
        matches!(self, Self::String(_))
    }

    pub fn is_chat_message(&self) -> bool {
        matches!(self, Self::ChatMessage(_))
    }

    pub fn as_string(&self) -> Option<&str> {
        match self {
            Self::String(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_string(self) -> Option<String> {
        match self {
            Self::String(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_chat_message(&self) -> Option<&ChatMessage> {
        match self {
            Self::ChatMessage(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_chat_message(self) -> Option<ChatMessage> {
        match self {
            Self::ChatMessage(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for BenchmarkRunRequestContentOneItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::String(value) => write!(f, "{}", value),
            Self::ChatMessage(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}
