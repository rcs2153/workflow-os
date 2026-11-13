import assert from "node:assert/strict";
import { spawn, spawnSync } from "node:child_process";
import { once } from "node:events";
import { accessSync, constants } from "node:fs";
import { setTimeout as sleep } from "node:timers/promises";

// Use only an empty, disposable database: this rehearsal installs a fault trigger.
const database = process.env.WORKFLOW_OS_RESULT_REHEARSAL_DATABASE_URL;
assert(database, "Set WORKFLOW_OS_RESULT_REHEARSAL_DATABASE_URL to an empty disposable database");
const binary = process.env.WORKFLOW_OS_RESULT_REHEARSAL_BINARY ?? "./target/debug/workflow-os-hosted";
accessSync(binary, constants.X_OK);
const port = process.env.WORKFLOW_OS_RESULT_REHEARSAL_PORT ?? "18579";
const base = `http://127.0.0.1:${port}`;
const auth = "disposable-result-rehearsal-only";
const operatorAuth = "disposable-conflict-operator-only";
const children = new Set();
const outputs = new Map();
function sql(query) {
  const result = spawnSync("psql", [database, "-X", "-A", "-t", "-v", "ON_ERROR_STOP=1", "-c", query], { encoding: "utf8" });
  assert.equal(result.status, 0, "disposable database command failed");
  return result.stdout.trim();
}
function start(args, actor, application) {
  const url = new URL(database);
  url.searchParams.set("application_name", application);
  const child = spawn(binary, args, { env: {
    ...process.env,
    WORKFLOW_OS_HOSTED_DATABASE_URL: url.toString(),
    WORKFLOW_OS_HOSTED_ACTOR: actor,
    WORKFLOW_OS_HOSTED_TOKEN: auth,
    WORKFLOW_OS_HOSTED_CONFLICT_OPERATOR_TOKEN: operatorAuth,
    WORKFLOW_OS_HOSTED_CONFLICT_OPERATOR_ACTOR: "user/conflict-operator",
    WORKFLOW_OS_HOSTED_BIND: `127.0.0.1:${port}`,
    WORKFLOW_OS_HOSTED_PROJECT_ROOT: `${process.cwd()}/deploy/hosted-alpha/project`,
  }, stdio: ["ignore", "pipe", "pipe"] });
  children.add(child);
  outputs.set(child, "");
  for (const stream of [child.stdout, child.stderr]) stream.on("data", data => {
    outputs.set(child, (outputs.get(child) + data).slice(-8192));
  });
  child.on("exit", () => children.delete(child));
  return child;
}
async function until(check, label, timeout = 45000) {
  const deadline = Date.now() + timeout;
  while (Date.now() < deadline) {
    if (await check()) return;
    await sleep(100);
  }
  throw new Error(`Timed out: ${label}`);
}
async function request(path, body, identity = auth) {
  return fetch(`${base}${path}`, {
    headers: { Authorization: `Bearer ${identity}`, "Content-Type": "application/json" },
    ...(body ? { method: "POST", body: JSON.stringify(body) } : {}),
  });
}
async function finished(child) {
  if (child.exitCode !== null || child.signalCode !== null) return;
  await Promise.race([once(child, "exit"), sleep(45000, undefined, { ref: false }).then(() => { throw new Error("worker did not exit"); })]);
}

async function startRehearsalApi(application) {
  const api = start([], "user/result-rehearsal", application);
  await until(async () => {
    assert.equal(api.exitCode, null, `API exited: ${outputs.get(api)}`);
    assert.equal(api.signalCode, null, `API stopped: ${outputs.get(api)}`);
    try { return (await request("/health/ready")).ok; } catch { return false; }
  }, "API readiness", 120000);
  return api;
}

