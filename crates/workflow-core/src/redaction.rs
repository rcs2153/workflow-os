use std::fmt;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Wrapper for sensitive values that must not be exposed through display or debug output.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct RedactedValue<T> {
    value: T,
}

impl<T> RedactedValue<T> {
    /// Wraps a sensitive value.
    #[must_use]
    pub const fn new(value: T) -> Self {
        Self { value }
    }

    /// Returns a reference to the sensitive inner value.
    ///
    /// Callers should avoid logging, displaying, or serializing this value directly.
    #[must_use]
    pub const fn expose_secret(&self) -> &T {
        &self.value
    }

    /// Consumes the wrapper and returns the sensitive inner value.
    #[must_use]
    pub fn into_secret(self) -> T {
        self.value
    }
}

impl<T> fmt::Display for RedactedValue<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("[REDACTED]")
    }
}

impl<T> fmt::Debug for RedactedValue<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("RedactedValue([REDACTED])")
    }
}

impl<T> Serialize for RedactedValue<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str("[REDACTED]")
    }
}

impl<'de, T> Deserialize<'de> for RedactedValue<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Self::new(T::deserialize(deserializer)?))
    }
}
