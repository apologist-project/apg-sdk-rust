pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ScrubUserResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<UserRedactResponse>,
}

impl ScrubUserResponse {
    pub fn builder() -> ScrubUserResponseBuilder {
        <ScrubUserResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ScrubUserResponseBuilder {
    data: Option<UserRedactResponse>,
}

impl ScrubUserResponseBuilder {
    pub fn data(mut self, value: UserRedactResponse) -> Self {
        self.data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ScrubUserResponse`].
    pub fn build(self) -> Result<ScrubUserResponse, BuildError> {
        Ok(ScrubUserResponse { data: self.data })
    }
}
