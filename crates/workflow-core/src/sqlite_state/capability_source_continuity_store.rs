use rusqlite::{params, OptionalExtension, TransactionBehavior};

use crate::{
    CurrentAuthoritySourceGeneration, CurrentAuthoritySourceWatermark, SpecContentHash,
    WorkflowOsError, WorkflowOsErrorKind,
};

use super::SqliteStateBackend;

pub(crate) struct CapabilitySourceContinuityObservation<'a> {
    pub(crate) continuity_key: &'a SpecContentHash,
    pub(crate) source_registration_commitment: &'a SpecContentHash,
    pub(crate) query_commitment: &'a SpecContentHash,
    pub(crate) generation: Option<CurrentAuthoritySourceGeneration>,
    pub(crate) watermark: &'a CurrentAuthoritySourceWatermark,
}

#[derive(Debug)]
pub(crate) enum CapabilitySourceContinuityPrior {
    First,
    PriorUnversioned {
        watermark: CurrentAuthoritySourceWatermark,
    },
    PriorVersioned {
        generation: CurrentAuthoritySourceGeneration,
        watermark: CurrentAuthoritySourceWatermark,
    },
}

impl SqliteStateBackend {
    #[allow(clippy::too_many_lines)]
    pub(crate) fn compare_and_advance_capability_source_continuity(
        &self,
        observation: &CapabilitySourceContinuityObservation<'_>,
    ) -> Result<CapabilitySourceContinuityPrior, WorkflowOsError> {
        let stored_generation = observation
            .generation
            .map(CurrentAuthoritySourceGeneration::get)
            .map(i64::try_from)
            .transpose()
            .map_err(|_| {
                continuity_error(
                    "generation_invalid",
                    "source continuity generation is invalid",
                )
            })?;
        let mut connection = self.connection()?;
        let transaction = connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(|_| {
                continuity_error("storage_unavailable", "source continuity is unavailable")
            })?;

        let prior = transaction
            .query_row(
                "SELECT source_registration_commitment, query_commitment, generation, watermark
                 FROM capability_source_continuity WHERE continuity_key = ?1",
                params![observation.continuity_key.as_str()],
                |row| {
                    Ok((
                        row.get::<_, String>(0)?,
                        row.get::<_, String>(1)?,
                        row.get::<_, Option<i64>>(2)?,
                        row.get::<_, String>(3)?,
                    ))
                },
            )
            .optional()
            .map_err(|_| {
                continuity_error("storage_unavailable", "source continuity is unavailable")
            })?;

        let continuity = if let Some((source, query, generation, watermark)) = prior {
            if source != observation.source_registration_commitment.as_str()
                || query != observation.query_commitment.as_str()
            {
                return Err(continuity_error(
                    "binding_conflict",
                    "source continuity binding conflicts with durable state",
                ));
            }
            let prior_generation = generation
                .map(u64::try_from)
                .transpose()
                .map_err(|_| {
                    continuity_error("state_invalid", "source continuity state is invalid")
                })?
                .map(CurrentAuthoritySourceGeneration::new)
                .transpose()
                .map_err(|_| {
                    continuity_error("state_invalid", "source continuity state is invalid")
                })?;
            let prior_watermark =
                CurrentAuthoritySourceWatermark::new(watermark).map_err(|_| {
                    continuity_error("state_invalid", "source continuity state is invalid")
                })?;
            validate_transition(
                prior_generation,
                &prior_watermark,
                observation.generation,
                observation.watermark,
            )?;
            if observation.generation != prior_generation
                || observation.watermark != &prior_watermark
            {
                transaction
                    .execute(
                        "UPDATE capability_source_continuity
                         SET generation = ?1, watermark = ?2
                         WHERE continuity_key = ?3
                           AND source_registration_commitment = ?4
                           AND query_commitment = ?5",
                        params![
                            stored_generation,
                            observation.watermark.as_str(),
                            observation.continuity_key.as_str(),
                            observation.source_registration_commitment.as_str(),
                            observation.query_commitment.as_str(),
                        ],
                    )
                    .map_err(|_| {
                        continuity_error("storage_unavailable", "source continuity is unavailable")
                    })?;
            }
            match prior_generation {
                Some(generation) => CapabilitySourceContinuityPrior::PriorVersioned {
                    generation,
                    watermark: prior_watermark,
                },
                None => CapabilitySourceContinuityPrior::PriorUnversioned {
                    watermark: prior_watermark,
                },
            }
        } else {
            transaction
                .execute(
                    "INSERT INTO capability_source_continuity
                     (continuity_key, source_registration_commitment, query_commitment,
                      generation, watermark)
                     VALUES (?1, ?2, ?3, ?4, ?5)",
                    params![
                        observation.continuity_key.as_str(),
                        observation.source_registration_commitment.as_str(),
                        observation.query_commitment.as_str(),
                        stored_generation,
                        observation.watermark.as_str(),
                    ],
                )
                .map_err(|_| {
                    continuity_error("storage_unavailable", "source continuity is unavailable")
                })?;
            CapabilitySourceContinuityPrior::First
        };

        transaction.commit().map_err(|_| {
            continuity_error("commit_ambiguous", "source continuity outcome is ambiguous")
        })?;
        Ok(continuity)
    }
}

