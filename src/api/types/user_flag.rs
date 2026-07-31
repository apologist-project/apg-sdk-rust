pub use crate::prelude::*;

/// A team-level user flag definition from the user_flags table.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UserFlag {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Upstream owning user id when present (mirrored from Ignite).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub synced_at: Option<String>,
}

impl UserFlag {
    pub fn builder() -> UserFlagBuilder {
        <UserFlagBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UserFlagBuilder {
    id: Option<i64>,
    name: Option<String>,
    user_id: Option<i64>,
    team_id: Option<i64>,
    synced_at: Option<String>,
}

impl UserFlagBuilder {
    pub fn id(mut self, value: i64) -> Self {
        self.id = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn user_id(mut self, value: i64) -> Self {
        self.user_id = Some(value);
        self
    }

    pub fn team_id(mut self, value: i64) -> Self {
        self.team_id = Some(value);
        self
    }

    pub fn synced_at(mut self, value: impl Into<String>) -> Self {
        self.synced_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UserFlag`].
    pub fn build(self) -> Result<UserFlag, BuildError> {
        Ok(UserFlag {
            id: self.id,
            name: self.name,
            user_id: self.user_id,
            team_id: self.team_id,
            synced_at: self.synced_at,
        })
    }
}
