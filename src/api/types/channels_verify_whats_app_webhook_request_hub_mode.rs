pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum VerifyWhatsAppWebhookRequestHubMode {
    #[serde(rename = "subscribe")]
    Subscribe,
}
impl fmt::Display for VerifyWhatsAppWebhookRequestHubMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Subscribe => "subscribe",
        };
        write!(f, "{}", s)
    }
}
