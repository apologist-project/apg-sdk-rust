pub use crate::prelude::*;

/// Query parameters for listConversations
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListConversationsQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    /// Results per page (clamped to 100).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<i64>,
}

impl ListConversationsQueryRequest {
    pub fn builder() -> ListConversationsQueryRequestBuilder {
        <ListConversationsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListConversationsQueryRequestBuilder {
    page: Option<i64>,
    per_page: Option<i64>,
}

impl ListConversationsQueryRequestBuilder {
    pub fn page(mut self, value: i64) -> Self {
        self.page = Some(value);
        self
    }

    pub fn per_page(mut self, value: i64) -> Self {
        self.per_page = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListConversationsQueryRequest`].
    pub fn build(self) -> Result<ListConversationsQueryRequest, BuildError> {
        Ok(ListConversationsQueryRequest {
            page: self.page,
            per_page: self.per_page,
        })
    }
}
