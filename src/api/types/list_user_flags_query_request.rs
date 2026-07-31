pub use crate::prelude::*;

/// Query parameters for listUserFlags
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListUserFlagsQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    /// Results per page (clamped to 100).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<i64>,
}

impl ListUserFlagsQueryRequest {
    pub fn builder() -> ListUserFlagsQueryRequestBuilder {
        <ListUserFlagsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListUserFlagsQueryRequestBuilder {
    page: Option<i64>,
    per_page: Option<i64>,
}

impl ListUserFlagsQueryRequestBuilder {
    pub fn page(mut self, value: i64) -> Self {
        self.page = Some(value);
        self
    }

    pub fn per_page(mut self, value: i64) -> Self {
        self.per_page = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListUserFlagsQueryRequest`].
    pub fn build(self) -> Result<ListUserFlagsQueryRequest, BuildError> {
        Ok(ListUserFlagsQueryRequest {
            page: self.page,
            per_page: self.per_page,
        })
    }
}
