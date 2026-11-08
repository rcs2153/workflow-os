use std::os::unix::fs::DirBuilderExt;
use std::path::Path;
use std::process::{Command, Output};

use super::*;
use crate::authorized_execution_continuity_state::internal::TrustedTimeObservation;
use crate::dispatch_reservation::DispatchReservationOperationId;
use crate::provider_comment_transport::installation_notification::tests::exercise_prepared_notification;
use crate::provider_comment_transport::tests::{with_independent_inventory, InventoryTrial};
use crate::provider_write::GitHubCommentContinuationAction;
use crate::sqlite_state::dispatch_reservation_store::InstallationCommentRecoveryPosture;
use crate::{
    GitHubPullRequestCommentTarget, GitHubPullRequestCommentWriteMode, SideEffectId,
    SideEffectLifecycleState, SideEffectRecordStore,
};

const ACCEPTED_BEFORE_ACK_EXIT: i32 = 77;
const TRIAL_ROOT: &str = "WORKFLOW_OS_INSTALLATION_RECOVERY_TRIAL";

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Handoff {
    database: PathBuf,
    operation: DispatchReservationOperationId,
    run: WorkflowRunId,
    step: StepId,
    bundle: SpecContentHash,
    actor: ActorId,
    effect: SideEffectId,
    target: GitHubPullRequestCommentTarget,
    body: String,
    idempotency_key: IdempotencyKey,
    received_at: TrustedTimeObservation,
}

impl Handoff {
    fn read(root: &Path) -> Self {
        let handoff: Self = serde_json::from_slice(
            &fs::read(root.join("handoff.json")).expect("fixture handoff bytes"),
        )
        .expect("typed fixture handoff");
        assert_eq!(handoff.database.parent(), Some(root));
        handoff
    }

    fn action(&self) -> GitHubCommentContinuationAction<'_> {
        GitHubCommentContinuationAction {
            run: &self.run,
            step: &self.step,
            bundle_root: &self.bundle,
            original_actor: &self.actor,
            side_effect: &self.effect,
            target: &self.target,
            body: &self.body,
            idempotency_key: &self.idempotency_key,
            mode: GitHubPullRequestCommentWriteMode::LiveSandbox,
        }
    }

    fn provider_comment(&self) -> String {
        let provider = Connection::open_with_flags(
            self.database.with_extension("provider.sqlite3"),
            rusqlite::OpenFlags::SQLITE_OPEN_READ_ONLY,
        )
        .expect("independent provider ledger");
        let (count, id, body): (u32, i64, String) = provider
            .query_row(
                "SELECT (SELECT COUNT(*) FROM accepted), id, body FROM accepted",
                [],
                |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
            )
            .expect("accepted fixture effect");
        assert_eq!((count, id), (1, 22));
        body
    }
}

pub(super) fn interrupt_after_provider_acceptance(
    path: &Path,
    operation: &DispatchReservationOperationId,
    action: GitHubCommentContinuationAction<'_>,
) -> ! {
    let root = trial_root();
    assert_eq!(path.parent(), Some(root.as_path()));
    let handoff = Handoff {
        database: path.to_owned(),
        operation: operation.clone(),
        run: action.run.clone(),
        step: action.step.clone(),
        bundle: action.bundle_root.clone(),
        actor: action.original_actor.clone(),
        effect: action.side_effect.clone(),
        target: action.target.clone(),
        body: action.body.to_owned(),
        idempotency_key: action.idempotency_key.clone(),
        received_at: crate::sqlite_state::continuity_store::observe_continuity_trusted_time()
            .expect("fixture receiver clock after acceptance"),
    };
    fs::write(
        root.join("handoff.json"),
        serde_json::to_vec(&handoff).expect("fixture handoff"),
    )
    .expect("persist fixture handoff before abrupt exit");
    std::process::exit(ACCEPTED_BEFORE_ACK_EXIT);
}

struct TrialRoot(PathBuf);

