use std::fmt;

use crate::WorkflowOsError;

const EXACT_ACCESS_REFERENCE: &str = "env/WORKFLOW_OS_GITHUB_TOKEN";
const EXACT_ENVIRONMENT_VARIABLE: &str = "WORKFLOW_OS_GITHUB_TOKEN";

/// Typed reference to the only credential source accepted by the first GitHub
/// repository-metadata read contract.
pub(super) struct GitHubRepositoryMetadataAccessReference(());

impl GitHubRepositoryMetadataAccessReference {
    /// Validates and constructs the exact supported access reference.
    pub(super) fn parse(value: &str) -> Result<Self, WorkflowOsError> {
        if value != EXACT_ACCESS_REFERENCE {
            return Err(WorkflowOsError::security(
                "github.repository_metadata.access_reference.unsupported",
                "GitHub repository metadata access requires the exact declared access reference",
            ));
        }

        Ok(Self(()))
    }
}

impl fmt::Debug for GitHubRepositoryMetadataAccessReference {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("GitHubRepositoryMetadataAccessReference")
            .field(&EXACT_ACCESS_REFERENCE)
            .finish()
    }
}

/// Presence-only posture for the exact GitHub repository metadata credential.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum GitHubRepositoryMetadataAccessPresence {
    /// The exact environment variable contains a non-empty value.
    Present,
    /// The exact environment variable is absent or empty.
    Missing,
}

/// Injected environment lookup boundary.
///
/// Implementations must return the value for the requested variable without
/// logging or otherwise retaining it.
pub(super) trait GitHubRepositoryEnvironmentReader {
    fn read(&self, name: &str) -> Option<String>;
}

impl<F> GitHubRepositoryEnvironmentReader for F
where
    F: Fn(&str) -> Option<String>,
{
    fn read(&self, name: &str) -> Option<String> {
        self(name)
    }
}

/// Ephemeral GitHub credential material.
///
/// This type intentionally implements neither cloning nor serialization and
/// exposes no string-returning accessor. Its debug representation is always
/// redacted.
struct TransientGitHubRepositoryAccess {
    value: String,
}

impl TransientGitHubRepositoryAccess {
    fn new(value: String) -> Option<Self> {
        (!value.is_empty()).then_some(Self { value })
    }

    fn consume_with<R>(self, consumer: impl for<'access> FnOnce(&'access str) -> R) -> R {
        consumer(&self.value)
    }
}

impl fmt::Debug for TransientGitHubRepositoryAccess {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("TransientGitHubRepositoryAccess([REDACTED])")
    }
}

/// Resolves the exact credential only inside a caller-supplied closure.
pub(super) struct GitHubRepositoryTransientAccessResolver<R> {
    environment: R,
}

