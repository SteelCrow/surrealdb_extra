use chrono::{DateTime, Utc};
use surrealdb::sql::Version;

#[derive(Debug, Clone)]
pub struct ExtraVersion(pub Version);

impl From<Version> for ExtraVersion {
    fn from(value: Version) -> Self {
        Self(value)
    }
}

impl From<DateTime<Utc>> for ExtraVersion {
    fn from(value: DateTime<Utc>) -> Self {
        Self(Version(value.into()))
    }
}
