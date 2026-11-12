//! Experimental Python embedding. Rust Core owns all policy and state transitions.
#![deny(unsafe_code)]

mod fixture;
mod presentation;

use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet};
use std::fs::{self, OpenOptions};
use std::path::PathBuf;
use std::rc::Rc;
use std::time::Duration;

use fs2::FileExt;
use pyo3::exceptions::PyException;
use pyo3::prelude::*;
use serde::Deserialize;
use serde_json::{json, Value};
use workflow_core::{
    execute_with_immutable_run_bundle, load_project, validate_loaded_project, ActorId,
    ApprovalDecisionKind, ApprovalPresentationId, CorrelationId, EventLogStore,
    ImmutableRunBundleId, ImmutableRunBundleSensitivity, ImmutableRunBundleStore,
    ImmutableRunBundleVersion, LocalApprovalDecisionRequest,
    LocalApprovalPresentationDecisionRequest, LocalApprovalPresentationProof,
    LocalExecutionBeforeSkillInvocationCheckpointInputs, LocalExecutionImmutableRunBundleInputs,
    LocalExecutionRequest, LocalExecutionWithImmutableRunBundleRequest, LocalExecutor,
    LocalSkillRegistry, SkillHandler, SkillId, SkillInput, SkillOutput, SkillVersion,
    SqliteStateBackend, StateBackend, Timestamp, WorkflowId, WorkflowOsError, WorkflowRun,
    WorkflowRunId, WorkflowRunStatus,
};

pyo3::create_exception!(
    _native,
    NativeError,
    PyException,
    "Structured native kernel error: (stable code, bounded message)."
);

type Outputs = Rc<RefCell<Vec<Value>>>;
type Handlers = Vec<(String, String, Py<PyAny>)>;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Request {
    project_dir: PathBuf,
    state_dir: PathBuf,
    mode: Mode,
    operation: Operation,
    workflow_id: Option<String>,
    run_id: Option<String>,
    approval_id: Option<String>,
    presentation_id: Option<String>,
    actor: String,
    reason: Option<String>,
    #[serde(default)]
    deny: bool,
}

#[derive(Clone, Copy, Deserialize)]
#[serde(rename_all = "snake_case")]
enum Mode {
    Local,
    GithubRepositoryMetadataFixture,
}

#[derive(Clone, Copy, Deserialize)]
#[serde(rename_all = "snake_case")]
enum Operation {
    Validate,
    Run,
    Approve,
    Inspect,
}

fn error(code: &str, message: &str) -> WorkflowOsError {
    WorkflowOsError::validation(code, message)
}

fn required<'a>(value: Option<&'a str>, name: &str) -> Result<&'a str, WorkflowOsError> {
    value.filter(|v| !v.trim().is_empty()).ok_or_else(|| {
        error(
            "python.request.missing_field",
            &format!("missing required {name}"),
        )
    })
}

fn execution(request: &Request) -> Result<LocalExecutionRequest, WorkflowOsError> {
    Ok(LocalExecutionRequest {
        project_root: request.project_dir.clone(),
        workflow_id: WorkflowId::new(required(request.workflow_id.as_deref(), "workflow_id")?)?,
        run_id: request
            .run_id
            .as_deref()
            .map(WorkflowRunId::new)
            .transpose()?,
        correlation_id: CorrelationId::generate(),
        actor: ActorId::new(&request.actor)?,
        before_skill_invocation_checkpoints:
            LocalExecutionBeforeSkillInvocationCheckpointInputs::default(),
        before_skill_invocation_hook: None,
        side_effect_events: Vec::new(),
        side_effect_lifecycle_events: Vec::new(),
    })
}

