pub use crate::prelude::*;

/// Canonical JSON body POSTed to a configured webhook URL. `notification`, `event`, and `agent` are always present; the remaining sections appear only when relevant to the event. Treat the payload as additive and ignore unrecognised fields.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WebhookPayload {
    #[serde(default)]
    pub notification: WebhookNotificationRef,
    pub event: WebhookEventInfo,
    #[serde(default)]
    pub agent: WebhookAgentRef,
    /// Present when the event is tied to a prompt. Includes the prompt and response plus `automations` and `tags` arrays. Shape mirrors the prompt API object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion: Option<HashMap<String, serde_json::Value>>,
    /// Present when the prompt arrived via a channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<WebhookNamedRef>,
    /// Present alongside `channel` when the channel has a platform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<WebhookNamedRef>,
    /// Present for cta_trigger and cta_click events.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cta: Option<WebhookCta>,
    /// Present for guardrail_trigger events.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guardrail: Option<WebhookNamedRef>,
    /// Present for CTA/guardrail events that ran an evaluation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluator: Option<WebhookNamedRef>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation: Option<WebhookEvaluation>,
}

impl WebhookPayload {
    pub fn builder() -> WebhookPayloadBuilder {
        <WebhookPayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WebhookPayloadBuilder {
    notification: Option<WebhookNotificationRef>,
    event: Option<WebhookEventInfo>,
    agent: Option<WebhookAgentRef>,
    completion: Option<HashMap<String, serde_json::Value>>,
    channel: Option<WebhookNamedRef>,
    platform: Option<WebhookNamedRef>,
    cta: Option<WebhookCta>,
    guardrail: Option<WebhookNamedRef>,
    evaluator: Option<WebhookNamedRef>,
    evaluation: Option<WebhookEvaluation>,
}

impl WebhookPayloadBuilder {
    pub fn notification(mut self, value: WebhookNotificationRef) -> Self {
        self.notification = Some(value);
        self
    }

    pub fn event(mut self, value: WebhookEventInfo) -> Self {
        self.event = Some(value);
        self
    }

    pub fn agent(mut self, value: WebhookAgentRef) -> Self {
        self.agent = Some(value);
        self
    }

    pub fn completion(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.completion = Some(value);
        self
    }

    pub fn channel(mut self, value: WebhookNamedRef) -> Self {
        self.channel = Some(value);
        self
    }

    pub fn platform(mut self, value: WebhookNamedRef) -> Self {
        self.platform = Some(value);
        self
    }

    pub fn cta(mut self, value: WebhookCta) -> Self {
        self.cta = Some(value);
        self
    }

    pub fn guardrail(mut self, value: WebhookNamedRef) -> Self {
        self.guardrail = Some(value);
        self
    }

    pub fn evaluator(mut self, value: WebhookNamedRef) -> Self {
        self.evaluator = Some(value);
        self
    }

    pub fn evaluation(mut self, value: WebhookEvaluation) -> Self {
        self.evaluation = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WebhookPayload`].
    /// This method will fail if any of the following fields are not set:
    /// - [`notification`](WebhookPayloadBuilder::notification)
    /// - [`event`](WebhookPayloadBuilder::event)
    /// - [`agent`](WebhookPayloadBuilder::agent)
    pub fn build(self) -> Result<WebhookPayload, BuildError> {
        Ok(WebhookPayload {
            notification: self
                .notification
                .ok_or_else(|| BuildError::missing_field("notification"))?,
            event: self
                .event
                .ok_or_else(|| BuildError::missing_field("event"))?,
            agent: self
                .agent
                .ok_or_else(|| BuildError::missing_field("agent"))?,
            completion: self.completion,
            channel: self.channel,
            platform: self.platform,
            cta: self.cta,
            guardrail: self.guardrail,
            evaluator: self.evaluator,
            evaluation: self.evaluation,
        })
    }
}
