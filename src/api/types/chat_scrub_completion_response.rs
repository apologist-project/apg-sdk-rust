pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ScrubCompletionResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<CompletionRedactResponse>,
}

impl ScrubCompletionResponse {
    pub fn builder() -> ScrubCompletionResponseBuilder {
        <ScrubCompletionResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ScrubCompletionResponseBuilder {
    data: Option<CompletionRedactResponse>,
}

impl ScrubCompletionResponseBuilder {
    pub fn data(mut self, value: CompletionRedactResponse) -> Self {
        self.data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ScrubCompletionResponse`].
    pub fn build(self) -> Result<ScrubCompletionResponse, BuildError> {
        Ok(ScrubCompletionResponse { data: self.data })
    }
}
