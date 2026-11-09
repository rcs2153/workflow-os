use std::io::Read;
use std::os::unix::fs::{DirBuilderExt, OpenOptionsExt};
use std::path::Path;
use std::process::{Command, Output};

use super::*;
use crate::authorized_execution_continuity_state::internal::TrustedTimeObservation;
use crate::dispatch_reservation::DispatchReservationOperationId;
use crate::provider_comment_transport::installation_exchange::{
    reader::ReviewedReadInstallation, tests::with_scoped_recovery_reader,
};
use crate::provider_comment_transport::installation_notification::tests::exercise_prepared_notification;
use crate::provider_comment_transport::installation_signer_channel::rehearsal::executable_digest;
use crate::provider_comment_transport::InstallationObserverProfile;
use crate::provider_write::GitHubCommentContinuationAction;
use crate::sqlite_state::dispatch_reservation_store::InstallationCommentRecoveryPosture;
use crate::AuthorizedExecutionWindowId;
use crate::{
    GitHubPullRequestCommentTarget, GitHubPullRequestCommentWriteMode, SideEffectId,
    SideEffectLifecycleState, SideEffectRecordStore,
};

const ACCEPTED_BEFORE_ACK_EXIT: i32 = 77;
const TRIAL_ROOT: &str = "WORKFLOW_OS_INSTALLATION_RECOVERY_TRIAL";
const READER_DEADLINE: &str = "WORKFLOW_OS_INSTALLATION_RECOVERY_READER_DEADLINE";
const LAUNCH_DIGEST: &str = "WORKFLOW_OS_INSTALLATION_RECOVERY_LAUNCH_DIGEST";
const LAUNCH_FILE: &str = "operator-launch.json";
const LAUNCH_DENIED_EXIT: i32 = 78;
const MAX_LAUNCH_BYTES: u64 = 4096;
const HOSTED_TRIAL: &str = "operator";

pub(super) fn hosted_operator_trial() -> bool {
    std::env::var("WORKFLOW_OS_SIGNER_TRIAL").as_deref() == Ok(HOSTED_TRIAL)
}

pub(super) fn exchange_hosted_operator(
    backend: &SqliteStateBackend,
    request: &crate::dispatch_reservation::DispatchReservationRequest,
    admission: crate::sqlite_state::live_sandbox_grant_store::LiveGrantAdmission<'_>,
    reviewed: crate::provider_comment_transport::installation_exchange::ReviewedInstallation<'_>,
    claim: crate::sqlite_state::dispatch_reservation_store::InstallationIssuanceClaim,
) -> Result<
    Option<crate::sqlite_state::dispatch_reservation_store::InstallationExchangedScope>,
    WorkflowOsError,
> {
    use crate::provider_comment_transport::installation_signer_channel::{
        decision::InstallationSignerDecisionReceiver,
        rehearsal::{
            accept_socket, bind_socket, connect_socket, exchange_operator_assertion, fixture_root,
            POLICY,
        },
        InstallationSignerChannel,
    };
    let root = fixture_root();
    let listener = bind_socket(&root.join("core-socket/decision"));
    let channel = InstallationSignerChannel::authenticate(
        connect_socket(&root.join("signer-socket/assertion")),
        &POLICY,
    )?;
    std::thread::scope(|threads| {
        let authority = threads.spawn(|| {
            InstallationSignerDecisionReceiver::authenticate(accept_socket(&listener), &POLICY)?
                .serve_once(|wire| {
                    backend.admit_installation_signer_request(request, admission, reviewed, wire)
                })
        });
        let exchanged = backend.exchange_with_installation_signer_using(
            request,
            admission,
            reviewed,
            claim,
            channel,
            exchange_operator_assertion,
        );
        authority.join().expect("Core authority worker")?;
        exchanged
    })
}