fn approval(
    request: &Request,
) -> Result<LocalApprovalPresentationDecisionRequest, WorkflowOsError> {
    Ok(LocalApprovalPresentationDecisionRequest {
        approval: LocalApprovalDecisionRequest {
            project_root: request.project_dir.clone(),
            run_id: WorkflowRunId::new(required(request.run_id.as_deref(), "run_id")?)?,
            approval_id: required(request.approval_id.as_deref(), "approval_id")?.to_owned(),
            decision: if request.deny {
                ApprovalDecisionKind::Denied
            } else {
                ApprovalDecisionKind::Granted
            },
            actor: ActorId::new(&request.actor)?,
            reason: required(request.reason.as_deref(), "reason")?.to_owned(),
            correlation_id: CorrelationId::generate(),
        },
        proof: LocalApprovalPresentationProof::PresentationId(ApprovalPresentationId::new(
            required(request.presentation_id.as_deref(), "presentation_id")?,
        )?),
        max_presentation_age: Some(Duration::from_secs(24 * 60 * 60)),
    })
}

struct PythonHandler {
    callback: Py<PyAny>,
    outputs: Outputs,
}

impl SkillHandler for PythonHandler {
    fn invoke(&self, input: SkillInput) -> Result<SkillOutput, WorkflowOsError> {
        // Python exceptions may contain credentials or provider payloads. Never persist them.
        let values = Python::attach(|py| {
            self.callback
                .bind(py)
                .call1((input.values.clone(),))?
                .extract::<BTreeMap<String, String>>()
        })
        .map_err(|_| {
            error(
                "python.callback.failed",
                "Python callback failed or returned an invalid string mapping",
            )
        })?;
        self.outputs
            .borrow_mut()
            .push(json!({"step_id": input.step_id, "values": values}));
        Ok(SkillOutput::new(values, None))
    }
}

fn bundle_inputs<S: ImmutableRunBundleStore>(
    store: &S,
    run_id: &WorkflowRunId,
    existing: Option<&WorkflowRun>,
) -> Result<LocalExecutionImmutableRunBundleInputs, WorkflowOsError> {
    if let Some(run) = existing {
        let binding = run
            .snapshot
            .identity
            .immutable_run_bundle
            .as_ref()
            .ok_or_else(|| {
                error(
                    "python.bundle.missing",
                    "SDK requires an immutable run bundle",
                )
            })?;
        let stored = store.read_exact_bundle(run_id, binding.bundle_id())?;
        return Ok(LocalExecutionImmutableRunBundleInputs {
            bundle_id: stored.manifest().bundle_id().clone(),
            bundle_version: stored.manifest().bundle_version().clone(),
            created_at: *stored.manifest().created_at(),
            sensitivity: stored.manifest().sensitivity(),
            redaction_required: stored.manifest().redaction_required(),
        });
    }
    Ok(LocalExecutionImmutableRunBundleInputs {
        bundle_id: ImmutableRunBundleId::new(format!("bundle/{run_id}"))?,
        bundle_version: ImmutableRunBundleVersion::new("v1")?,
        created_at: Timestamp::now_utc(),
        sensitivity: ImmutableRunBundleSensitivity::Internal,
        redaction_required: true,
    })
}

fn local(request: &Request, handlers: Handlers) -> Result<Value, WorkflowOsError> {
    let backend = open_state(request, "local.sqlite3")?;
    let outputs: Outputs = Rc::default();
    let mut registry = LocalSkillRegistry::new();
    let mut registered = BTreeSet::new();
    for (id, version, callback) in handlers {
        if !id.starts_with("local/") || !registered.insert((id.clone(), version.clone())) {
            return Err(error(
                "python.handler.registration",
                "Callbacks must have unique local/ skill IDs and versions",
            ));
        }
        registry.register(
            SkillId::new(id)?,
            SkillVersion::new(version)?,
            Box::new(PythonHandler {
                callback,
                outputs: Rc::clone(&outputs),
            }),
        );
    }
    let executor = LocalExecutor::new(&backend, &registry);
    let run = match request.operation {
        Operation::Run => {
            let mut execution = execution(request)?;
            let run_id = execution
                .run_id
                .clone()
                .unwrap_or_else(WorkflowRunId::generate);
            execution.run_id = Some(run_id.clone());
            let existing = if backend.read_events(&run_id)?.is_empty() {
                None
            } else {
                Some(backend.rehydrate_run(&run_id)?)
            };
            let bundle = bundle_inputs(&backend, &run_id, existing.as_ref())?;
            execute_with_immutable_run_bundle(
                &executor,
                &backend,
                &LocalExecutionWithImmutableRunBundleRequest { execution, bundle },
            )?
            .into_parts()
            .0
        }
        Operation::Approve => executor.decide_approval_with_presentation(approval(request)?)?,
        Operation::Inspect => backend.rehydrate_run(&WorkflowRunId::new(required(
            request.run_id.as_deref(),
            "run_id",
        )?)?)?,
        Operation::Validate => return Ok(json!({"valid": true})),
    };
    let presentation = if run.snapshot.status == WorkflowRunStatus::WaitingForApproval {
        Some(presentation::persist(
            &backend,
            &run,
            false,
            !matches!(request.operation, Operation::Inspect),
        )?)
    } else {
        None
    };
    Ok(
        json!({"run": run, "presentation": presentation, "outputs": *outputs.borrow(), "provider_posture": null}),
    )
}

