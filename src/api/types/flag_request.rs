pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct FlagRequest {
    #[serde(default)]
    pub flagged: bool,
}

impl FlagRequest {
    pub fn builder() -> FlagRequestBuilder {
        <FlagRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FlagRequestBuilder {
    flagged: Option<bool>,
}

impl FlagRequestBuilder {
    pub fn flagged(mut self, value: bool) -> Self {
        self.flagged = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`FlagRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`flagged`](FlagRequestBuilder::flagged)
    pub fn build(self) -> Result<FlagRequest, BuildError> {
        Ok(FlagRequest {
            flagged: self
                .flagged
                .ok_or_else(|| BuildError::missing_field("flagged"))?,
        })
    }
}
