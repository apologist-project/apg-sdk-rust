pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ResumeAgentResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<AgentPauseState>,
}

impl ResumeAgentResponse {
    pub fn builder() -> ResumeAgentResponseBuilder {
        <ResumeAgentResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ResumeAgentResponseBuilder {
    data: Option<AgentPauseState>,
}

impl ResumeAgentResponseBuilder {
    pub fn data(mut self, value: AgentPauseState) -> Self {
        self.data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ResumeAgentResponse`].
    pub fn build(self) -> Result<ResumeAgentResponse, BuildError> {
        Ok(ResumeAgentResponse { data: self.data })
    }
}
