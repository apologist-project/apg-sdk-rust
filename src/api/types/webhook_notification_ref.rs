pub use crate::prelude::*;

/// The notification configuration that produced this delivery.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WebhookNotificationRef {
    #[serde(default)]
    pub id: i64,
    #[serde(default)]
    pub name: String,
}

impl WebhookNotificationRef {
    pub fn builder() -> WebhookNotificationRefBuilder {
        <WebhookNotificationRefBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WebhookNotificationRefBuilder {
    id: Option<i64>,
    name: Option<String>,
}

impl WebhookNotificationRefBuilder {
    pub fn id(mut self, value: i64) -> Self {
        self.id = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WebhookNotificationRef`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](WebhookNotificationRefBuilder::id)
    /// - [`name`](WebhookNotificationRefBuilder::name)
    pub fn build(self) -> Result<WebhookNotificationRef, BuildError> {
        Ok(WebhookNotificationRef {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
