pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ShareRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

impl ShareRequest {
    pub fn builder() -> ShareRequestBuilder {
        <ShareRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ShareRequestBuilder {
    conversation_id: Option<String>,
    session_id: Option<String>,
    user_id: Option<String>,
}

impl ShareRequestBuilder {
    pub fn conversation_id(mut self, value: impl Into<String>) -> Self {
        self.conversation_id = Some(value.into());
        self
    }

    pub fn session_id(mut self, value: impl Into<String>) -> Self {
        self.session_id = Some(value.into());
        self
    }

    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ShareRequest`].
    pub fn build(self) -> Result<ShareRequest, BuildError> {
        Ok(ShareRequest {
            conversation_id: self.conversation_id,
            session_id: self.session_id,
            user_id: self.user_id,
        })
    }
}