async function rehearseConcurrentCreates() {
  const count = 25;
  const retryable = new Set(["immutable_bundle", "creation_prefix", "dispatch", "rehydrate"]
    .map(stage => `executor.hosted_no_write.${stage}_retry_exhausted`));
  retryable.add("hosted.run_create.reservation_retry_exhausted");
  const bodies = Array.from({ length: count }, (_, index) => ({
    run_id: `run-http-contention-${index}`, workflow_id: "hosted/recovery-proof",
    bundle_id: `bundle-http-contention-${index}`, bundle_version: "v1",
    created_at: new Date().toISOString(), correlation_id: `correlation-http-contention-${index}`,
    idempotency_key: `http-contention-create-${index}`, sensitivity: "internal", redaction_required: true,
  }));
  let retries = 0;
  async function create(body) {
    for (let attempt = 0; attempt < 4; attempt++) {
      const response = await request("/api/v0alpha1/runs", body);
      const result = await response.json();
      if (response.ok) {
        assert.equal(response.status, 201);
        assert.equal(result.snapshot.identity.run_id, body.run_id);
        return result;
      }
      assert.equal(response.status, 409, `unexpected create status: ${result.code}`);
      assert(retryable.has(result.code), `non-retryable conflict: ${result.code}`);
      assert(attempt < 3, `exact retries exhausted for ${body.run_id}`);
      retries++;
      await sleep(100 * (attempt + 1) + 10 * Number(body.run_id.split("-").at(-1)));
    }
  }
  const submitted = await Promise.allSettled(bodies.map(create));
  for (const result of submitted) {
    if (result.status === "rejected") throw result.reason;
  }
  for (let index = 0; index < count; index++) {
    const worker = start(["--worker-once"], "service/http-contention", "wos-http-contention");
    await finished(worker);
    assert.equal(worker.exitCode, 0, outputs.get(worker));
    assert.match(outputs.get(worker), /worker_outcome: completed/);
  }
  const reports = new Set();
  for (const body of bodies) {
    const path = `/api/v0alpha1/runs/${body.run_id}`;
    const events = async () => {
      const response = await request(`${path}/events?limit=50`);
      assert.equal(response.status, 200);
      const page = await response.json();
      assert.equal(page.has_more, false);
      return page.events;
    };
    const before = await events();
    for (const kind of ["RunCreated", "SkillInvocationStarted", "SkillInvocationSucceeded", "RunCompleted"]) {
      assert.equal(before.filter(event => event.kind.kind === kind).length, 1, `${body.run_id}: ${kind}`);
    }
    const report = async () => {
      const response = await request(`${path}/report`);
      assert.equal(response.status, 200);
      const metadata = await response.json();
      assert.equal(metadata.run_id, body.run_id);
      assert(metadata.report_id);
      return metadata;
    };
    const original = await report();
    reports.add(original.report_id);
    const replay = await create(body);
    assert.equal(replay.snapshot.status, "completed");
    assert.deepEqual(await events(), before);
    assert.deepEqual(await report(), original);
  }
  assert.equal(reports.size, count);
  const idle = start(["--worker-once"], "service/http-contention", "wos-http-contention");
  await finished(idle);
  assert.equal(idle.exitCode, 0, outputs.get(idle));
  assert.match(outputs.get(idle), /worker_outcome: idle/);
  console.log(`hosted_http_contention: ${count} distinct terminal runs and reports; ${retries} exact retries; no queued work remained`);
}

