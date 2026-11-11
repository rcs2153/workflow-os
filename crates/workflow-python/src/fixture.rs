//! The existing fixture-only Core path, without CLI text or subprocesses.
use super::{
    approval, bundle_inputs, error, execution, presentation, required, Operation, Request,
};
use serde_json::{json, Value};
use std::fs;
use workflow_core::{
    decide_github_repository_metadata_fixture_approval, execute_github_repository_metadata_fixture,
    load_project, EventLogStore, GitHubRepositoryMetadataFixtureExecutionRequest,
    LocalExecutionWithImmutableRunBundleRequest, SqliteStateBackend, StateBackend, Timestamp,
    WorkflowOsError, WorkflowRunId, WorkflowRunStatus,
};

pub(super) fn dispatch(request: &Request) -> Result<Value, WorkflowOsError> {
    let backend =
        SqliteStateBackend::open(request.state_dir.join("github-repository-metadata.sqlite3"))?;
    let mut posture = None;
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
            let loaded = load_project(&request.project_dir);
            let project = loaded
                .bundle
                .ok_or_else(|| error("python.project.invalid", "Project is unavailable"))?;
            let workflow = project
                .workflows
                .iter()
                .find(|w| w.definition.id == execution.workflow_id)
                .ok_or_else(|| error("python.workflow.missing", "Workflow is unavailable"))?;
            let [step] = workflow.definition.steps.as_slice() else {
                return Err(error(
                    "python.fixture.scope",
                    "Fixture mode requires exactly one metadata read step",
                ));
            };
            let fixture_json = if existing
                .as_ref()
                .is_some_and(|run| run.snapshot.status.is_terminal())
            {
                String::new()
            } else {
                read_fixture(request)?
            };
            let observed_at = Timestamp::now_utc();
            let result = execute_github_repository_metadata_fixture(
                &backend,
                &GitHubRepositoryMetadataFixtureExecutionRequest {
                    execution: LocalExecutionWithImmutableRunBundleRequest { execution, bundle },
                    selected_step_id: step.id.clone(),
                    fixture_json,
                    observed_at,
                    expires_at: Timestamp::from_offset_date_time(
                        observed_at.as_offset_date_time() + time::Duration::minutes(5),
                    ),
                },
            )?;
            posture = Some(format!("{:?}", result.posture()));
            result.run().clone()
        }
        Operation::Approve => {
            let fixture = if request.deny {
                None
            } else {
                Some(read_fixture(request)?)
            };
            decide_github_repository_metadata_fixture_approval(
                &backend,
                approval(request)?,
                fixture.as_deref(),
            )?
        }
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
            true,
            !matches!(request.operation, Operation::Inspect),
        )?)
    } else {
        None
    };
    Ok(
        json!({"run": run, "presentation": presentation, "outputs": [], "provider_posture": posture}),
    )
}

fn read_fixture(request: &Request) -> Result<String, WorkflowOsError> {
    fs::read_to_string(request.project_dir.join("fixtures/github/repository.json")).map_err(|_| {
        error(
            "python.fixture.unavailable",
            "Repository metadata fixture is missing or unreadable",
        )
    })
}
