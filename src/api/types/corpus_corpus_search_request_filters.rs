pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CorpusSearchRequestFilters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub types: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub languages: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_ids: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contributor_ids: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_ids: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classification_ids: Option<Vec<i64>>,
}

impl CorpusSearchRequestFilters {
    pub fn builder() -> CorpusSearchRequestFiltersBuilder {
        <CorpusSearchRequestFiltersBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CorpusSearchRequestFiltersBuilder {
    model: Option<String>,
    ids: Option<Vec<i64>>,
    types: Option<Vec<String>>,
    languages: Option<Vec<String>>,
    collection_ids: Option<Vec<i64>>,
    contributor_ids: Option<Vec<i64>>,
    category_ids: Option<Vec<i64>>,
    classification_ids: Option<Vec<i64>>,
}

impl CorpusSearchRequestFiltersBuilder {
    pub fn model(mut self, value: impl Into<String>) -> Self {
        self.model = Some(value.into());
        self
    }

    pub fn ids(mut self, value: Vec<i64>) -> Self {
        self.ids = Some(value);
        self
    }

    pub fn types(mut self, value: Vec<String>) -> Self {
        self.types = Some(value);
        self
    }

    pub fn languages(mut self, value: Vec<String>) -> Self {
        self.languages = Some(value);
        self
    }

    pub fn collection_ids(mut self, value: Vec<i64>) -> Self {
        self.collection_ids = Some(value);
        self
    }

    pub fn contributor_ids(mut self, value: Vec<i64>) -> Self {
        self.contributor_ids = Some(value);
        self
    }

    pub fn category_ids(mut self, value: Vec<i64>) -> Self {
        self.category_ids = Some(value);
        self
    }

    pub fn classification_ids(mut self, value: Vec<i64>) -> Self {
        self.classification_ids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CorpusSearchRequestFilters`].
    pub fn build(self) -> Result<CorpusSearchRequestFilters, BuildError> {
        Ok(CorpusSearchRequestFilters {
            model: self.model,
            ids: self.ids,
            types: self.types,
            languages: self.languages,
            collection_ids: self.collection_ids,
            contributor_ids: self.contributor_ids,
            category_ids: self.category_ids,
            classification_ids: self.classification_ids,
        })
    }
}
