pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct WebhookEventInfo {
    /// Stable machine-readable event key.
    pub key: WebhookEventInfoKey,
    /// Human-readable event label.
    #[serde(default)]
    pub label: String,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub occurred_at: DateTime<FixedOffset>,
}

impl WebhookEventInfo {
    pub fn builder() -> WebhookEventInfoBuilder {
        <WebhookEventInfoBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WebhookEventInfoBuilder {
    key: Option<WebhookEventInfoKey>,
    label: Option<String>,
    occurred_at: Option<DateTime<FixedOffset>>,
}

impl WebhookEventInfoBuilder {
    pub fn key(mut self, value: WebhookEventInfoKey) -> Self {
        self.key = Some(value);
        self
    }

    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    pub fn occurred_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.occurred_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WebhookEventInfo`].
    /// This method will fail if any of the following fields are not set:
    /// - [`key`](WebhookEventInfoBuilder::key)
    /// - [`label`](WebhookEventInfoBuilder::label)
    /// - [`occurred_at`](WebhookEventInfoBuilder::occurred_at)
    pub fn build(self) -> Result<WebhookEventInfo, BuildError> {
        Ok(WebhookEventInfo {
            key: self.key.ok_or_else(|| BuildError::missing_field("key"))?,
            label: self
                .label
                .ok_or_else(|| BuildError::missing_field("label"))?,
            occurred_at: self
                .occurred_at
                .ok_or_else(|| BuildError::missing_field("occurred_at"))?,
        })
    }
}
