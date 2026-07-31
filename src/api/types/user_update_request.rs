pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UserUpdateRequest {
    /// Your external identifier for the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    /// Applied tags as a mix of existing tag ids and/or default-language tag names. Unknown ids or names are rejected. Tags are mirror-owned and never created here.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<UserUpdateRequestTagsItem>>,
    /// Responder to persist for this user on the requesting agent. Must be active on the agent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub responder_id: Option<i64>,
}

impl UserUpdateRequest {
    pub fn builder() -> UserUpdateRequestBuilder {
        <UserUpdateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UserUpdateRequestBuilder {
    external_id: Option<String>,
    tags: Option<Vec<UserUpdateRequestTagsItem>>,
    responder_id: Option<i64>,
}

impl UserUpdateRequestBuilder {
    pub fn external_id(mut self, value: impl Into<String>) -> Self {
        self.external_id = Some(value.into());
        self
    }

    pub fn tags(mut self, value: Vec<UserUpdateRequestTagsItem>) -> Self {
        self.tags = Some(value);
        self
    }

    pub fn responder_id(mut self, value: i64) -> Self {
        self.responder_id = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UserUpdateRequest`].
    pub fn build(self) -> Result<UserUpdateRequest, BuildError> {
        Ok(UserUpdateRequest {
            external_id: self.external_id,
            tags: self.tags,
            responder_id: self.responder_id,
        })
    }
}