fn validate_transition(
    prior_generation: Option<CurrentAuthoritySourceGeneration>,
    prior_watermark: &CurrentAuthoritySourceWatermark,
    current_generation: Option<CurrentAuthoritySourceGeneration>,
    current_watermark: &CurrentAuthoritySourceWatermark,
) -> Result<(), WorkflowOsError> {
    match (prior_generation, current_generation) {
        (Some(_), None) => Err(continuity_error(
            "generation_missing",
            "source continuity generation is unavailable",
        )),
        (Some(prior), Some(current)) if current < prior => Err(continuity_error(
            "generation_rollback",
            "source continuity generation moved backward",
        )),
        (Some(prior), Some(current))
            if current == prior && current_watermark != prior_watermark =>
        {
            Err(continuity_error(
                "watermark_substitution",
                "source continuity changed without a new generation",
            ))
        }
        (None, None) if current_watermark != prior_watermark => Err(continuity_error(
            "unversioned_substitution",
            "unversioned source continuity changed",
        )),
        _ => Ok(()),
    }
}

fn continuity_error(suffix: &str, message: &'static str) -> WorkflowOsError {
    WorkflowOsError::new(
        WorkflowOsErrorKind::Security,
        format!("current_authority.github_repository_read.source_continuity.{suffix}"),
        message,
    )
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use std::fs;
    use std::path::PathBuf;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::{Arc, Barrier};
    use std::thread;

    use super::*;

    struct Fixture {
        backend: SqliteStateBackend,
        root: PathBuf,
        key: SpecContentHash,
        source: SpecContentHash,
        query: SpecContentHash,
    }

    impl Fixture {
        fn new() -> Self {
            static NEXT_DATABASE: AtomicUsize = AtomicUsize::new(1);
            let root = std::env::temp_dir().join(format!(
                "workflow-os-source-continuity-store-{}-{}",
                std::process::id(),
                NEXT_DATABASE.fetch_add(1, Ordering::SeqCst)
            ));
            let _ = fs::remove_dir_all(&root);
            fs::create_dir_all(&root).expect("fixture root");
            Self {
                backend: SqliteStateBackend::open(root.join("state.sqlite3"))
                    .expect("state backend"),
                root,
                key: SpecContentHash::from_text("continuity key"),
                source: SpecContentHash::from_text("source registration"),
                query: SpecContentHash::from_text("exact query"),
            }
        }

        fn observe(
            &self,
            generation: u64,
            watermark: &CurrentAuthoritySourceWatermark,
        ) -> Result<CapabilitySourceContinuityPrior, WorkflowOsError> {
            self.backend
                .compare_and_advance_capability_source_continuity(
                    &CapabilitySourceContinuityObservation {
                        continuity_key: &self.key,
                        source_registration_commitment: &self.source,
                        query_commitment: &self.query,
                        generation: Some(
                            CurrentAuthoritySourceGeneration::new(generation).expect("generation"),
                        ),
                        watermark,
                    },
                )
        }
    }

    impl Drop for Fixture {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.root);
        }
    }

    fn watermark(value: &str) -> CurrentAuthoritySourceWatermark {
        CurrentAuthoritySourceWatermark::new(format!("watermark/{value}")).expect("watermark")
    }

    #[test]
    fn concurrent_exact_observation_has_one_creator_and_one_replay() {
        let fixture = Fixture::new();
        let backend = Arc::new(fixture.backend.clone());
        let barrier = Arc::new(Barrier::new(3));
        let handles = (0..2)
            .map(|_| {
                let backend = Arc::clone(&backend);
                let barrier = Arc::clone(&barrier);
                let key = fixture.key.clone();
                let source = fixture.source.clone();
                let query = fixture.query.clone();
                thread::spawn(move || {
                    let watermark = watermark("same");
                    barrier.wait();
                    backend.compare_and_advance_capability_source_continuity(
                        &CapabilitySourceContinuityObservation {
                            continuity_key: &key,
                            source_registration_commitment: &source,
                            query_commitment: &query,
                            generation: Some(
                                CurrentAuthoritySourceGeneration::new(1).expect("generation"),
                            ),
                            watermark: &watermark,
                        },
                    )
                })
            })
            .collect::<Vec<_>>();
        barrier.wait();
        let outcomes = handles
            .into_iter()
            .map(|handle| handle.join().expect("continuity writer").expect("outcome"))
            .collect::<Vec<_>>();
        assert_eq!(
            outcomes
                .iter()
                .filter(|outcome| matches!(outcome, CapabilitySourceContinuityPrior::First))
                .count(),
            1
        );
        assert_eq!(
            outcomes
                .iter()
                .filter(|outcome| matches!(
                    outcome,
                    CapabilitySourceContinuityPrior::PriorVersioned { .. }
                ))
                .count(),
            1
        );
    }

    #[test]
    fn concurrent_conflicting_observation_has_one_winner() {
        let fixture = Fixture::new();
        let backend = Arc::new(fixture.backend.clone());
        let barrier = Arc::new(Barrier::new(3));
        let handles = ["left", "right"]
            .into_iter()
            .map(|value| {
                let backend = Arc::clone(&backend);
                let barrier = Arc::clone(&barrier);
                let key = fixture.key.clone();
                let source = fixture.source.clone();
                let query = fixture.query.clone();
                thread::spawn(move || {
                    let watermark = watermark(value);
                    barrier.wait();
                    backend.compare_and_advance_capability_source_continuity(
                        &CapabilitySourceContinuityObservation {
                            continuity_key: &key,
                            source_registration_commitment: &source,
                            query_commitment: &query,
                            generation: Some(
                                CurrentAuthoritySourceGeneration::new(1).expect("generation"),
                            ),
                            watermark: &watermark,
                        },
                    )
                })
            })
            .collect::<Vec<_>>();
        barrier.wait();
        let outcomes = handles
            .into_iter()
            .map(|handle| handle.join().expect("continuity writer"))
            .collect::<Vec<_>>();
        assert_eq!(outcomes.iter().filter(|outcome| outcome.is_ok()).count(), 1);
        let error = outcomes
            .into_iter()
            .find_map(Result::err)
            .expect("one conflict");
        assert_eq!(
            error.code(),
            "current_authority.github_repository_read.source_continuity.watermark_substitution"
        );
    }

    #[test]
    fn durable_binding_substitution_fails_without_leaking_values() {
        let fixture = Fixture::new();
        fixture.observe(1, &watermark("first")).expect("first");
        let secret = "secret-token-binding";
        let substituted = SpecContentHash::from_text(secret);
        for (source, query) in [
            (&substituted, &fixture.query),
            (&fixture.source, &substituted),
        ] {
            let error = fixture
                .backend
                .compare_and_advance_capability_source_continuity(
                    &CapabilitySourceContinuityObservation {
                        continuity_key: &fixture.key,
                        source_registration_commitment: source,
                        query_commitment: query,
                        generation: Some(
                            CurrentAuthoritySourceGeneration::new(1).expect("generation"),
                        ),
                        watermark: &watermark("first"),
                    },
                )
                .expect_err("binding substitution rejected");
            assert_eq!(
                error.code(),
                "current_authority.github_repository_read.source_continuity.binding_conflict"
            );
            assert!(!error.to_string().contains(secret));
        }

        let database = fs::read(fixture.root.join("state.sqlite3")).expect("database bytes");
        let database = String::from_utf8_lossy(&database);
        assert!(!database.contains("source registration"));
        assert!(!database.contains("exact query"));
        assert!(!database.contains(secret));
    }
}
