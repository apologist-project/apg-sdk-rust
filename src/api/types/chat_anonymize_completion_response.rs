pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AnonymizeCompletionResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<CompletionRedactResponse>,
}

impl AnonymizeCompletionResponse {
    pub fn builder() -> AnonymizeCompletionResponseBuilder {
        <AnonymizeCompletionResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AnonymizeCompletionResponseBuilder {
    data: Option<CompletionRedactResponse>,
}

impl AnonymizeCompletionResponseBuilder {
    pub fn data(mut self, value: CompletionRedactResponse) -> Self {
        self.data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AnonymizeCompletionResponse`].
    pub fn build(self) -> Result<AnonymizeCompletionResponse, BuildError> {
        Ok(AnonymizeCompletionResponse { data: self.data })
    }
}
