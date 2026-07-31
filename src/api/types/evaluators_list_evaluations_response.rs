pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListEvaluationsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<HashMap<String, serde_json::Value>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<i64>,
}

impl ListEvaluationsResponse {
    pub fn builder() -> ListEvaluationsResponseBuilder {
        <ListEvaluationsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListEvaluationsResponseBuilder {
    data: Option<Vec<HashMap<String, serde_json::Value>>>,
    total: Option<i64>,
    page: Option<i64>,
    per_page: Option<i64>,
}

impl ListEvaluationsResponseBuilder {
    pub fn data(mut self, value: Vec<HashMap<String, serde_json::Value>>) -> Self {
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

    /// Consumes the builder and constructs a [`ListEvaluationsResponse`].
    pub fn build(self) -> Result<ListEvaluationsResponse, BuildError> {
        Ok(ListEvaluationsResponse {
            data: self.data,
            total: self.total,
            page: self.page,
            per_page: self.per_page,
        })
    }
}