async function rehearseExactCreateRecovery(stage) {
  const runId = `run-http-${stage}-recovery`;
  const body = {
    run_id: runId, workflow_id: "hosted/recovery-proof",
    bundle_id: `bundle-http-${stage}-recovery`, bundle_version: "v1",
    created_at: new Date().toISOString(), correlation_id: `correlation-http-${stage}-recovery`,
    idempotency_key: `http-${stage}-recovery-create`, sensitivity: "internal", redaction_required: true,
  };
  const path = `/api/v0alpha1/runs/${body.run_id}`;
  const reservation = stage === "reservation";
  const table = reservation ? "idempotency" : "events";
  const predicate = reservation ? `NEW.key = '${body.idempotency_key}'` : `NEW.run_id = '${runId}'`;
  sql(`CREATE SEQUENCE public.wos_create_fault_attempts;
    CREATE FUNCTION public.wos_create_fault() RETURNS trigger LANGUAGE plpgsql AS $$
    BEGIN
      IF ${predicate} THEN
        PERFORM nextval('public.wos_create_fault_attempts');
        RAISE EXCEPTION 'fixture serialization failure' USING ERRCODE = '40001';
      END IF;
      RETURN NEW;
    END $$;
    CREATE TRIGGER wos_create_fault BEFORE INSERT ON workflow_os.${table}
      FOR EACH ROW EXECUTE FUNCTION public.wos_create_fault();`);
  try {
    const rejected = await request("/api/v0alpha1/runs", body);
    assert.equal(rejected.status, 409);
    assert.equal((await rejected.json()).code, reservation
      ? "hosted.run_create.reservation_retry_exhausted"
      : "executor.hosted_no_write.creation_prefix_retry_exhausted");
    if (reservation) {
      // Each of the eight reservation calls uses three backend transaction attempts.
      assert.equal(sql("SELECT last_value FROM public.wos_create_fault_attempts"), "24");
      assert.equal(sql(`SELECT count(*) FROM workflow_os.idempotency WHERE key='${body.idempotency_key}'`), "0");
    }
    assert.equal(sql(`SELECT count(*) FROM workflow_os.events WHERE run_id='${runId}'`), "0");
    const report = await request(`${path}/report`);
    assert.equal(report.status, 409);
    assert.equal((await report.json()).code, "runtime.events.empty");
    assert.equal(sql(`SELECT count(*) FROM workflow_os.records WHERE family='work_report' AND key2='${runId}'`), "0");
    assert.equal(sql(`SELECT count(*) FROM workflow_os.records WHERE family='hosted_work_item' AND payload::jsonb ->> 'run_id' = '${runId}'`), "0");
  } finally {
    sql(`DROP TRIGGER wos_create_fault ON workflow_os.${table};
      DROP FUNCTION public.wos_create_fault();
      DROP SEQUENCE public.wos_create_fault_attempts;`);
  }
  const recovered = await request("/api/v0alpha1/runs", body);
  assert.equal(recovered.status, 201);
  assert.equal((await recovered.json()).snapshot.identity.run_id, body.run_id);
  const queuedHistoryResponse = await request(`${path}/events?limit=1`);
  assert.equal(queuedHistoryResponse.status, 200);
  const queuedHistory = await queuedHistoryResponse.json();
  assert.match(queuedHistory.history_revision, /^[a-f0-9]{64}$/);
  const worker = start(["--worker-once"], "service/http-prefix-recovery", "wos-http-prefix-recovery");
  await finished(worker);
  assert.equal(worker.exitCode, 0, outputs.get(worker));
  assert.match(outputs.get(worker), /worker_outcome: completed/);
  async function read(suffix) {
    const response = await request(path + suffix);
    assert.equal(response.status, 200);
    return response.json();
  }
  const events = await read("/events?limit=50");
  assert.equal(events.has_more, false);
  assert.match(events.history_revision, /^[a-f0-9]{64}$/);
  assert.notEqual(events.history_revision, queuedHistory.history_revision);
  const firstPage = await read("/events?limit=1");
  assert.equal(firstPage.has_more, true);
  assert.equal(firstPage.history_revision, events.history_revision);
  const nextPage = await read(`/events?limit=1&after_sequence=${firstPage.events[0].sequence_number}`);
  assert.equal(nextPage.history_revision, events.history_revision);
  assert.deepEqual(nextPage.events[0], events.events[1]);
  for (const kind of ["RunCreated", "SkillInvocationStarted", "SkillInvocationSucceeded", "RunCompleted"]) {
    assert.equal(events.events.filter(event => event.kind.kind === kind).length, 1, kind);
  }
  const report = await read("/report");
  assert.equal(report.run_id, body.run_id);
  assert(report.report_id);
  const replay = await request("/api/v0alpha1/runs", body);
  assert(replay.ok);
  assert.equal((await replay.json()).snapshot.status, "completed");
  assert.deepEqual(await read("/events?limit=50"), events);
  assert.deepEqual(await read("/report"), report);
  console.log(`hosted_http_exact_retry: passed; 409 ${stage} exhaustion recovered once`);
  return body;
}

