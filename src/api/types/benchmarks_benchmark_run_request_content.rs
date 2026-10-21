pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum BenchmarkRunRequestContent {
    OptionalString(Option<String>),

    OptionalBenchmarkRunRequestContentOneItemList(Option<Vec<BenchmarkRunRequestContentOneItem>>),
}

impl BenchmarkRunRequestContent {
    pub fn is_optional_string(&self) -> bool {
        matches!(self, Self::OptionalString(_))
    }

    pub fn is_optional_benchmark_run_request_content_one_item_list(&self) -> bool {
        matches!(self, Self::OptionalBenchmarkRunRequestContentOneItemList(_))
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

    pub fn as_optional_benchmark_run_request_content_one_item_list(
        &self,
    ) -> Option<&Vec<BenchmarkRunRequestContentOneItem>> {
        match self {
            Self::OptionalBenchmarkRunRequestContentOneItemList(value) => value.as_ref(),
            _ => None,
        }
    }

    pub fn into_optional_benchmark_run_request_content_one_item_list(
        self,
    ) -> Option<Vec<BenchmarkRunRequestContentOneItem>> {
        match self {
            Self::OptionalBenchmarkRunRequestContentOneItemList(value) => value,
            _ => None,
        }
    }
}
