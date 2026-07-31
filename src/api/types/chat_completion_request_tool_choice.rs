pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ChatCompletionRequestToolChoice {
    ChatCompletionRequestToolChoiceZero(ChatCompletionRequestToolChoiceZero),

    OptionalStringToValueMap(Option<HashMap<String, serde_json::Value>>),
}

impl ChatCompletionRequestToolChoice {
    pub fn is_chat_completion_request_tool_choice_zero(&self) -> bool {
        matches!(self, Self::ChatCompletionRequestToolChoiceZero(_))
    }

    pub fn is_optional_string_to_value_map(&self) -> bool {
        matches!(self, Self::OptionalStringToValueMap(_))
    }

    pub fn as_chat_completion_request_tool_choice_zero(
        &self,
    ) -> Option<&ChatCompletionRequestToolChoiceZero> {
        match self {
            Self::ChatCompletionRequestToolChoiceZero(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_chat_completion_request_tool_choice_zero(
        self,
    ) -> Option<ChatCompletionRequestToolChoiceZero> {
        match self {
            Self::ChatCompletionRequestToolChoiceZero(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_optional_string_to_value_map(&self) -> Option<&HashMap<String, serde_json::Value>> {
        match self {
            Self::OptionalStringToValueMap(value) => value.as_ref(),
            _ => None,
        }
    }

    pub fn into_optional_string_to_value_map(self) -> Option<HashMap<String, serde_json::Value>> {
        match self {
            Self::OptionalStringToValueMap(value) => value,
            _ => None,
        }
    }
}
