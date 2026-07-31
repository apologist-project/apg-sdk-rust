pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WebhookAgentRef {
    #[serde(default)]
    pub id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl WebhookAgentRef {
    pub fn builder() -> WebhookAgentRefBuilder {
        <WebhookAgentRefBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WebhookAgentRefBuilder {
    id: Option<i64>,
    name: Option<String>,
}

impl WebhookAgentRefBuilder {
    pub fn id(mut self, value: i64) -> Self {
        self.id = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WebhookAgentRef`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](WebhookAgentRefBuilder::id)
    pub fn build(self) -> Result<WebhookAgentRef, BuildError> {
        Ok(WebhookAgentRef {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name,
        })
    }
}
