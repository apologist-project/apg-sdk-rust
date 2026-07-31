pub use crate::prelude::*;

/// Stable machine-readable event key.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum WebhookEventInfoKey {
    PromptSubmit,
    ResponseStart,
    ResponseEnd,
    AutomationsEnd,
    ResponseLike,
    ResponseFlag,
    ResponseFeedback,
    ReferralClick,
    CtaTrigger,
    CtaClick,
    GuardrailTrigger,
    AttributionClick,
    FooterClick,
    NewUser,
    NewDevice,
    NewSession,
    NewConversation,
    Error,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for WebhookEventInfoKey {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::PromptSubmit => serializer.serialize_str("prompt_submit"),
            Self::ResponseStart => serializer.serialize_str("response_start"),
            Self::ResponseEnd => serializer.serialize_str("response_end"),
            Self::AutomationsEnd => serializer.serialize_str("automations_end"),
            Self::ResponseLike => serializer.serialize_str("response_like"),
            Self::ResponseFlag => serializer.serialize_str("response_flag"),
            Self::ResponseFeedback => serializer.serialize_str("response_feedback"),
            Self::ReferralClick => serializer.serialize_str("referral_click"),
            Self::CtaTrigger => serializer.serialize_str("cta_trigger"),
            Self::CtaClick => serializer.serialize_str("cta_click"),
            Self::GuardrailTrigger => serializer.serialize_str("guardrail_trigger"),
            Self::AttributionClick => serializer.serialize_str("attribution_click"),
            Self::FooterClick => serializer.serialize_str("footer_click"),
            Self::NewUser => serializer.serialize_str("new_user"),
            Self::NewDevice => serializer.serialize_str("new_device"),
            Self::NewSession => serializer.serialize_str("new_session"),
            Self::NewConversation => serializer.serialize_str("new_conversation"),
            Self::Error => serializer.serialize_str("error"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for WebhookEventInfoKey {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "prompt_submit" => Ok(Self::PromptSubmit),
            "response_start" => Ok(Self::ResponseStart),
            "response_end" => Ok(Self::ResponseEnd),
            "automations_end" => Ok(Self::AutomationsEnd),
            "response_like" => Ok(Self::ResponseLike),
            "response_flag" => Ok(Self::ResponseFlag),
            "response_feedback" => Ok(Self::ResponseFeedback),
            "referral_click" => Ok(Self::ReferralClick),
            "cta_trigger" => Ok(Self::CtaTrigger),
            "cta_click" => Ok(Self::CtaClick),
            "guardrail_trigger" => Ok(Self::GuardrailTrigger),
            "attribution_click" => Ok(Self::AttributionClick),
            "footer_click" => Ok(Self::FooterClick),
            "new_user" => Ok(Self::NewUser),
            "new_device" => Ok(Self::NewDevice),
            "new_session" => Ok(Self::NewSession),
            "new_conversation" => Ok(Self::NewConversation),
            "error" => Ok(Self::Error),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for WebhookEventInfoKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PromptSubmit => write!(f, "prompt_submit"),
            Self::ResponseStart => write!(f, "response_start"),
            Self::ResponseEnd => write!(f, "response_end"),
            Self::AutomationsEnd => write!(f, "automations_end"),
            Self::ResponseLike => write!(f, "response_like"),
            Self::ResponseFlag => write!(f, "response_flag"),
            Self::ResponseFeedback => write!(f, "response_feedback"),
            Self::ReferralClick => write!(f, "referral_click"),
            Self::CtaTrigger => write!(f, "cta_trigger"),
            Self::CtaClick => write!(f, "cta_click"),
            Self::GuardrailTrigger => write!(f, "guardrail_trigger"),
            Self::AttributionClick => write!(f, "attribution_click"),
            Self::FooterClick => write!(f, "footer_click"),
            Self::NewUser => write!(f, "new_user"),
            Self::NewDevice => write!(f, "new_device"),
            Self::NewSession => write!(f, "new_session"),
            Self::NewConversation => write!(f, "new_conversation"),
            Self::Error => write!(f, "error"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
