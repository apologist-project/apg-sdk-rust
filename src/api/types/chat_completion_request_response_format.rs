pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ChatCompletionRequestResponseFormat {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<ChatCompletionRequestResponseFormatType>,
    /// Required when type is json_schema. Supplies the JSON Schema the structured output must conform to. Structured outputs are non-streaming.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub json_schema: Option<ChatCompletionRequestResponseFormatJsonSchema>,
}

impl ChatCompletionRequestResponseFormat {
    pub fn builder() -> ChatCompletionRequestResponseFormatBuilder {
        <ChatCompletionRequestResponseFormatBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ChatCompletionRequestResponseFormatBuilder {
    r#type: Option<ChatCompletionRequestResponseFormatType>,
    json_schema: Option<ChatCompletionRequestResponseFormatJsonSchema>,
}

impl ChatCompletionRequestResponseFormatBuilder {
    pub fn r#type(mut self, value: ChatCompletionRequestResponseFormatType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn json_schema(mut self, value: ChatCompletionRequestResponseFormatJsonSchema) -> Self {
        self.json_schema = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ChatCompletionRequestResponseFormat`].
    pub fn build(self) -> Result<ChatCompletionRequestResponseFormat, BuildError> {
        Ok(ChatCompletionRequestResponseFormat {
            r#type: self.r#type,
            json_schema: self.json_schema,
        })
    }
}
