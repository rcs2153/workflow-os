use std::fmt;

use serde::{Deserialize, Serialize};
use time::format_description::well_known::Rfc3339;
use time::OffsetDateTime;

use crate::WorkflowOsError;

/// UTC timestamp serialized as RFC 3339 text.
#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct Timestamp(OffsetDateTime);

impl Timestamp {
    /// Returns the current UTC timestamp.
    #[must_use]
    pub fn now_utc() -> Self {
        Self(OffsetDateTime::now_utc())
    }

    /// Creates a timestamp from an `OffsetDateTime`, normalized to UTC.
    #[must_use]
    pub fn from_offset_date_time(value: OffsetDateTime) -> Self {
        Self(value.to_offset(time::UtcOffset::UTC))
    }

    /// Parses an RFC 3339 timestamp.
    ///
    /// # Errors
    ///
    /// Returns an error when the value is not valid RFC 3339 timestamp text.
    pub fn parse_rfc3339(value: &str) -> Result<Self, WorkflowOsError> {
        let timestamp = OffsetDateTime::parse(value, &Rfc3339).map_err(|_| {
            WorkflowOsError::validation(
                "timestamp.invalid",
                "timestamp must be valid RFC 3339 text",
            )
        })?;
        Ok(Self::from_offset_date_time(timestamp))
    }

    /// Formats the timestamp as RFC 3339 text.
    #[must_use]
    pub fn to_rfc3339(&self) -> String {
        self.0
            .format(&Rfc3339)
            .unwrap_or_else(|_| "1970-01-01T00:00:00Z".to_owned())
    }

    /// Returns the timestamp as an `OffsetDateTime`.
    #[must_use]
    pub const fn as_offset_date_time(&self) -> OffsetDateTime {
        self.0
    }
}

impl fmt::Display for Timestamp {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.to_rfc3339())
    }
}

impl fmt::Debug for Timestamp {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("Timestamp")
            .field(&self.to_rfc3339())
            .finish()
    }
}

impl TryFrom<String> for Timestamp {
    type Error = WorkflowOsError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::parse_rfc3339(&value)
    }
}

impl From<Timestamp> for String {
    fn from(value: Timestamp) -> Self {
        value.to_rfc3339()
    }
}
