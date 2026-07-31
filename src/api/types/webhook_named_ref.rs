pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WebhookNamedRef {
    #[serde(default)]
    pub id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl WebhookNamedRef {
    pub fn builder() -> WebhookNamedRefBuilder {
        <WebhookNamedRefBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WebhookNamedRefBuilder {
    id: Option<i64>,
    name: Option<String>,
}

impl WebhookNamedRefBuilder {
    pub fn id(mut self, value: i64) -> Self {
        self.id = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WebhookNamedRef`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](WebhookNamedRefBuilder::id)
    pub fn build(self) -> Result<WebhookNamedRef, BuildError> {
        Ok(WebhookNamedRef {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name,
        })
    }
}