impl Drop for TrialRoot {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn trial_root() -> PathBuf {
    let root = PathBuf::from(std::env::var_os(TRIAL_ROOT).expect("private fixture root"));
    assert_eq!(std::env::temp_dir(), root);
    root
}

fn run_child(root: &Path, test: &str) -> Output {
    let (_, module) = module_path!().split_once("::").expect("test module");
    Command::new(std::env::current_exe().expect("fixture test binary"))
        .args([
            "--ignored",
            "--exact",
            &format!("{module}::{test}"),
            "--nocapture",
        ])
        .env(TRIAL_ROOT, root)
        .env("TMPDIR", root)
        .output()
        .expect("run separate Core process")
}

#[test]
fn live_installation_comment_recovers_across_process_replacement() {
    let id = NEXT_FIXTURE.fetch_add(1, Ordering::Relaxed);
    let path = std::env::temp_dir().join(format!(
        "workflow-os-recovery-process-{}-{id}",
        std::process::id()
    ));
    fs::DirBuilder::new()
        .mode(0o700)
        .create(&path)
        .expect("create fixture root");
    let root = TrialRoot(path);
    let producer = run_child(&root.0, "producer_child");
    assert_eq!(
        producer.status.code(),
        Some(ACCEPTED_BEFORE_ACK_EXIT),
        "producer did not stop at acknowledgement loss: {} {}",
        String::from_utf8_lossy(&producer.stdout),
        String::from_utf8_lossy(&producer.stderr),
    );
    let handoff = Handoff::read(&root.0);
    let comment = handoff.provider_comment();
    let backend =
        SqliteStateBackend::open_read_only(&handoff.database).expect("open interrupted Core");
    let history_before = backend.read_events(&handoff.run).expect("attempt history");
    assert_eq!(
        backend
            .inspect_installation_comment_recovery(
                &handoff.run,
                &handoff.operation,
                &handoff.effect
            )
            .expect("unresolved effect after producer exit"),
        InstallationCommentRecoveryPosture::ReconciliationRequired,
    );
    drop(backend);

    let replacement = run_child(&root.0, "replacement_child");
    assert!(
        replacement.status.success()
            && String::from_utf8_lossy(&replacement.stdout).contains("1 passed"),
        "replacement did not recover: {} {}",
        String::from_utf8_lossy(&replacement.stdout),
        String::from_utf8_lossy(&replacement.stderr),
    );
    assert_eq!(handoff.provider_comment(), comment);
    let backend =
        SqliteStateBackend::open_read_only(&handoff.database).expect("reopen completed Core");
    assert_eq!(
        backend
            .recover_installation_comment(
                &handoff.operation,
                handoff.action(),
                || panic!("terminal replay cannot load notification"),
                || panic!("terminal replay cannot load reader")
            )
            .expect("terminal proof replay"),
        InstallationCommentRecoveryPosture::ObservationReconciled,
    );
    let effect = backend
        .read_side_effect_record(&handoff.effect)
        .expect("read exact effect")
        .expect("effect");
    assert_eq!(
        effect.lifecycle_state(),
        SideEffectLifecycleState::Completed
    );
    let history = backend
        .read_events(&handoff.run)
        .expect("recovered history");
    assert_eq!(history.len(), history_before.len() + 1);
    assert_eq!(&history[..history_before.len()], history_before.as_slice());
    assert!(history.last().is_some_and(|event| matches!(
        event.kind,
        WorkflowRunEventKind::SideEffectCompleted(_)
    ) && event
        .event_id
        .as_str()
        .starts_with("installation-comment-observed/")));
}

#[test]
#[ignore = "launched by the isolated process replacement fixture"]
fn producer_child() {
    let _ = trial_root();
    exercise_live_grant_reservation(LiveGrantTrial::InstallationClaim(
        InstallationClaimTrial::Coordinator(CoordinatorTrial::ProcessRecovery),
    ));
    panic!("producer did not interrupt");
}

#[test]
#[ignore = "launched by the isolated process replacement fixture"]
fn replacement_child() {
    let handoff = Handoff::read(&trial_root());
    let body = handoff.provider_comment();
    let backend = SqliteStateBackend::open(&handoff.database).expect("replace Core");
    let connection = backend
        .existing_read_only_connection()
        .expect("preparation connection");
    let payload: String = connection
        .query_row(
            "SELECT payload FROM installation_send_preparations WHERE operation_id=?1",
            [handoff.operation.as_str()],
            |row| row.get(0),
        )
        .expect("exact preparation");
    let prepared: serde_json::Value = serde_json::from_str(&payload).expect("prepared record");
    let scope = serde_json::from_value(prepared["scope"].clone()).expect("writer scope");
    let earliest = serde_json::from_value(prepared["clock"].clone()).expect("preparation clock");
    exercise_prepared_notification(
        &body,
        &scope,
        &earliest,
        &handoff.received_at,
        serde_json::from_value(prepared["deadline"].clone()).expect("deadline"),
        |receiver, notification| {
            backend
                .verify_prepared_installation_notification(
                    &handoff.operation,
                    handoff.action(),
                    receiver,
                    notification,
                )
                .expect("fixture notification verification")
        },
        |receiver, notification, _| {
            with_independent_inventory(InventoryTrial::Exact, &body, &scope, |reader| {
                assert_eq!(
                    backend
                        .recover_installation_comment(
                            &handoff.operation,
                            handoff.action(),
                            || Some((receiver, notification)),
                            || Some(reader)
                        )
                        .expect("reconcile replacement"),
                    InstallationCommentRecoveryPosture::ObservationReconciled,
                );
            });
        },
    );
}
