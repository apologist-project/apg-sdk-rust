pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LikeRequest {
    #[serde(default)]
    pub liked: bool,
}

impl LikeRequest {
    pub fn builder() -> LikeRequestBuilder {
        <LikeRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LikeRequestBuilder {
    liked: Option<bool>,
}

impl LikeRequestBuilder {
    pub fn liked(mut self, value: bool) -> Self {
        self.liked = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LikeRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`liked`](LikeRequestBuilder::liked)
    pub fn build(self) -> Result<LikeRequest, BuildError> {
        Ok(LikeRequest {
            liked: self
                .liked
                .ok_or_else(|| BuildError::missing_field("liked"))?,
        })
    }
}