async function rehearseConflictOperator(body, api) {
  const runPath = `/api/v0alpha1/runs/${body.run_id}`;
  const route = `/api/v0alpha1/operator/runs/${body.run_id}/creation-conflicts`;
  const currentRoute = `${route}/current-decision`;
  sql(`UPDATE workflow_os.events SET payload = jsonb_set(payload::jsonb, '{actor}', '"user/legacy-actor"')::text
       WHERE run_id = '${body.run_id}' AND sequence_number = 1`);
  async function verifyConflict() {
    const response = await request("/api/v0alpha1/runs", body);
    assert.equal(response.status, 409);
    assert.equal((await response.json()).code, "postgres_state.hosted_creation.prefix_conflict");
  }
  await verifyConflict();
  assert.equal((await request(route)).status, 401);
  assert.equal((await request(currentRoute)).status, 401);
  assert.equal((await request(currentRoute, undefined, operatorAuth)).status, 404);
  assert.equal((await request(runPath, undefined, operatorAuth)).status, 401);
  const response = await request(route, undefined, operatorAuth);
  assert.equal(response.status, 200);
  const observations = await response.json();
  assert.equal(observations.length, 1);
  const [observation] = observations;
  const before = await (await request(`${runPath}/events`)).json();
  // The database is disposable and there are no workers running during this comparison.
  const readExecutionRecords = () => sql(`SELECT family, key1, key2, revision, payload
    FROM workflow_os.records
    WHERE family NOT IN ('hosted_conflict_disposition', 'hosted_conflict_disposition_head')
    ORDER BY family, key1, key2`);
  const executionRecords = readExecutionRecords();
  assert.equal(observation.observed_history_revision, before.history_revision);
  const requestBody = {
    run_id: body.run_id, decision_id: "fixture-investigation",
    observed_history_revision: observation.observed_history_revision,
    expected_prefix_revision: observation.expected_prefix_revision,
    supersedes: null, disposition: "investigation_required", reason: "Legacy actor mismatch",
    owner: "user/conflict-operator", evidence_event_ids: [before.events[0].event_id],
  };
  assert.equal((await request(route, requestBody)).status, 401);
  assert.equal((await request(route, { ...requestBody, run_id: "run-result-interruption" }, operatorAuth)).status, 400);
  const decision = await request(route, requestBody, operatorAuth);
  assert.equal(decision.status, 200);
  const recorded = await decision.json();
  assert.equal(recorded.actor, "user/conflict-operator");
  assert.equal(api.kill("SIGINT"), true);
  await finished(api);
  assert.equal(api.exitCode, 0, outputs.get(api));
  await startRehearsalApi("wos-result-api-replacement");
  const current = await request(currentRoute, undefined, operatorAuth);
  assert.equal(current.status, 200);
  assert.deepEqual(await current.json(), {
    decision: recorded, current_history_revision: before.history_revision,
  });
  const replay = await request(route, requestBody, operatorAuth);
  assert.equal(replay.status, 200);
  assert.deepEqual(await replay.json(), recorded);
  assert.equal((await request(route, { ...requestBody, reason: "Changed reasoning" }, operatorAuth)).status, 409);
  await verifyConflict();
  assert.deepEqual(await (await request(`${runPath}/events`)).json(), before);
  assert.equal(readExecutionRecords(), executionRecords);
  assert.equal(sql(`SELECT count(*) FROM workflow_os.records WHERE family='hosted_conflict_disposition' AND key1='${body.run_id}'`), "1");
  // Simulate a different history after review without changing the decision.
  sql(`UPDATE workflow_os.events SET payload = jsonb_set(payload::jsonb, '{actor}', '"user/new-legacy-actor"')::text
       WHERE run_id = '${body.run_id}' AND sequence_number = 1`);
  const changed = await request(currentRoute, undefined, operatorAuth);
  assert.equal(changed.status, 200);
  const updated = await changed.json();
  assert.deepEqual(updated.decision, recorded);
  assert.notEqual(updated.current_history_revision, before.history_revision);
  const updatedEvents = await (await request(`${runPath}/events`)).json();
  assert.equal(updated.current_history_revision, updatedEvents.history_revision);
  assert.equal(readExecutionRecords(), executionRecords);
  console.log("hosted_conflict_current_decision: passed; predecessor recovered after restart and stale evidence stays visible");
  console.log("hosted_conflict_operator: passed; separate access and stable decision replay after API restart without run writes");
}

async function rehearseInvalidHistoryInspection() {
  const path = "/api/v0alpha1/runs/run-invalid-legacy-history";
  // Seed an internally bound row that cannot begin a valid run.
  sql(`INSERT INTO workflow_os.events
        (run_id, sequence_number, event_id, workflow_id, schema_version, workflow_version, spec_hash, payload)
      SELECT 'run-invalid-legacy-history', sequence_number, 'event-invalid-legacy-history',
        workflow_id, schema_version, workflow_version, spec_hash,
        (payload::jsonb || '{"run_id":"run-invalid-legacy-history","event_id":"event-invalid-legacy-history","kind":{"kind":"RunStarted"}}'::jsonb)::text
      FROM workflow_os.events WHERE run_id='run-result-interruption' AND sequence_number=1`);
  const rejected = await request(path);
  assert.equal(rejected.status, 409);
  assert.equal((await rejected.json()).code, "runtime.run_created.missing");
  assert.equal((await fetch(`${base}${path}/events`)).status, 401);
  async function inspect() {
    const response = await request(`${path}/events?limit=1`);
    assert.equal(response.status, 200);
    return response.json();
  }
  const observed = await inspect();
  assert.equal(observed.events.length, 1);
  assert.equal(observed.events[0].kind.kind, "RunStarted");
  assert.equal(observed.has_more, false);
  assert.match(observed.history_revision, /^[a-f0-9]{64}$/);
  assert.deepEqual(await inspect(), observed);
  assert.equal(sql("SELECT count(*) FROM workflow_os.records WHERE key1='run-invalid-legacy-history'"), "0");
  sql("UPDATE workflow_os.events SET event_id='event-invalid-legacy-misbound' WHERE run_id='run-invalid-legacy-history'");
  const misbound = await request(`${path}/events`);
  assert.equal(misbound.status, 409);
  assert.equal((await misbound.json()).code, "postgres_state.event.identity_mismatch");
  console.log("hosted_invalid_history_inspection: passed; grammar-invalid events remain inspectable, misbound events fail closed");
}

