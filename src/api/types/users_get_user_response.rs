pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetUserResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<User>,
}

impl GetUserResponse {
    pub fn builder() -> GetUserResponseBuilder {
        <GetUserResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetUserResponseBuilder {
    data: Option<User>,
}

impl GetUserResponseBuilder {
    pub fn data(mut self, value: User) -> Self {
        self.data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetUserResponse`].
    pub fn build(self) -> Result<GetUserResponse, BuildError> {
        Ok(GetUserResponse { data: self.data })
    }
}
