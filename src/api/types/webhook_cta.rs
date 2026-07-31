pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WebhookCta {
    #[serde(default)]
    pub id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}

impl WebhookCta {
    pub fn builder() -> WebhookCtaBuilder {
        <WebhookCtaBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WebhookCtaBuilder {
    id: Option<i64>,
    name: Option<String>,
    content: Option<String>,
}

impl WebhookCtaBuilder {
    pub fn id(mut self, value: i64) -> Self {
        self.id = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn content(mut self, value: impl Into<String>) -> Self {
        self.content = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WebhookCta`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](WebhookCtaBuilder::id)
    pub fn build(self) -> Result<WebhookCta, BuildError> {
        Ok(WebhookCta {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name,
            content: self.content,
        })
    }
}
