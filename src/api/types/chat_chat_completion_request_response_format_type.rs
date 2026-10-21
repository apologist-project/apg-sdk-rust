pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ChatCompletionRequestResponseFormatType {
    Text,
    Html,
    Json,
    Raw,
    JsonSchema,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ChatCompletionRequestResponseFormatType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Text => serializer.serialize_str("text"),
            Self::Html => serializer.serialize_str("html"),
            Self::Json => serializer.serialize_str("json"),
            Self::Raw => serializer.serialize_str("raw"),
            Self::JsonSchema => serializer.serialize_str("json_schema"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ChatCompletionRequestResponseFormatType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "text" => Ok(Self::Text),
            "html" => Ok(Self::Html),
            "json" => Ok(Self::Json),
            "raw" => Ok(Self::Raw),
            "json_schema" => Ok(Self::JsonSchema),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ChatCompletionRequestResponseFormatType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Text => write!(f, "text"),
            Self::Html => write!(f, "html"),
            Self::Json => write!(f, "json"),
            Self::Raw => write!(f, "raw"),
            Self::JsonSchema => write!(f, "json_schema"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