fn reader_deadline() -> Timestamp {
    Timestamp::parse_rfc3339(&std::env::var(READER_DEADLINE).expect("reviewed reader deadline"))
        .expect("valid fixture deadline")
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct OperatorLaunch {
    executable: SpecContentHash,
    producer_entry: String,
    replacement_entry: String,
    reader_deadline: Timestamp,
}

fn checked_launch(
    root: &Path,
    entry: &str,
    executable: &SpecContentHash,
    expected: &str,
    deadline: Timestamp,
) -> Option<SpecContentHash> {
    let file = fs::OpenOptions::new()
        .read(true)
        .custom_flags(nix::libc::O_NOFOLLOW | nix::libc::O_NONBLOCK)
        .open(root.join(LAUNCH_FILE))
        .ok()?;
    if !file.metadata().ok()?.is_file() {
        return None;
    }
    let mut bytes = Vec::new();
    file.take(MAX_LAUNCH_BYTES + 1)
        .read_to_end(&mut bytes)
        .ok()?;
    if bytes.len() as u64 > MAX_LAUNCH_BYTES {
        return None;
    }
    let actual = SpecContentHash::from_bytes(&bytes);
    if actual.as_str() != expected {
        return None;
    }
    let launch: OperatorLaunch = serde_json::from_slice(&bytes).ok()?;
    let reviewed_entry = match entry {
        "producer_child" => &launch.producer_entry,
        "replacement_child" => &launch.replacement_entry,
        _ => return None,
    };
    (reviewed_entry == entry
        && &launch.executable == executable
        && launch.reader_deadline == deadline)
        .then_some(actual)
}

fn current_launch(root: &Path, entry: &str) -> Option<SpecContentHash> {
    checked_launch(
        root,
        entry,
        &executable_digest(),
        &std::env::var(LAUNCH_DIGEST).ok()?,
        Timestamp::parse_rfc3339(&std::env::var(READER_DEADLINE).ok()?).ok()?,
    )
}

pub(super) fn dispatch_launch_commitment(invocation: &SpecContentHash) -> SpecContentHash {
    if hosted_operator_trial() {
        let launch = crate::provider_comment_transport::installation_signer_channel::rehearsal::launch_commitment();
        return window_launch_commitment(&launch, invocation);
    }
    let Some(launch) = current_launch(&trial_root(), "producer_child") else {
        std::process::exit(LAUNCH_DENIED_EXIT);
    };
    window_launch_commitment(&launch, invocation)
}

fn window_launch_commitment(
    launch: &SpecContentHash,
    invocation: &SpecContentHash,
) -> SpecContentHash {
    SpecContentHash::from_text(&format!(
        "workflow-os/synthetic-operator-launch/v1\n{}\n{}",
        launch.as_str(),
        invocation.as_str()
    ))
}

fn reader_scope(target: &GitHubPullRequestCommentTarget) -> ReviewedReadInstallation<'_> {
    use std::num::NonZeroU64;
    ReviewedReadInstallation {
        app_id: NonZeroU64::new(34).expect("separate read App"),
        app_slug: "fixture-app",
        installation: NonZeroU64::new(45).expect("separate read installation"),
        reader: NonZeroU64::new(22).expect("read bot"),
        repository: NonZeroU64::new(123).expect("fixture repository"),
        target,
        deadline: reader_deadline(),
    }
}

