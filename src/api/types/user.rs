pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct User {
    /// Internal user id (UUID).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    /// First-write-wins acquisition / campaign referral code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referral_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub migrated_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub migrated_to_user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<TagRef>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub responder_id: Option<i64>,
}

impl User {
    pub fn builder() -> UserBuilder {
        <UserBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UserBuilder {
    id: Option<String>,
    external_id: Option<String>,
    referral_code: Option<String>,
    team_id: Option<i64>,
    created_at: Option<String>,
    migrated_at: Option<String>,
    migrated_to_user_id: Option<String>,
    tags: Option<Vec<TagRef>>,
    responder_id: Option<i64>,
}

impl UserBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn external_id(mut self, value: impl Into<String>) -> Self {
        self.external_id = Some(value.into());
        self
    }

    pub fn referral_code(mut self, value: impl Into<String>) -> Self {
        self.referral_code = Some(value.into());
        self
    }

    pub fn team_id(mut self, value: i64) -> Self {
        self.team_id = Some(value);
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn migrated_at(mut self, value: impl Into<String>) -> Self {
        self.migrated_at = Some(value.into());
        self
    }

    pub fn migrated_to_user_id(mut self, value: impl Into<String>) -> Self {
        self.migrated_to_user_id = Some(value.into());
        self
    }

    pub fn tags(mut self, value: Vec<TagRef>) -> Self {
        self.tags = Some(value);
        self
    }

    pub fn responder_id(mut self, value: i64) -> Self {
        self.responder_id = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`User`].
    pub fn build(self) -> Result<User, BuildError> {
        Ok(User {
            id: self.id,
            external_id: self.external_id,
            referral_code: self.referral_code,
            team_id: self.team_id,
            created_at: self.created_at,
            migrated_at: self.migrated_at,
            migrated_to_user_id: self.migrated_to_user_id,
            tags: self.tags,
            responder_id: self.responder_id,
        })
    }
}
