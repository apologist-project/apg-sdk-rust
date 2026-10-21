pub use crate::prelude::*;

/// Required when type is json_schema. Supplies the JSON Schema the structured output must conform to. Structured outputs are non-streaming.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ChatCompletionRequestResponseFormatJsonSchema {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strict: Option<bool>,
}

impl ChatCompletionRequestResponseFormatJsonSchema {
    pub fn builder() -> ChatCompletionRequestResponseFormatJsonSchemaBuilder {
        <ChatCompletionRequestResponseFormatJsonSchemaBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ChatCompletionRequestResponseFormatJsonSchemaBuilder {
    name: Option<String>,
    description: Option<String>,
    schema: Option<HashMap<String, serde_json::Value>>,
    strict: Option<bool>,
}

impl ChatCompletionRequestResponseFormatJsonSchemaBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn schema(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.schema = Some(value);
        self
    }

    pub fn strict(mut self, value: bool) -> Self {
        self.strict = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ChatCompletionRequestResponseFormatJsonSchema`].
    pub fn build(self) -> Result<ChatCompletionRequestResponseFormatJsonSchema, BuildError> {
        Ok(ChatCompletionRequestResponseFormatJsonSchema {
            name: self.name,
            description: self.description,
            schema: self.schema,
            strict: self.strict,
        })
    }
}
