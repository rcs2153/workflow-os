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
    NoWriteHostedExecutionProvider,
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
    backend.initialize_schema()?;

    if matches!(mode, ProcessMode::Worker | ProcessMode::WorkerOnce) {
        let worker = Arc::new(HostedWorker::new(
            backend,
            actor,
            Arc::new(NoWriteHostedExecutionProvider::new()?),
            Duration::from_secs(30),
        ));
        if mode == ProcessMode::WorkerOnce {
            print_worker_outcome(worker.run_once()?.is_some());
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
    let state = HostedApiState::new(
        backend,
        auth,
        format!("workflow-os-hosted/{}", env!("CARGO_PKG_VERSION")),
    )?;
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
        let current = Arc::clone(&worker);
        let outcome = tokio::task::spawn_blocking(move || current.run_once())
            .await
            .map_err(|_| {
                WorkflowOsError::invalid_state(
                    "hosted.worker.task.failed",
                    "hosted worker task failed",
                )
            })??;
        print_worker_outcome(outcome.is_some());
        tokio::select! {
            () = &mut shutdown => return Ok(()),
            () = tokio::time::sleep(Duration::from_millis(500)) => {}
        }
    }
}

fn print_worker_outcome(completed: bool) {
    println!(
        "worker_outcome: {}",
        if completed { "completed" } else { "idle" }
    );
}
