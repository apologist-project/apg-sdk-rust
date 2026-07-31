pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListUsersResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<User>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<i64>,
}

impl ListUsersResponse {
    pub fn builder() -> ListUsersResponseBuilder {
        <ListUsersResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListUsersResponseBuilder {
    data: Option<Vec<User>>,
    total: Option<i64>,
    page: Option<i64>,
    per_page: Option<i64>,
}

impl ListUsersResponseBuilder {
    pub fn data(mut self, value: Vec<User>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn total(mut self, value: i64) -> Self {
        self.total = Some(value);
        self
    }

    pub fn page(mut self, value: i64) -> Self {
        self.page = Some(value);
        self
    }

    pub fn per_page(mut self, value: i64) -> Self {
        self.per_page = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListUsersResponse`].
    pub fn build(self) -> Result<ListUsersResponse, BuildError> {
        Ok(ListUsersResponse {
            data: self.data,
            total: self.total,
            page: self.page,
            per_page: self.per_page,
        })
    }
}
