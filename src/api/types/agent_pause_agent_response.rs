pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PauseAgentResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<AgentPauseState>,
}

impl PauseAgentResponse {
    pub fn builder() -> PauseAgentResponseBuilder {
        <PauseAgentResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PauseAgentResponseBuilder {
    data: Option<AgentPauseState>,
}

impl PauseAgentResponseBuilder {
    pub fn data(mut self, value: AgentPauseState) -> Self {
        self.data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PauseAgentResponse`].
    pub fn build(self) -> Result<PauseAgentResponse, BuildError> {
        Ok(PauseAgentResponse { data: self.data })
    }
}
