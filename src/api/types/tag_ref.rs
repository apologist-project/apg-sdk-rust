pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TagRef {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl TagRef {
    pub fn builder() -> TagRefBuilder {
        <TagRefBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TagRefBuilder {
    id: Option<i64>,
    name: Option<String>,
}

impl TagRefBuilder {
    pub fn id(mut self, value: i64) -> Self {
        self.id = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TagRef`].
    pub fn build(self) -> Result<TagRef, BuildError> {
        Ok(TagRef {
            id: self.id,
            name: self.name,
        })
    }
}
