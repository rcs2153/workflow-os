use std::fmt;
use std::str::FromStr;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::WorkflowOsError;

static NEXT_GENERATED_ID: AtomicU64 = AtomicU64::new(1);

fn validate_identifier(type_name: &'static str, value: &str) -> Result<(), WorkflowOsError> {
    if value.is_empty() {
        return Err(WorkflowOsError::validation(
            "identifier.empty",
            format!("{type_name} cannot be empty"),
        ));
    }

    if value.len() > 128 {
        return Err(WorkflowOsError::validation(
            "identifier.too_long",
            format!("{type_name} cannot exceed 128 bytes"),
        ));
    }

    let is_valid = value
        .bytes()
        .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-' | b'/'));

    if !is_valid {
        return Err(WorkflowOsError::validation(
            "identifier.invalid_character",
            format!("{type_name} contains an invalid character"),
        ));
    }

    Ok(())
}

macro_rules! string_id {
    ($(#[$meta:meta])* $name:ident) => {
        $(#[$meta])*
        #[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
        #[serde(try_from = "String", into = "String")]
        pub struct $name(String);

        impl $name {
            /// Creates a new strongly typed identifier from a validated string.
            ///
            /// # Errors
            ///
            /// Returns an error when the identifier is empty, too long, or contains
            /// characters outside the canonical identifier character set.
            pub fn new(value: impl Into<String>) -> Result<Self, WorkflowOsError> {
                let value = value.into();
                validate_identifier(stringify!($name), &value)?;
                Ok(Self(value))
            }

            /// Returns the identifier as a string slice.
            #[must_use]
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str(&self.0)
            }
        }

        impl fmt::Debug for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.debug_tuple(stringify!($name)).field(&self.0).finish()
            }
        }

        impl From<$name> for String {
            fn from(value: $name) -> Self {
                value.0
            }
        }

        impl TryFrom<String> for $name {
            type Error = WorkflowOsError;

            fn try_from(value: String) -> Result<Self, Self::Error> {
                Self::new(value)
            }
        }

        impl FromStr for $name {
            type Err = WorkflowOsError;

            fn from_str(value: &str) -> Result<Self, Self::Err> {
                Self::new(value)
            }
        }
    };
}

fn generate_value(prefix: &str) -> String {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    let counter = NEXT_GENERATED_ID.fetch_add(1, Ordering::Relaxed);

    format!("{prefix}-{timestamp}-{counter}")
}

macro_rules! generated_id {
    ($(#[$meta:meta])* $name:ident, $prefix:literal) => {
        $(#[$meta])*
        #[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
        #[serde(try_from = "String", into = "String")]
        pub struct $name(String);

        impl $name {
            /// Generates a new unique identifier.
            #[must_use]
            pub fn generate() -> Self {
                Self(generate_value($prefix))
            }

            /// Creates a generated identifier from a validated string.
            ///
            /// # Errors
            ///
            /// Returns an error when the identifier is empty, too long, or contains
            /// characters outside the canonical identifier character set.
            pub fn new(value: impl Into<String>) -> Result<Self, WorkflowOsError> {
                let value = value.into();
                validate_identifier(stringify!($name), &value)?;
                Ok(Self(value))
            }

            /// Returns the identifier as a string slice.
            #[must_use]
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str(&self.0)
            }
        }

        impl fmt::Debug for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.debug_tuple(stringify!($name)).field(&self.0).finish()
            }
        }

        impl From<$name> for String {
            fn from(value: $name) -> Self {
                value.0
            }
        }

        impl TryFrom<String> for $name {
            type Error = WorkflowOsError;

            fn try_from(value: String) -> Result<Self, Self::Error> {
                Self::new(value)
            }
        }

        impl FromStr for $name {
            type Err = WorkflowOsError;

            fn from_str(value: &str) -> Result<Self, Self::Err> {
                Self::new(value)
            }
        }
    };
}

string_id!(
    /// Identifier for a local Workflow OS project.
    ProjectId
);
string_id!(
    /// Identifier for a workflow definition.
    WorkflowId
);
string_id!(
    /// Version identifier for a workflow definition.
    WorkflowVersion
);
string_id!(
    /// Identifier for a step within a workflow definition.
    StepId
);
generated_id!(
    /// Identifier for a workflow run.
    WorkflowRunId,
    "run"
);
string_id!(
    /// Identifier for a skill definition.
    SkillId
);
string_id!(
    /// Version identifier for a skill definition.
    SkillVersion
);
generated_id!(
    /// Identifier for one logical skill invocation.
    SkillInvocationId,
    "skill-invocation"
);
generated_id!(
    /// Identifier for one attempt of a skill invocation.
    SkillAttemptId,
    "skill-attempt"
);
generated_id!(
    /// Identifier for an event in the append-only event log.
    EventId,
    "event"
);
generated_id!(
    /// Identifier used to correlate related operations, logs, diagnostics, and events.
    CorrelationId,
    "correlation"
);
string_id!(
    /// Stable key used to deduplicate repeat requests and side-effect attempts.
    IdempotencyKey
);
string_id!(
    /// Identifier for a human or system actor.
    ActorId
);
string_id!(
    /// Identifier for an adapter implementation.
    AdapterId
);
string_id!(
    /// Identifier for a configured external integration.
    IntegrationId
);
string_id!(
    /// Identifier for a policy definition.
    PolicyId
);
string_id!(
    /// Version identifier for a public Workflow OS schema.
    SchemaVersion
);

/// Deterministic SHA-256 content hash for a canonical spec representation.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct SpecContentHash(String);

impl SpecContentHash {
    /// Creates a spec content hash from lowercase hexadecimal SHA-256 text.
    ///
    /// # Errors
    ///
    /// Returns an error when the value is not a lowercase SHA-256 hex digest.
    pub fn new(value: impl Into<String>) -> Result<Self, WorkflowOsError> {
        let value = value.into();
        let is_valid = value.len() == 64
            && value
                .bytes()
                .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte));

        if !is_valid {
            return Err(WorkflowOsError::validation(
                "spec_hash.invalid",
                "spec content hash must be a lowercase SHA-256 hex digest",
            ));
        }

        Ok(Self(value))
    }

    /// Hashes bytes using SHA-256.
    #[must_use]
    pub fn from_bytes(bytes: impl AsRef<[u8]>) -> Self {
        let digest = Sha256::digest(bytes.as_ref());
        Self(hex_lower(&digest))
    }

    /// Hashes text using SHA-256 over the exact UTF-8 bytes.
    #[must_use]
    pub fn from_text(text: &str) -> Self {
        Self::from_bytes(text.as_bytes())
    }

    /// Returns the lowercase hexadecimal digest.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for SpecContentHash {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl fmt::Debug for SpecContentHash {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("SpecContentHash")
            .field(&self.0)
            .finish()
    }
}

impl From<SpecContentHash> for String {
    fn from(value: SpecContentHash) -> Self {
        value.0
    }
}

impl TryFrom<String> for SpecContentHash {
    type Error = WorkflowOsError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl FromStr for SpecContentHash {
    type Err = WorkflowOsError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

fn hex_lower(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut output = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        output.push(char::from(HEX[usize::from(byte >> 4)]));
        output.push(char::from(HEX[usize::from(byte & 0x0f)]));
    }
    output
}
