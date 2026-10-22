#![deny(unsafe_code)]
#![doc = "Unpublished foreground entry point for one explicit trusted-host operation."]

use std::collections::BTreeMap;
use std::env;
use std::path::PathBuf;
use std::process::ExitCode;

use workflow_core::{
    admit_trusted_host_local_application_operation, admit_trusted_host_two_step_no_write_operation,
    persist_trusted_host_terminal_work_report_artifact, ActorId, CorrelationId,
    ExplicitLocalCheckProfileSelection, ImmutableRunBundleId, ImmutableRunBundleSensitivity,
    ImmutableRunBundleVersion, LocalExecutionBeforeSkillInvocationCheckpointInputs,
    LocalExecutionImmutableRunBundleInputs, LocalExecutionRequest,
    LocalExecutionWithImmutableRunBundleRequest, SqliteStateBackend, StepId, Timestamp,
    TrustedHostLocalApplicationAdmissionOutcome, TrustedHostLocalApplicationAdmissionRequest,
    TrustedHostTerminalWorkReportArtifactPosture, TrustedHostTwoStepNoWriteAdmissionRequest,
    TrustedHostTwoStepNoWriteProfileRegistry, WorkflowId, WorkflowRunId,
};
use workflow_local_host::LocalHostPreparedOperation;

const ALLOWED_ARGUMENTS: [&str; 16] = [
    "mode",
    "profile",
    "governance-project-root",
    "repository-root",
    "state-database",
    "npm-executable",
    "workflow-os-executable",
    "workflow-id",
    "step-id",
    "run-id",
    "bundle-id",
    "observed-at",
    "expires-at",
    "actor",
    "correlation-id",
    "npm-cache-directory",
];
const REQUIRED_ARGUMENTS: [&str; 9] = [
    "governance-project-root",
    "state-database",
    "workflow-id",
    "run-id",
    "bundle-id",
    "observed-at",
    "expires-at",
    "actor",
    "correlation-id",
];