pub(super) fn reader_profile(
    target: &GitHubPullRequestCommentTarget,
) -> InstallationObserverProfile {
    let mut profile = crate::provider_comment_transport::installation_notification::tests::fixture_observer_profile();
    let scope = reader_scope(target);
    profile.reader = scope.reader;
    profile.reader_access_review = scope.access_commitment().expect("scoped recovery reader");
    profile
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Handoff {
    database: PathBuf,
    operation: DispatchReservationOperationId,
    window: AuthorizedExecutionWindowId,
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

    fn matches_launch(&self, backend: &SqliteStateBackend, launch: &SpecContentHash) -> bool {
        let Ok(connection) = backend.existing_read_only_connection() else {
            return false;
        };
        let Ok(snapshot) = crate::sqlite_state::continuity_codec::load_snapshot(&connection) else {
            return false;
        };
        let Ok(invocation) = self.action().commitment() else {
            return false;
        };
        snapshot.windows.get(&self.window).is_some_and(|window| {
            window.run_id == self.run
                && window.step_id == self.step
                && window.governance_commitment == window_launch_commitment(launch, &invocation)
        })
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
    window: &AuthorizedExecutionWindowId,
    action: GitHubCommentContinuationAction<'_>,
) -> ! {
    let root = trial_root();
    assert_eq!(path.parent(), Some(root.as_path()));
    let handoff = Handoff {
        database: path.to_owned(),
        operation: operation.clone(),
        window: window.clone(),
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
    if hosted_operator_trial() {
        use std::os::unix::fs::PermissionsExt;
        let provider = crate::provider_comment_transport::installation_signer_channel::rehearsal::fixture_root()
            .join("provider-result/comment");
        fs::write(&provider, handoff.provider_comment()).expect("export one synthetic effect");
        fs::set_permissions(provider, fs::Permissions::from_mode(0o440))
            .expect("read-only evidence for isolated observer roles");
    }
    std::process::exit(ACCEPTED_BEFORE_ACK_EXIT);
}

#[test]
#[ignore = "requires the hosted signer rehearsal launcher"]
fn hosted_operator_producer() {
    use crate::provider_comment_transport::installation_signer_channel::rehearsal::{
        assert_isolated, POLICY,
    };
    assert_isolated(POLICY.exchange_uid);
    assert!(hosted_operator_trial());
    producer_child();
}

#[test]
#[ignore = "requires the hosted signer rehearsal launcher"]
fn hosted_operator_receiver() {
    use crate::provider_comment_transport::installation_notification::rehearsal::send_operator_receipt;
    use crate::provider_comment_transport::installation_signer_channel::rehearsal::{
        assert_isolated, fixture_root,
    };
    assert_isolated(61_026);
    let body = fs::read_to_string(fixture_root().join("provider-result/comment"))
        .expect("one accepted effect");
    send_operator_receipt(&body, reader_deadline(), || {
        crate::sqlite_state::continuity_store::observe_continuity_trusted_time().ok()
    });
}

#[test]
#[ignore = "requires the hosted signer rehearsal launcher"]
fn hosted_operator_reader() {
    use crate::provider_comment_transport::installation_signer_channel::rehearsal::{
        assert_isolated, fixture_root, reader::serve_operator_inventory,
    };
    assert_isolated(61_025);
    let body = fs::read_to_string(fixture_root().join("provider-result/comment"))
        .expect("one accepted effect");
    let target =
        GitHubPullRequestCommentTarget::new("example", "sandbox", 7).expect("fixture target");
    let scope = reader_scope(&target);
    let inventory = serde_json::json!([{
        "id": 22, "user": {"id": 456}, "body": body,
        "created_at":"1970-01-01T00:00:20Z", "updated_at":"1970-01-01T00:00:20Z",
        "url":"https://api.github.com/repos/example/sandbox/issues/comments/22",
        "issue_url":"https://api.github.com/repos/example/sandbox/issues/7"
    }]);
    serve_operator_inventory(
        &scope.access_commitment().expect("exact reader scope"),
        scope.deadline,
        &inventory,
    );
}

#[test]
#[ignore = "requires the hosted signer rehearsal launcher"]
fn hosted_operator_replacement() {
    use crate::provider_comment_transport::installation_notification::{
        rehearsal::with_operator_receipt, ReviewedCreationReceiver,
    };
    use crate::provider_comment_transport::installation_signer_channel::rehearsal::{
        assert_isolated, launch_commitment, reader::read_operator_inventory, POLICY,
    };
    assert_isolated(POLICY.exchange_uid);
    let handoff = Handoff::read(&trial_root());
    let backend = SqliteStateBackend::open(&handoff.database).expect("reopen original operation");
    assert!(handoff.matches_launch(&backend, &launch_commitment()));
    let before = backend.read_events(&handoff.run).expect("original history");
    let body = handoff.provider_comment();
    assert_eq!(
        backend
            .recover_installation_comment_unix_rehearsal(
                &handoff.operation,
                handoff.action(),
                |_| None,
                reader_scope(&handoff.target),
                |_| panic!("missing receipt cannot open reader")
            )
            .expect("unavailable observer"),
        InstallationCommentRecoveryPosture::ReconciliationRequired
    );
    let connection = backend
        .existing_read_only_connection()
        .expect("read preparation");
    let payload: String = connection
        .query_row(
            "SELECT payload FROM installation_send_preparations WHERE operation_id=?1",
            [handoff.operation.as_str()],
            |row| row.get(0),
        )
        .expect("original preparation");
    let prepared: serde_json::Value = serde_json::from_str(&payload).expect("preparation record");
    let scope = serde_json::from_value(prepared["scope"].clone()).expect("writer scope");
    let profile = reader_profile(&handoff.target);
    let receiver = ReviewedCreationReceiver {
        installation: profile.receiver_installation,
        repository: reader_scope(&handoff.target).repository,
        writer_scope: &scope,
        github_hmac: &[12; 32],
        attestation_key: &profile.attestation_key,
        webhook_verifier_generation: Some(&profile.webhook_verifier_generation),
    };
    let result = with_operator_receipt(|notification| {
        backend.recover_installation_comment_unix_rehearsal(
            &handoff.operation,
            handoff.action(),
            |_| Some((receiver, notification)),
            reader_scope(&handoff.target),
            read_operator_inventory,
        )
    });
    assert_eq!(
        result.expect("exact operation recovered"),
        InstallationCommentRecoveryPosture::ObservationReconciled
    );
    assert_operator_terminal(&handoff, &backend, &before);
    assert_eq!(handoff.provider_comment(), body);
}

fn assert_operator_terminal(
    handoff: &Handoff,
    backend: &SqliteStateBackend,
    before: &[WorkflowRunEvent],
) {
    assert_eq!(
        backend
            .recover_installation_comment(
                &handoff.operation,
                handoff.action(),
                |_| panic!("terminal replay does not contact receiver"),
                |_| panic!("terminal replay does not open reader"),
            )
            .expect("durable proof replay"),
        InstallationCommentRecoveryPosture::ObservationReconciled
    );
    let history = backend
        .read_events(&handoff.run)
        .expect("completed history");
    assert_eq!(history.len(), before.len() + 1);
    assert_eq!(&history[..before.len()], before);
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

fn run_child(
    root: &Path,
    test: &str,
    reader_deadline: Timestamp,
    launch: &SpecContentHash,
) -> Output {
    let (_, module) = module_path!().split_once("::").expect("test module");
    Command::new(std::env::current_exe().expect("fixture test binary"))
        .args([
            "--ignored",
            "--exact",
            &format!("{module}::{test}"),
            "--nocapture",
        ])
        .env_clear()
        .env(TRIAL_ROOT, root)
        .env(READER_DEADLINE, reader_deadline.to_rfc3339())
        .env(LAUNCH_DIGEST, launch.as_str())
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
    let reader_deadline = Timestamp::from_offset_date_time(
        Timestamp::now_utc().as_offset_date_time() + time::Duration::minutes(15),
    );
    let config = OperatorLaunch {
        executable: executable_digest(),
        producer_entry: "producer_child".to_owned(),
        replacement_entry: "replacement_child".to_owned(),
        reader_deadline,
    };
    let approved = serde_json::to_vec(&config).expect("reviewed launch config");
    let launch = SpecContentHash::from_bytes(&approved);
    let config_path = root.0.join(LAUNCH_FILE);
    fs::write(&config_path, &approved).expect("stage reviewed launch config");

    let changed = SpecContentHash::from_text("different reviewed launch");
    assert_eq!(
        run_child(&root.0, "producer_child", reader_deadline, &changed)
            .status
            .code(),
        Some(LAUNCH_DENIED_EXIT)
    );
    assert!(!root.0.join("handoff.json").exists());
    let producer = run_child(&root.0, "producer_child", reader_deadline, &launch);
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

    assert_launch_substitutions(&root.0, &handoff, config, &approved);
    let backend = SqliteStateBackend::open_read_only(&handoff.database)
        .expect("inspect rejected replacements");
    assert_eq!(
        backend
            .read_events(&handoff.run)
            .expect("unchanged history"),
        history_before
    );
    assert_eq!(handoff.provider_comment(), comment);
    drop(backend);
    assert_successful_replacement(
        &root.0,
        &handoff,
        reader_deadline,
        &launch,
        &comment,
        &history_before,
    );
}

fn assert_launch_substitutions(
    root: &Path,
    handoff: &Handoff,
    mut config: OperatorLaunch,
    approved: &[u8],
) {
    let launch = SpecContentHash::from_bytes(approved);
    let changed = SpecContentHash::from_text("different reviewed launch");
    let reader_deadline = config.reader_deadline;
    let config_path = root.join(LAUNCH_FILE);
    for (digest, deadline) in [(&changed, reader_deadline), (&launch, Timestamp::now_utc())] {
        assert_eq!(
            run_child(root, "replacement_child", deadline, digest)
                .status
                .code(),
            Some(LAUNCH_DENIED_EXIT)
        );
    }
    let reviewed_executable = config.executable.clone();
    config.executable = SpecContentHash::from_text("substituted executable");
    let wrong_executable = serde_json::to_vec(&config).expect("changed executable config");
    fs::write(&config_path, &wrong_executable).expect("stage substituted executable");
    assert_eq!(
        run_child(
            root,
            "replacement_child",
            reader_deadline,
            &SpecContentHash::from_bytes(&wrong_executable)
        )
        .status
        .code(),
        Some(LAUNCH_DENIED_EXIT)
    );
    config.executable = reviewed_executable;
    config.replacement_entry = "producer_child".to_owned();
    let wrong_role = serde_json::to_vec(&config).expect("changed role config");
    fs::write(&config_path, &wrong_role).expect("stage substituted role");
    assert_eq!(
        run_child(
            root,
            "replacement_child",
            reader_deadline,
            &SpecContentHash::from_bytes(&wrong_role)
        )
        .status
        .code(),
        Some(LAUNCH_DENIED_EXIT)
    );
    config.replacement_entry = "replacement_child".to_owned();
    // Even a self-consistent launch is not the one persisted by Core.
    let mut changed_config = serde_json::to_vec(&config).expect("launch config");
    changed_config.push(b'\n');
    fs::write(&config_path, &changed_config).expect("stage alternate launch config");
    assert_eq!(
        run_child(
            root,
            "replacement_child",
            reader_deadline,
            &SpecContentHash::from_bytes(&changed_config)
        )
        .status
        .code(),
        Some(LAUNCH_DENIED_EXIT)
    );
    fs::write(&config_path, approved).expect("restore reviewed launch config");

    let handoff_path = root.join("handoff.json");
    let original = fs::read(&handoff_path).expect("original handoff");
    let mut changed: Handoff = serde_json::from_slice(&original).expect("typed handoff");
    changed.actor = ActorId::new("agent/substitute").expect("changed actor");
    let actor_substitution = serde_json::to_vec(&changed).expect("encode changed actor");
    changed.actor = handoff.actor.clone();
    changed.target =
        GitHubPullRequestCommentTarget::new("example", "sandbox", 999).expect("changed PR");
    let target_substitution = serde_json::to_vec(&changed).expect("encode changed target");
    for bytes in [actor_substitution, target_substitution] {
        fs::write(&handoff_path, bytes).expect("stage changed action");
        assert_eq!(
            run_child(root, "replacement_child", reader_deadline, &launch)
                .status
                .code(),
            Some(LAUNCH_DENIED_EXIT)
        );
    }
    fs::write(handoff_path, original).expect("restore handoff");
}

fn assert_successful_replacement(
    root: &Path,
    handoff: &Handoff,
    reader_deadline: Timestamp,
    launch: &SpecContentHash,
    comment: &str,
    history_before: &[WorkflowRunEvent],
) {
    let replacement = run_child(root, "replacement_child", reader_deadline, launch);
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
    assert!(handoff.matches_launch(&backend, launch));
    assert_eq!(
        backend
            .recover_installation_comment(
                &handoff.operation,
                handoff.action(),
                |_| panic!("terminal replay cannot load notification"),
                |_| panic!("terminal replay cannot load reader")
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
    assert_eq!(&history[..history_before.len()], history_before);
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
    let root = trial_root();
    let Some(launch) = current_launch(&root, "replacement_child") else {
        std::process::exit(LAUNCH_DENIED_EXIT);
    };
    let handoff = Handoff::read(&root);
    let inspection = SqliteStateBackend::open_read_only(&handoff.database).expect("inspect Core");
    if !handoff.matches_launch(&inspection, &launch) {
        std::process::exit(LAUNCH_DENIED_EXIT);
    }
    drop(inspection);
    let body = handoff.provider_comment();
    let backend = SqliteStateBackend::open(&handoff.database).expect("replace Core");
    assert_eq!(
        backend
            .recover_installation_comment(
                &handoff.operation,
                handoff.action(),
                |_| None,
                |_| panic!("missing receiver cannot open the reader")
            )
            .expect("unavailable receiver stays unresolved"),
        InstallationCommentRecoveryPosture::ReconciliationRequired
    );
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
            let reviewed = reader_scope(&handoff.target);
            let other_pr =
                GitHubPullRequestCommentTarget::new("example", "sandbox", 999).expect("other PR");
            for substituted in [
                ReviewedReadInstallation {
                    target: &other_pr,
                    ..reviewed
                },
                ReviewedReadInstallation {
                    app_slug: "other-app",
                    ..reviewed
                },
                ReviewedReadInstallation {
                    repository: std::num::NonZeroU64::new(999).expect("nonzero repository"),
                    ..reviewed
                },
            ] {
                assert_eq!(
                    backend
                        .recover_installation_comment(
                            &handoff.operation,
                            handoff.action(),
                            |_| Some((receiver, notification)),
                            |authorization| authorization.exchange(substituted, |_| panic!(
                                "scope drift must not issue access"
                            )),
                        )
                        .expect("reject reader substitution"),
                    InstallationCommentRecoveryPosture::ReconciliationRequired
                );
            }
            with_scoped_recovery_reader(reviewed, &body, |reader| {
                assert_eq!(
                    backend
                        .recover_installation_comment(
                            &handoff.operation,
                            handoff.action(),
                            |_| Some((receiver, notification)),
                            |authorization| authorization
                                .exchange(reviewed, |expected| reader.issue(expected))
                        )
                        .expect("reconcile replacement"),
                    InstallationCommentRecoveryPosture::ObservationReconciled,
                );
            });
        },
    );
}
