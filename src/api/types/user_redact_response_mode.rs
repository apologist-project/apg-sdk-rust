pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UserRedactResponseMode {
    Scrub,
    Anonymize,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for UserRedactResponseMode {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Scrub => serializer.serialize_str("scrub"),
            Self::Anonymize => serializer.serialize_str("anonymize"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for UserRedactResponseMode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "scrub" => Ok(Self::Scrub),
            "anonymize" => Ok(Self::Anonymize),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for UserRedactResponseMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Scrub => write!(f, "scrub"),
            Self::Anonymize => write!(f, "anonymize"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
