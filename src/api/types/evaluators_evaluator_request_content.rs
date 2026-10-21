pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum EvaluatorRequestContent {
    String(String),

    EvaluatorRequestContentOneItemList(Vec<EvaluatorRequestContentOneItem>),
}

impl EvaluatorRequestContent {
    pub fn is_string(&self) -> bool {
        matches!(self, Self::String(_))
    }

    pub fn is_evaluator_request_content_one_item_list(&self) -> bool {
        matches!(self, Self::EvaluatorRequestContentOneItemList(_))
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

    pub fn as_evaluator_request_content_one_item_list(
        &self,
    ) -> Option<&Vec<EvaluatorRequestContentOneItem>> {
        match self {
            Self::EvaluatorRequestContentOneItemList(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_evaluator_request_content_one_item_list(
        self,
    ) -> Option<Vec<EvaluatorRequestContentOneItem>> {
        match self {
            Self::EvaluatorRequestContentOneItemList(value) => Some(value),
            _ => None,
        }
    }
}