fn main() -> ExitCode {
    match run() {
        Ok(code) => {
            println!("trusted_host_local_application: {code}");
            ExitCode::SUCCESS
        }
        Err(code) => {
            eprintln!("trusted_host_local_application: {code}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<&'static str, &'static str> {
    let arguments = parse_arguments()?;
    match arguments.get("mode").map(String::as_str) {
        None | Some("one-step") => run_one_step(&arguments),
        Some("two-step-no-write") => run_two_step(&arguments),
        Some(_) => Err("input.invalid"),
    }
}

fn run_one_step(arguments: &BTreeMap<String, String>) -> Result<&'static str, &'static str> {
    if arguments.contains_key("mode")
        && arguments.get("mode").map(String::as_str) != Some("one-step")
    {
        return Err("input.invalid");
    }
    let governance_project_root = path_argument(arguments, "governance-project-root")?;
    let profile = resolve_profile(arguments, &governance_project_root)?;
    let backend = SqliteStateBackend::open(path_argument(arguments, "state-database")?)
        .map_err(|_| "state.unavailable")?;
    let observed_at = timestamp_argument(arguments, "observed-at")?;
    let expires_at = timestamp_argument(arguments, "expires-at")?;
    if expires_at <= observed_at {
        return Err("input.invalid");
    }
    let request = TrustedHostLocalApplicationAdmissionRequest {
        execution: LocalExecutionWithImmutableRunBundleRequest {
            execution: LocalExecutionRequest {
                project_root: governance_project_root,
                workflow_id: WorkflowId::new(argument(arguments, "workflow-id")?)
                    .map_err(|_| "input.invalid")?,
                run_id: Some(
                    WorkflowRunId::new(argument(arguments, "run-id")?)
                        .map_err(|_| "input.invalid")?,
                ),
                correlation_id: CorrelationId::new(argument(arguments, "correlation-id")?)
                    .map_err(|_| "input.invalid")?,
                actor: ActorId::new(argument(arguments, "actor")?).map_err(|_| "input.invalid")?,
                before_skill_invocation_checkpoints:
                    LocalExecutionBeforeSkillInvocationCheckpointInputs::default(),
                before_skill_invocation_hook: None,
                side_effect_events: Vec::new(),
                side_effect_lifecycle_events: Vec::new(),
            },
            bundle: LocalExecutionImmutableRunBundleInputs {
                bundle_id: ImmutableRunBundleId::new(argument(arguments, "bundle-id")?)
                    .map_err(|_| "input.invalid")?,
                bundle_version: ImmutableRunBundleVersion::new("v1")
                    .map_err(|_| "input.invalid")?,
                created_at: observed_at,
                sensitivity: ImmutableRunBundleSensitivity::Internal,
                redaction_required: true,
            },
        },
        selected_step_id: StepId::new(argument(arguments, "step-id")?)
            .map_err(|_| "input.invalid")?,
        observed_at,
        expires_at,
    };

    let admitted = admit_trusted_host_local_application_operation(&backend, &profile, &request)
        .map_err(workflow_core::TrustedHostLocalApplicationFailure::code)?;
    match admitted {
        TrustedHostLocalApplicationAdmissionOutcome::Prepared(prepared) => {
            let outcome = run_prepared(prepared)?;
            if matches!(outcome, "operation.terminal" | "continuation.terminal") {
                artifact_posture(&backend, &profile, &request, false)
            } else {
                Ok(outcome)
            }
        }
        TrustedHostLocalApplicationAdmissionOutcome::Waiting(reason) => Ok(match reason {
            workflow_core::TrustedHostLocalApplicationWaitReason::Approval => {
                "admission.waiting_approval"
            }
            workflow_core::TrustedHostLocalApplicationWaitReason::ExternalCondition => {
                "admission.waiting_external_condition"
            }
        }),
        TrustedHostLocalApplicationAdmissionOutcome::TerminalReplay => {
            artifact_posture(&backend, &profile, &request, true)
        }
        TrustedHostLocalApplicationAdmissionOutcome::Blocked(reason) => Ok(match reason {
            workflow_core::TrustedHostLocalApplicationBlockReason::UnsupportedWorkflow => {
                "admission.blocked_unsupported_workflow"
            }
            workflow_core::TrustedHostLocalApplicationBlockReason::CurrentState => {
                "admission.blocked_current_state"
            }
        }),
        TrustedHostLocalApplicationAdmissionOutcome::Denied => Ok("admission.denied"),
    }
}

#[allow(clippy::too_many_lines)]
fn run_two_step(arguments: &BTreeMap<String, String>) -> Result<&'static str, &'static str> {
    if arguments.contains_key("profile") || arguments.contains_key("step-id") {
        return Err("input.invalid");
    }
    let governance_project_root = path_argument(arguments, "governance-project-root")?;
    let repository_root = path_argument(arguments, "repository-root")?;
    let npm_executable = path_argument(arguments, "npm-executable")?;
    let npm_cache_directory = arguments.get("npm-cache-directory").map(PathBuf::from);
    let docs_check = ExplicitLocalCheckProfileSelection::docs_check()
        .resolve_docs_check(npm_executable, repository_root, npm_cache_directory)
        .map_err(|_| "profile.invalid")?;
    let project_validation = ExplicitLocalCheckProfileSelection::workflow_os_project_validation()
        .resolve(
            path_argument(arguments, "workflow-os-executable")?,
            governance_project_root.clone(),
        )
        .map_err(|_| "profile.invalid")?;
    let registry = TrustedHostTwoStepNoWriteProfileRegistry::new(project_validation, docs_check)
        .map_err(|_| "profile.invalid")?;
    let backend = SqliteStateBackend::open(path_argument(arguments, "state-database")?)
        .map_err(|_| "state.unavailable")?;
    let observed_at = timestamp_argument(arguments, "observed-at")?;
    let expires_at = timestamp_argument(arguments, "expires-at")?;
    if expires_at <= observed_at {
        return Err("input.invalid");
    }
    let mut request = TrustedHostTwoStepNoWriteAdmissionRequest {
        execution: LocalExecutionWithImmutableRunBundleRequest {
            execution: LocalExecutionRequest {
                project_root: governance_project_root,
                workflow_id: WorkflowId::new(argument(arguments, "workflow-id")?)
                    .map_err(|_| "input.invalid")?,
                run_id: Some(
                    WorkflowRunId::new(argument(arguments, "run-id")?)
                        .map_err(|_| "input.invalid")?,
                ),
                correlation_id: CorrelationId::new(argument(arguments, "correlation-id")?)
                    .map_err(|_| "input.invalid")?,
                actor: ActorId::new(argument(arguments, "actor")?).map_err(|_| "input.invalid")?,
                before_skill_invocation_checkpoints:
                    LocalExecutionBeforeSkillInvocationCheckpointInputs::default(),
                before_skill_invocation_hook: None,
                side_effect_events: Vec::new(),
                side_effect_lifecycle_events: Vec::new(),
            },
            bundle: LocalExecutionImmutableRunBundleInputs {
                bundle_id: ImmutableRunBundleId::new(argument(arguments, "bundle-id")?)
                    .map_err(|_| "input.invalid")?,
                bundle_version: ImmutableRunBundleVersion::new("v1")
                    .map_err(|_| "input.invalid")?,
                created_at: observed_at,
                sensitivity: ImmutableRunBundleSensitivity::Internal,
                redaction_required: true,
            },
        },
        observed_at,
        expires_at,
    };

    let mut executed = 0_u8;
    loop {
        let admitted =
            admit_trusted_host_two_step_no_write_operation(&backend, &registry, &request)
                .map_err(workflow_core::TrustedHostLocalApplicationFailure::code)?;
        match admitted {
            TrustedHostLocalApplicationAdmissionOutcome::Prepared(prepared) => {
                if executed == 2 {
                    return Err("operation.bound_exceeded");
                }
                let outcome = run_prepared(prepared)?;
                executed += 1;
                if !matches!(outcome, "operation.terminal" | "continuation.terminal") {
                    return Ok(outcome);
                }
                request.observed_at = Timestamp::now_utc();
                if request.observed_at >= request.expires_at {
                    return Err("operation.window_expired");
                }
            }
            TrustedHostLocalApplicationAdmissionOutcome::TerminalReplay => {
                return two_step_artifact_posture(&backend, &registry, &request, executed == 0);
            }
            TrustedHostLocalApplicationAdmissionOutcome::Waiting(reason) => {
                return Ok(match reason {
                    workflow_core::TrustedHostLocalApplicationWaitReason::Approval => {
                        "admission.waiting_approval"
                    }
                    workflow_core::TrustedHostLocalApplicationWaitReason::ExternalCondition => {
                        "admission.waiting_external_condition"
                    }
                });
            }
            TrustedHostLocalApplicationAdmissionOutcome::Blocked(reason) => {
                return Ok(match reason {
                    workflow_core::TrustedHostLocalApplicationBlockReason::UnsupportedWorkflow => {
                        "admission.blocked_unsupported_workflow"
                    }
                    workflow_core::TrustedHostLocalApplicationBlockReason::CurrentState => {
                        "admission.blocked_current_state"
                    }
                });
            }
            TrustedHostLocalApplicationAdmissionOutcome::Denied => return Ok("admission.denied"),
        }
    }
}

fn two_step_artifact_posture(
    backend: &SqliteStateBackend,
    registry: &TrustedHostTwoStepNoWriteProfileRegistry,
    request: &TrustedHostTwoStepNoWriteAdmissionRequest,
    replay: bool,
) -> Result<&'static str, &'static str> {
    match (
        workflow_core::persist_trusted_host_two_step_terminal_work_report_artifact(
            backend, registry, request,
        )
        .posture(),
        replay,
    ) {
        (TrustedHostTerminalWorkReportArtifactPosture::Persisted, false) => {
            Ok("operation.terminal_report_artifact_persisted")
        }
        (TrustedHostTerminalWorkReportArtifactPosture::AlreadyPersisted, false) => {
            Ok("operation.terminal_report_artifact_already_persisted")
        }
        (TrustedHostTerminalWorkReportArtifactPosture::GenerationFailed, false) => {
            Err("operation.terminal_report_generation_failed")
        }
        (TrustedHostTerminalWorkReportArtifactPosture::PersistenceFailed, false) => {
            Err("operation.terminal_report_artifact_persistence_failed")
        }
        (TrustedHostTerminalWorkReportArtifactPosture::Persisted, true) => {
            Ok("admission.terminal_replay_report_artifact_persisted")
        }
        (TrustedHostTerminalWorkReportArtifactPosture::AlreadyPersisted, true) => {
            Ok("admission.terminal_replay_report_artifact_already_persisted")
        }
        (TrustedHostTerminalWorkReportArtifactPosture::GenerationFailed, true) => {
            Err("admission.terminal_replay_report_generation_failed")
        }
        (TrustedHostTerminalWorkReportArtifactPosture::PersistenceFailed, true) => {
            Err("admission.terminal_replay_report_artifact_persistence_failed")
        }
    }
}

fn artifact_posture(
    backend: &SqliteStateBackend,
    profile: &workflow_core::ResolvedExplicitLocalCheckProfile,
    request: &TrustedHostLocalApplicationAdmissionRequest,
    replay: bool,
) -> Result<&'static str, &'static str> {
    match (
        persist_trusted_host_terminal_work_report_artifact(backend, profile, request).posture(),
        replay,
    ) {
        (TrustedHostTerminalWorkReportArtifactPosture::Persisted, false) => {
            Ok("operation.terminal_report_artifact_persisted")
        }
        (TrustedHostTerminalWorkReportArtifactPosture::AlreadyPersisted, false) => {
            Ok("operation.terminal_report_artifact_already_persisted")
        }
        (TrustedHostTerminalWorkReportArtifactPosture::GenerationFailed, false) => {
            Err("operation.terminal_report_generation_failed")
        }
        (TrustedHostTerminalWorkReportArtifactPosture::PersistenceFailed, false) => {
            Err("operation.terminal_report_artifact_persistence_failed")
        }
        (TrustedHostTerminalWorkReportArtifactPosture::Persisted, true) => {
            Ok("admission.terminal_replay_report_artifact_persisted")
        }
        (TrustedHostTerminalWorkReportArtifactPosture::AlreadyPersisted, true) => {
            Ok("admission.terminal_replay_report_artifact_already_persisted")
        }
        (TrustedHostTerminalWorkReportArtifactPosture::GenerationFailed, true) => {
            Err("admission.terminal_replay_report_generation_failed")
        }
        (TrustedHostTerminalWorkReportArtifactPosture::PersistenceFailed, true) => {
            Err("admission.terminal_replay_report_artifact_persistence_failed")
        }
    }
}

fn resolve_profile(
    arguments: &BTreeMap<String, String>,
    governance_project_root: &std::path::Path,
) -> Result<workflow_core::ResolvedExplicitLocalCheckProfile, &'static str> {
    match arguments.get("profile").map(String::as_str) {
        None | Some("docs-check") => {
            if arguments.contains_key("workflow-os-executable") {
                return Err("input.invalid");
            }
            let repository_root = path_argument(arguments, "repository-root")?;
            let npm_executable = path_argument(arguments, "npm-executable")?;
            let npm_cache_directory = arguments.get("npm-cache-directory").map(PathBuf::from);
            ExplicitLocalCheckProfileSelection::docs_check()
                .resolve_docs_check(npm_executable, repository_root, npm_cache_directory)
                .map_err(|_| "profile.invalid")
        }
        Some("workflow-os-project-validation") => {
            if arguments.contains_key("repository-root")
                || arguments.contains_key("npm-executable")
                || arguments.contains_key("npm-cache-directory")
            {
                return Err("input.invalid");
            }
            let executable = path_argument(arguments, "workflow-os-executable")?;
            ExplicitLocalCheckProfileSelection::workflow_os_project_validation()
                .resolve(executable, governance_project_root.to_path_buf())
                .map_err(|_| "profile.invalid")
        }
        Some(_) => Err("profile.invalid"),
    }
}

fn run_prepared(
    prepared: workflow_core::TrustedHostLocalApplicationPreparedSession<'_>,
) -> Result<&'static str, &'static str> {
    let outcome = LocalHostPreparedOperation::from_prepared(prepared)
        .run()
        .map_err(workflow_core::TrustedHostLocalApplicationFailure::code)?;
    Ok(match outcome {
        workflow_core::TrustedHostLocalApplicationOutcome::CanceledBeforeEntry => {
            "operation.canceled_before_entry"
        }
        workflow_core::TrustedHostLocalApplicationOutcome::EntryStopped(reason) => match reason {
            workflow_core::TrustedHostLocalApplicationEntryStopReason::AwaitCondition => {
                "operation.await_condition"
            }
            workflow_core::TrustedHostLocalApplicationEntryStopReason::Blocked => {
                "operation.blocked"
            }
            workflow_core::TrustedHostLocalApplicationEntryStopReason::Terminal => {
                "operation.terminal"
            }
        },
        workflow_core::TrustedHostLocalApplicationOutcome::ContinuationStopped(reason) => {
            match reason {
                workflow_core::TrustedHostLocalApplicationContinuationStopReason::Canceled => {
                    "continuation.canceled"
                }
                workflow_core::TrustedHostLocalApplicationContinuationStopReason::Blocked => {
                    "continuation.blocked"
                }
                workflow_core::TrustedHostLocalApplicationContinuationStopReason::Terminal => {
                    "continuation.terminal"
                }
                workflow_core::TrustedHostLocalApplicationContinuationStopReason::UnsupportedWait => {
                    "continuation.unsupported_wait"
                }
                workflow_core::TrustedHostLocalApplicationContinuationStopReason::WakeBudgetExhausted => {
                    "continuation.wake_budget_exhausted"
                }
            }
        }
    })
}

fn parse_arguments() -> Result<BTreeMap<String, String>, &'static str> {
    let mut arguments = env::args().skip(1);
    let mut parsed = BTreeMap::new();
    while let Some(flag) = arguments.next() {
        let Some(name) = flag.strip_prefix("--") else {
            return Err("input.invalid");
        };
        if parsed.contains_key(name) {
            return Err("input.invalid");
        }
        let value = arguments.next().ok_or("input.invalid")?;
        if value.starts_with("--") || value.is_empty() {
            return Err("input.invalid");
        }
        parsed.insert(name.to_owned(), value);
    }
    if parsed
        .keys()
        .any(|key| !ALLOWED_ARGUMENTS.contains(&key.as_str()))
        || REQUIRED_ARGUMENTS
            .iter()
            .any(|required| !parsed.contains_key(*required))
    {
        return Err("input.invalid");
    }
    if parsed.get("mode").map(String::as_str) != Some("two-step-no-write")
        && !parsed.contains_key("step-id")
    {
        return Err("input.invalid");
    }
    Ok(parsed)
}

fn argument<'a>(
    arguments: &'a BTreeMap<String, String>,
    name: &str,
) -> Result<&'a str, &'static str> {
    arguments
        .get(name)
        .map(String::as_str)
        .ok_or("input.invalid")
}

fn path_argument(
    arguments: &BTreeMap<String, String>,
    name: &str,
) -> Result<PathBuf, &'static str> {
    Ok(PathBuf::from(argument(arguments, name)?))
}

fn timestamp_argument(
    arguments: &BTreeMap<String, String>,
    name: &str,
) -> Result<Timestamp, &'static str> {
    Timestamp::parse_rfc3339(argument(arguments, name)?).map_err(|_| "input.invalid")
}
