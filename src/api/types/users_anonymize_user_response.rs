pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AnonymizeUserResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<UserRedactResponse>,
}

impl AnonymizeUserResponse {
    pub fn builder() -> AnonymizeUserResponseBuilder {
        <AnonymizeUserResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AnonymizeUserResponseBuilder {
    data: Option<UserRedactResponse>,
}

impl AnonymizeUserResponseBuilder {
    pub fn data(mut self, value: UserRedactResponse) -> Self {
        self.data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AnonymizeUserResponse`].
    pub fn build(self) -> Result<AnonymizeUserResponse, BuildError> {
        Ok(AnonymizeUserResponse { data: self.data })
    }
}
