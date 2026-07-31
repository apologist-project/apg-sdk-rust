pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum BenchmarkRunRequestContent {
    OptionalString(Option<String>),

    OptionalValueList(Option<Vec<serde_json::Value>>),
}

impl BenchmarkRunRequestContent {
    pub fn is_optional_string(&self) -> bool {
        matches!(self, Self::OptionalString(_))
    }

    pub fn is_optional_value_list(&self) -> bool {
        matches!(self, Self::OptionalValueList(_))
    }

    pub fn as_optional_string(&self) -> Option<&str> {
        match self {
            Self::OptionalString(value) => value.as_deref(),
            _ => None,
        }
    }

    pub fn into_optional_string(self) -> Option<String> {
        match self {
            Self::OptionalString(value) => value,
            _ => None,
        }
    }

    pub fn as_optional_value_list(&self) -> Option<&Vec<serde_json::Value>> {
        match self {
            Self::OptionalValueList(value) => value.as_ref(),
            _ => None,
        }
    }

    pub fn into_optional_value_list(self) -> Option<Vec<serde_json::Value>> {
        match self {
            Self::OptionalValueList(value) => value,
            _ => None,
        }
    }
}