impl<R> GitHubRepositoryTransientAccessResolver<R>
where
    R: GitHubRepositoryEnvironmentReader,
{
    pub(super) const fn new(environment: R) -> Self {
        Self { environment }
    }

    /// Reports whether the exact credential is currently present without
    /// exposing its value.
    pub(super) fn presence(
        &self,
        _reference: &GitHubRepositoryMetadataAccessReference,
    ) -> GitHubRepositoryMetadataAccessPresence {
        match self
            .environment
            .read(EXACT_ENVIRONMENT_VARIABLE)
            .and_then(TransientGitHubRepositoryAccess::new)
        {
            Some(_) => GitHubRepositoryMetadataAccessPresence::Present,
            None => GitHubRepositoryMetadataAccessPresence::Missing,
        }
    }

    /// Resolves and consumes the exact credential within one lexical scope.
    ///
    /// # Errors
    ///
    /// Returns a non-sensitive security error when the exact credential is
    /// absent or empty. No fallback variable is consulted.
    pub(super) fn with_access<T>(
        &self,
        _reference: &GitHubRepositoryMetadataAccessReference,
        consumer: impl for<'access> FnOnce(&'access str) -> T,
    ) -> Result<T, WorkflowOsError> {
        let access = self
            .environment
            .read(EXACT_ENVIRONMENT_VARIABLE)
            .and_then(TransientGitHubRepositoryAccess::new)
            .ok_or_else(|| {
                WorkflowOsError::security(
                    "github.repository_metadata.access.missing",
                    "GitHub repository metadata access is not connected",
                )
            })?;

        Ok(access.consume_with(consumer))
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::expect_used)]

    use std::cell::{Cell, RefCell};
    use std::collections::BTreeMap;

    use super::*;

    struct RecordingEnvironment {
        values: BTreeMap<String, String>,
        reads: RefCell<Vec<String>>,
    }

    impl RecordingEnvironment {
        fn new(values: impl IntoIterator<Item = (&'static str, &'static str)>) -> Self {
            Self {
                values: values
                    .into_iter()
                    .map(|(name, value)| (name.to_owned(), value.to_owned()))
                    .collect(),
                reads: RefCell::new(Vec::new()),
            }
        }
    }

    impl GitHubRepositoryEnvironmentReader for RecordingEnvironment {
        fn read(&self, name: &str) -> Option<String> {
            self.reads.borrow_mut().push(name.to_owned());
            self.values.get(name).cloned()
        }
    }

    #[test]
    fn access_reference_accepts_only_the_exact_declared_value() {
        assert!(
            GitHubRepositoryMetadataAccessReference::parse("env/WORKFLOW_OS_GITHUB_TOKEN").is_ok()
        );

        for unsupported in [
            "WORKFLOW_OS_GITHUB_TOKEN",
            "env/GITHUB_TOKEN",
            "env/workflow_os_github_token",
            "env/WORKFLOW_OS_GITHUB_TOKEN ",
            "",
        ] {
            let error = GitHubRepositoryMetadataAccessReference::parse(unsupported)
                .expect_err("unsupported reference must fail closed");
            assert_eq!(
                error.code(),
                "github.repository_metadata.access_reference.unsupported"
            );
            if !unsupported.is_empty() {
                assert!(!error.to_string().contains(unsupported));
            }
        }
    }

    #[test]
    fn presence_reads_only_the_exact_environment_variable() {
        let reference = GitHubRepositoryMetadataAccessReference::parse(EXACT_ACCESS_REFERENCE)
            .expect("exact reference");
        let environment = RecordingEnvironment::new([
            ("WORKFLOW_OS_GITHUB_TOKEN", "exact-secret-marker"),
            ("GITHUB_TOKEN", "fallback-secret-marker"),
        ]);
        let resolver = GitHubRepositoryTransientAccessResolver::new(environment);

        assert_eq!(
            resolver.presence(&reference),
            GitHubRepositoryMetadataAccessPresence::Present
        );
        assert_eq!(
            resolver.environment.reads.borrow().as_slice(),
            ["WORKFLOW_OS_GITHUB_TOKEN"]
        );
    }

    #[test]
    fn fallback_token_does_not_satisfy_presence_or_access() {
        let reference = GitHubRepositoryMetadataAccessReference::parse(EXACT_ACCESS_REFERENCE)
            .expect("exact reference");
        let environment = RecordingEnvironment::new([("GITHUB_TOKEN", "fallback-secret-marker")]);
        let resolver = GitHubRepositoryTransientAccessResolver::new(environment);
        let consumer_called = Cell::new(false);

        assert_eq!(
            resolver.presence(&reference),
            GitHubRepositoryMetadataAccessPresence::Missing
        );
        let error = resolver
            .with_access(&reference, |_| {
                consumer_called.set(true);
            })
            .expect_err("fallback token must not authorize access");

        assert_eq!(error.code(), "github.repository_metadata.access.missing");
        assert!(!consumer_called.get());
        assert_eq!(
            resolver.environment.reads.borrow().as_slice(),
            ["WORKFLOW_OS_GITHUB_TOKEN", "WORKFLOW_OS_GITHUB_TOKEN"]
        );
    }

    #[test]
    fn empty_exact_token_is_treated_as_missing() {
        let reference = GitHubRepositoryMetadataAccessReference::parse(EXACT_ACCESS_REFERENCE)
            .expect("exact reference");
        let resolver = GitHubRepositoryTransientAccessResolver::new(RecordingEnvironment::new([(
            "WORKFLOW_OS_GITHUB_TOKEN",
            "",
        )]));

        assert_eq!(
            resolver.presence(&reference),
            GitHubRepositoryMetadataAccessPresence::Missing
        );
        assert_eq!(
            resolver
                .with_access(&reference, |_| ())
                .expect_err("empty token must fail closed")
                .code(),
            "github.repository_metadata.access.missing"
        );
    }

    #[test]
    fn resolver_consumes_access_in_the_supplied_scope() {
        let reference = GitHubRepositoryMetadataAccessReference::parse(EXACT_ACCESS_REFERENCE)
            .expect("exact reference");
        let resolver = GitHubRepositoryTransientAccessResolver::new(RecordingEnvironment::new([(
            "WORKFLOW_OS_GITHUB_TOKEN",
            "exact-secret-marker",
        )]));

        let observed_length = resolver
            .with_access(&reference, str::len)
            .expect("exact token should be available to the scoped consumer");

        assert_eq!(observed_length, "exact-secret-marker".len());
    }

    #[test]
    fn transient_access_debug_is_redacted() {
        let access = TransientGitHubRepositoryAccess::new("exact-secret-marker".to_owned())
            .expect("non-empty access");
        let rendered = format!("{access:?}");

        assert_eq!(rendered, "TransientGitHubRepositoryAccess([REDACTED])");
        assert!(!rendered.contains("exact-secret-marker"));
    }

    #[test]
    fn closure_reader_is_supported_without_process_environment_access() {
        let reference = GitHubRepositoryMetadataAccessReference::parse(EXACT_ACCESS_REFERENCE)
            .expect("exact reference");
        let resolver = GitHubRepositoryTransientAccessResolver::new(|name: &str| {
            (name == "WORKFLOW_OS_GITHUB_TOKEN").then(|| "exact-secret-marker".to_owned())
        });

        assert_eq!(
            resolver.presence(&reference),
            GitHubRepositoryMetadataAccessPresence::Present
        );
    }
}
