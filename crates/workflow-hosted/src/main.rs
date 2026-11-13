//! Single-tenant hosted alpha API and stateless worker process.

use std::env;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use postgres::Config;
use workflow_core::{
    ActorId, PostgresNoTlsConnectionFactory, PostgresStateBackend, WorkflowOsError,
};
use workflow_hosted::{
    hosted_router, HostedApiAuth, HostedApiState, HostedAuthTokenDigest, HostedWorker,
    HostedWorkerOutcome, NoWriteHostedExecutionProvider,
};

#[tokio::main]
async fn main() {
    if let Err(error) = run().await {
        eprintln!("error[{}]: {}", error.code(), error.message());
        std::process::exit(1);
    }
}

async fn run() -> Result<(), WorkflowOsError> {
    let mode = process_mode();
    let database_url = required_env("WORKFLOW_OS_HOSTED_DATABASE_URL")?;
    let actor = ActorId::new(required_env("WORKFLOW_OS_HOSTED_ACTOR")?)?;
    let config = database_url.parse::<Config>().map_err(|_| {
        WorkflowOsError::validation(
            "hosted.database.configuration.invalid",
            "hosted database configuration is invalid",
        )
    })?;
    let backend = PostgresStateBackend::new(Arc::new(PostgresNoTlsConnectionFactory::new(config)));
    let initialization_backend = backend.clone();
    tokio::task::spawn_blocking(move || initialization_backend.initialize_schema())
        .await
        .map_err(|_| {
            WorkflowOsError::invalid_state(
                "hosted.state.initialization_task.failed",
                "hosted state initialization task failed",
            )
        })??;

    if matches!(mode, ProcessMode::Worker | ProcessMode::WorkerOnce) {
        let worker = Arc::new(HostedWorker::new(
            backend,
            actor,
            Arc::new(NoWriteHostedExecutionProvider::new()?),
            Duration::from_secs(30),
        ));
        if mode == ProcessMode::WorkerOnce {
            print_worker_outcome(run_worker_once(worker).await?.as_ref());
        } else {
            run_worker_loop(worker).await?;
        }
        return Ok(());
    }

    let token = required_env("WORKFLOW_OS_HOSTED_TOKEN")?;
    let bind = env::var("WORKFLOW_OS_HOSTED_BIND")
        .unwrap_or_else(|_| "127.0.0.1:8080".to_owned())
        .parse::<SocketAddr>()
        .map_err(|_| {
            WorkflowOsError::validation("hosted.bind.invalid", "hosted bind address is invalid")
        })?;
    let auth = HostedApiAuth::new(HostedAuthTokenDigest::from_token(&token)?, actor);
    let mut state = HostedApiState::new(
        backend,
        auth,
        format!("workflow-os-hosted/{}", env!("CARGO_PKG_VERSION")),
        required_env("WORKFLOW_OS_HOSTED_PROJECT_ROOT")?,
    )?;
    let operator_token = env::var_os("WORKFLOW_OS_HOSTED_CONFLICT_OPERATOR_TOKEN");
    let operator_actor = env::var_os("WORKFLOW_OS_HOSTED_CONFLICT_OPERATOR_ACTOR");
    if operator_token.is_some() || operator_actor.is_some() {
        let operator = HostedApiAuth::new(
            HostedAuthTokenDigest::from_token(&required_env(
                "WORKFLOW_OS_HOSTED_CONFLICT_OPERATOR_TOKEN",
            )?)?,
            ActorId::new(required_env("WORKFLOW_OS_HOSTED_CONFLICT_OPERATOR_ACTOR")?)?,
        );
        state = state.with_conflict_operator(operator)?;
    }
    let listener = tokio::net::TcpListener::bind(bind).await.map_err(|_| {
        WorkflowOsError::invalid_state(
            "hosted.listener.unavailable",
            "hosted listener is unavailable",
        )
    })?;
    axum::serve(listener, hosted_router(state))
        .with_graceful_shutdown(shutdown_signal())
        .await
        .map_err(|_| WorkflowOsError::invalid_state("hosted.server.failed", "hosted server failed"))
}

fn required_env(name: &'static str) -> Result<String, WorkflowOsError> {
    env::var(name).map_err(|_| {
        WorkflowOsError::validation(
            "hosted.configuration.missing",
            "required hosted configuration is missing",
        )
    })
}

async fn shutdown_signal() {
    let _ = tokio::signal::ctrl_c().await;
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum ProcessMode {
    Api,
    Worker,
    WorkerOnce,
}

fn process_mode() -> ProcessMode {
    let mut mode = ProcessMode::Api;
    for argument in env::args().skip(1) {
        mode = match argument.as_str() {
            "--worker" => ProcessMode::Worker,
            "--worker-once" => ProcessMode::WorkerOnce,
            _ => mode,
        };
    }
    mode
}

async fn run_worker_loop(worker: Arc<HostedWorker>) -> Result<(), WorkflowOsError> {
    let mut shutdown = Box::pin(shutdown_signal());
    loop {
        print_worker_outcome(run_worker_once(Arc::clone(&worker)).await?.as_ref());
        tokio::select! {
            () = &mut shutdown => return Ok(()),
            () = tokio::time::sleep(Duration::from_millis(500)) => {}
        }
    }
}

async fn run_worker_once(
    worker: Arc<HostedWorker>,
) -> Result<Option<HostedWorkerOutcome>, WorkflowOsError> {
    let outcome = tokio::task::spawn_blocking(move || worker.run_once())
        .await
        .map_err(|_| {
            WorkflowOsError::invalid_state("hosted.worker.task.failed", "hosted worker task failed")
        })??;
    Ok(outcome)
}

fn worker_outcome_label(outcome: Option<&HostedWorkerOutcome>) -> &'static str {
    match outcome {
        None => "idle",
        Some(HostedWorkerOutcome::Receipt(receipt)) => match receipt.status() {
            workflow_core::HostedExecutionStatus::Completed => "completed",
            workflow_core::HostedExecutionStatus::Failed => "failed",
            workflow_core::HostedExecutionStatus::Canceled => "canceled",
            workflow_core::HostedExecutionStatus::Ambiguous => "reconciliation_required",
        },
        Some(HostedWorkerOutcome::RejectedBeforeStart) => "rejected_before_start",
        Some(HostedWorkerOutcome::ReconciliationRequired) => "reconciliation_required",
        Some(HostedWorkerOutcome::CanceledBeforeStart) => "canceled_before_start",
    }
}

fn print_worker_outcome(outcome: Option<&HostedWorkerOutcome>) {
    println!("worker_outcome: {}", worker_outcome_label(outcome));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn non_receipt_outcomes_never_claim_completion() {
        assert_eq!(worker_outcome_label(None), "idle");
        assert_eq!(
            worker_outcome_label(Some(&HostedWorkerOutcome::ReconciliationRequired)),
            "reconciliation_required"
        );
        assert_eq!(
            worker_outcome_label(Some(&HostedWorkerOutcome::RejectedBeforeStart)),
            "rejected_before_start"
        );
        assert_eq!(
            worker_outcome_label(Some(&HostedWorkerOutcome::CanceledBeforeStart)),
            "canceled_before_start"
        );
    }
}