assert.equal(sql("SELECT count(*) FROM pg_namespace WHERE nspname='workflow_os'"), "0", "database must not contain Workflow OS state");
try {
  const api = await startRehearsalApi("wos-result-api");
  sql(`CREATE FUNCTION public.wos_result_fault() RETURNS trigger LANGUAGE plpgsql AS $$
    BEGIN
      IF NEW.payload::jsonb #>> '{kind,kind}' = 'SkillInvocationSucceeded' THEN
        PERFORM pg_sleep(120);
      END IF;
      RETURN NEW;
    END $$;
    CREATE TRIGGER wos_result_fault BEFORE INSERT ON workflow_os.events
      FOR EACH ROW EXECUTE FUNCTION public.wos_result_fault();`);
  const body = {
    run_id: "run-result-interruption", workflow_id: "hosted/recovery-proof",
    bundle_id: "bundle-result-interruption", bundle_version: "v1",
    created_at: new Date().toISOString(), correlation_id: "correlation-result-interruption",
    idempotency_key: "result-interruption-create", sensitivity: "internal", redaction_required: true,
  };
  assert.equal((await request("/api/v0alpha1/runs", body)).status, 201);
  const original = start(["--worker-once"], "service/result-original", "wos-result-original");
  await until(() => sql("SELECT count(*) FROM pg_stat_activity WHERE application_name='wos-result-original' AND wait_event='PgSleep'") === "1", "success transaction fault");
  original.kill("SIGKILL");
  await finished(original);
  sql("SELECT pg_terminate_backend(pid) FROM pg_stat_activity WHERE application_name='wos-result-original'");
  sql("DROP TRIGGER wos_result_fault ON workflow_os.events; DROP FUNCTION public.wos_result_fault()");
  assert.equal(sql("SELECT key2 FROM workflow_os.records WHERE family='hosted_execution_attempt'"), "invoking");
  assert.equal(sql("SELECT count(*) FROM workflow_os.events WHERE payload::jsonb #>> '{kind,kind}'='SkillInvocationSucceeded'"), "0");
  await until(() => sql("SELECT count(*) FROM workflow_os.worker_leases WHERE expires_at > clock_timestamp()") === "0", "original lease expiry");
  const replacement = start(["--worker-once"], "service/result-replacement", "wos-result-replacement");
  await finished(replacement);
  assert.equal(replacement.exitCode, 0, `replacement failed: ${outputs.get(replacement)}`);
  assert.match(outputs.get(replacement), /reconciliation/);
  assert.equal(sql("SELECT key2 FROM workflow_os.records WHERE family='hosted_execution_attempt'"), "reconciliation_required");
  assert.equal(sql("SELECT key2 FROM workflow_os.records WHERE family='hosted_work_item'"), "ambiguous");
  assert.equal(sql("SELECT count(*) FROM workflow_os.records WHERE family='hosted_execution_attempt'"), "1");
  assert.equal(sql("SELECT count(*) FROM workflow_os.events WHERE payload::jsonb #>> '{kind,kind}'='EscalationTriggered'"), "1");
  const run = await (await request("/api/v0alpha1/runs/run-result-interruption")).json();
  assert.equal(run.snapshot.status, "escalated");
  assert.equal((await request("/api/v0alpha1/runs/run-result-interruption/report")).status, 404);
  const before = sql("SELECT count(*) FROM workflow_os.events");
  const replay = await request("/api/v0alpha1/runs", body);
  assert(replay.ok);
  assert.equal((await replay.json()).snapshot.status, "escalated");
  const next = start(["--worker-once"], "service/result-next", "wos-result-next");
  await finished(next);
  assert.equal(next.exitCode, 0);
  assert.match(outputs.get(next), /idle/);
  assert.equal(sql("SELECT count(*) FROM workflow_os.events"), before);
  assert.equal(sql("SELECT count(*) FROM workflow_os.events WHERE payload::jsonb #>> '{kind,kind}' IN ('SkillInvocationSucceeded','RunCompleted')"), "0");
  console.log("hosted_result_interruption: passed; durable reconciliation, no repeated invocation or fabricated report");
  await rehearseConcurrentCreates();
  await rehearseExactCreateRecovery("reservation");
  await rehearseConflictOperator(await rehearseExactCreateRecovery("prefix"), api);
  await rehearseInvalidHistoryInspection();
} finally {
  for (const child of [...children]) {
    child.kill("SIGKILL");
    await finished(child);
  }
}
