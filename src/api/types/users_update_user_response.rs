pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateUserResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<User>,
}

impl UpdateUserResponse {
    pub fn builder() -> UpdateUserResponseBuilder {
        <UpdateUserResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateUserResponseBuilder {
    data: Option<User>,
}

impl UpdateUserResponseBuilder {
    pub fn data(mut self, value: User) -> Self {
        self.data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateUserResponse`].
    pub fn build(self) -> Result<UpdateUserResponse, BuildError> {
        Ok(UpdateUserResponse { data: self.data })
    }
}