fn open_state(request: &Request, filename: &str) -> Result<SqliteStateBackend, WorkflowOsError> {
    let path = request.state_dir.join(filename);
    if matches!(request.operation, Operation::Inspect) {
        SqliteStateBackend::open_read_only(path)
    } else {
        SqliteStateBackend::open(path)
    }
}

fn dispatch(request: &Request, handlers: Handlers) -> Result<Value, WorkflowOsError> {
    // Reject unsupported governance modes rather than silently using a weaker executor.
    if !matches!(request.operation, Operation::Inspect) {
        let loaded = load_project(&request.project_dir);
        let validation = validate_loaded_project(&loaded);
        if validation.has_errors() {
            return Err(error("python.project.invalid", "Project validation failed"));
        }
        if matches!(request.mode, Mode::Local)
            && loaded
                .bundle
                .as_ref()
                .is_some_and(|bundle| bundle.manifest.definition.governance.is_some())
        {
            return Err(error("python.governance.unsupported", "Local Python callbacks do not support project governance profiles; use an explicitly supported composition"));
        }
    }
    if matches!(request.operation, Operation::Validate) {
        return Ok(json!({"valid": true}));
    }
    match request.mode {
        Mode::Local => local(request, handlers),
        Mode::GithubRepositoryMetadataFixture => {
            if !handlers.is_empty() {
                return Err(error(
                    "python.fixture.callbacks",
                    "Fixture mode does not accept Python callbacks",
                ));
            }
            fixture::dispatch(request)
        }
    }
}

#[pyfunction]
fn request(payload: &str, handlers: Handlers) -> PyResult<String> {
    let input: Request = serde_json::from_str(payload)
        .map_err(|_| NativeError::new_err(("python.request.invalid", "Invalid SDK request")))?;
    let _lock = if matches!(input.operation, Operation::Run | Operation::Approve) {
        Some(lock_writer(&input.state_dir)?)
    } else {
        None
    };
    let output = dispatch(&input, handlers)
        .map_err(|err| NativeError::new_err((err.code().to_owned(), err.to_string())))?;
    serde_json::to_string(&output).map_err(|_| {
        NativeError::new_err(("python.result.invalid", "Cannot serialize kernel result"))
    })
}

fn lock_writer(state_dir: &std::path::Path) -> PyResult<fs::File> {
    // Nonblocking writer acquisition also rejects reentrant callback writes.
    fs::create_dir_all(state_dir).map_err(|_| {
        NativeError::new_err((
            "python.state.unavailable",
            "Cannot open SDK state directory",
        ))
    })?;
    let lock = OpenOptions::new()
        .create(true)
        .truncate(false)
        .read(true)
        .write(true)
        .open(state_dir.join(".python-sdk.lock"))
        .map_err(|_| {
            NativeError::new_err(("python.state.unavailable", "Cannot open SDK state lock"))
        })?;
    lock.try_lock_exclusive().map_err(|_| {
        NativeError::new_err((
            "python.state.busy",
            "Another SDK operation is active; concurrent and reentrant calls are rejected",
        ))
    })?;
    Ok(lock)
}

#[pymodule]
fn _native(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add("NativeError", module.py().get_type::<NativeError>())?;
    module.add_function(wrap_pyfunction!(request, module)?)?;
    Ok(())
}
