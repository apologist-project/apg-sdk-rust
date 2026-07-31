pub use crate::prelude::*;

/// Query parameters for listUsers
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListUsersQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    /// Results per page (clamped to 100).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    /// Comma-separated tag ids.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub responder_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_timestamp: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_timestamp: Option<String>,
}

impl ListUsersQueryRequest {
    pub fn builder() -> ListUsersQueryRequestBuilder {
        <ListUsersQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListUsersQueryRequestBuilder {
    page: Option<i64>,
    per_page: Option<i64>,
    external_id: Option<String>,
    tags: Option<String>,
    responder_id: Option<String>,
    min_timestamp: Option<String>,
    max_timestamp: Option<String>,
}

impl ListUsersQueryRequestBuilder {
    pub fn page(mut self, value: i64) -> Self {
        self.page = Some(value);
        self
    }

    pub fn per_page(mut self, value: i64) -> Self {
        self.per_page = Some(value);
        self
    }

    pub fn external_id(mut self, value: impl Into<String>) -> Self {
        self.external_id = Some(value.into());
        self
    }

    pub fn tags(mut self, value: impl Into<String>) -> Self {
        self.tags = Some(value.into());
        self
    }

    pub fn responder_id(mut self, value: impl Into<String>) -> Self {
        self.responder_id = Some(value.into());
        self
    }

    pub fn min_timestamp(mut self, value: impl Into<String>) -> Self {
        self.min_timestamp = Some(value.into());
        self
    }

    pub fn max_timestamp(mut self, value: impl Into<String>) -> Self {
        self.max_timestamp = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListUsersQueryRequest`].
    pub fn build(self) -> Result<ListUsersQueryRequest, BuildError> {
        Ok(ListUsersQueryRequest {
            page: self.page,
            per_page: self.per_page,
            external_id: self.external_id,
            tags: self.tags,
            responder_id: self.responder_id,
            min_timestamp: self.min_timestamp,
            max_timestamp: self.max_timestamp,
        })
    }
}
