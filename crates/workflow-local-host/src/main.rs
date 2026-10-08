#![deny(unsafe_code)]
#![doc = "Unpublished foreground entry point for one explicit trusted-host operation."]

use std::collections::BTreeMap;
use std::env;
use std::path::PathBuf;
use std::process::ExitCode;

use workflow_core::{
    admit_trusted_host_local_application_operation, ActorId, CorrelationId,
    ExplicitLocalCheckProfileSelection, ImmutableRunBundleId, ImmutableRunBundleSensitivity,
    ImmutableRunBundleVersion, LocalExecutionBeforeSkillInvocationCheckpointInputs,
    LocalExecutionImmutableRunBundleInputs, LocalExecutionRequest,
    LocalExecutionWithImmutableRunBundleRequest, SqliteStateBackend, StepId, Timestamp,
    TrustedHostLocalApplicationAdmissionOutcome, TrustedHostLocalApplicationAdmissionRequest,
    WorkflowId, WorkflowRunId,
};
use workflow_local_host::LocalHostPreparedOperation;

const ALLOWED_ARGUMENTS: [&str; 13] = [
    "governance-project-root",
    "repository-root",
    "state-database",
    "npm-executable",
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
const REQUIRED_ARGUMENTS: [&str; 12] = [
    "governance-project-root",
    "repository-root",
    "state-database",
    "npm-executable",
    "workflow-id",
    "step-id",
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
    let governance_project_root = path_argument(&arguments, "governance-project-root")?;
    let repository_root = path_argument(&arguments, "repository-root")?;
    let npm_executable = path_argument(&arguments, "npm-executable")?;
    let npm_cache_directory = arguments.get("npm-cache-directory").map(PathBuf::from);
    let profile = ExplicitLocalCheckProfileSelection::docs_check()
        .resolve_docs_check(npm_executable, repository_root, npm_cache_directory)
        .map_err(|_| "profile.invalid")?;
    let backend = SqliteStateBackend::open(path_argument(&arguments, "state-database")?)
        .map_err(|_| "state.unavailable")?;
    let observed_at = timestamp_argument(&arguments, "observed-at")?;
    let expires_at = timestamp_argument(&arguments, "expires-at")?;
    if expires_at <= observed_at {
        return Err("input.invalid");
    }
    let request = TrustedHostLocalApplicationAdmissionRequest {
        execution: LocalExecutionWithImmutableRunBundleRequest {
            execution: LocalExecutionRequest {
                project_root: governance_project_root,
                workflow_id: WorkflowId::new(argument(&arguments, "workflow-id")?)
                    .map_err(|_| "input.invalid")?,
                run_id: Some(
                    WorkflowRunId::new(argument(&arguments, "run-id")?)
                        .map_err(|_| "input.invalid")?,
                ),
                correlation_id: CorrelationId::new(argument(&arguments, "correlation-id")?)
                    .map_err(|_| "input.invalid")?,
                actor: ActorId::new(argument(&arguments, "actor")?).map_err(|_| "input.invalid")?,
                before_skill_invocation_checkpoints:
                    LocalExecutionBeforeSkillInvocationCheckpointInputs::default(),
                before_skill_invocation_hook: None,
                side_effect_events: Vec::new(),
                side_effect_lifecycle_events: Vec::new(),
            },
            bundle: LocalExecutionImmutableRunBundleInputs {
                bundle_id: ImmutableRunBundleId::new(argument(&arguments, "bundle-id")?)
                    .map_err(|_| "input.invalid")?,
                bundle_version: ImmutableRunBundleVersion::new("v1")
                    .map_err(|_| "input.invalid")?,
                created_at: observed_at,
                sensitivity: ImmutableRunBundleSensitivity::Internal,
                redaction_required: true,
            },
        },
        selected_step_id: StepId::new(argument(&arguments, "step-id")?)
            .map_err(|_| "input.invalid")?,
        observed_at,
        expires_at,
    };

    let admitted = admit_trusted_host_local_application_operation(&backend, &profile, &request)
        .map_err(workflow_core::TrustedHostLocalApplicationFailure::code)?;
    match admitted {
        TrustedHostLocalApplicationAdmissionOutcome::Prepared(prepared) => run_prepared(prepared),
        TrustedHostLocalApplicationAdmissionOutcome::Waiting(reason) => Ok(match reason {
            workflow_core::TrustedHostLocalApplicationWaitReason::Approval => {
                "admission.waiting_approval"
            }
            workflow_core::TrustedHostLocalApplicationWaitReason::ExternalCondition => {
                "admission.waiting_external_condition"
            }
        }),
        TrustedHostLocalApplicationAdmissionOutcome::TerminalReplay => {
            Ok("admission.terminal_replay")
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
