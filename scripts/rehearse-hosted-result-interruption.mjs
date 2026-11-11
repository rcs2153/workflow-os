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
async function request(path, body) {
  return fetch(`${base}${path}`, {
    headers: { Authorization: `Bearer ${auth}`, "Content-Type": "application/json" },
    ...(body ? { method: "POST", body: JSON.stringify(body) } : {}),
  });
}
async function finished(child) {
  if (child.exitCode !== null || child.signalCode !== null) return;
  await Promise.race([once(child, "exit"), sleep(45000, undefined, { ref: false }).then(() => { throw new Error("worker did not exit"); })]);
}

assert.equal(sql("SELECT count(*) FROM pg_namespace WHERE nspname='workflow_os'"), "0", "database must not contain Workflow OS state");
try {
  const api = start([], "user/result-rehearsal", "wos-result-api");
  await until(async () => {
    assert.equal(api.exitCode, null, `API exited: ${outputs.get(api)}`);
    try { return (await request("/health/ready")).ok; } catch { return false; }
  }, "API readiness", 120000);
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
} finally {
  for (const child of [...children]) {
    child.kill("SIGKILL");
    await finished(child);
  }
}
