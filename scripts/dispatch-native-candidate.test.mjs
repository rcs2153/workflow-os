import assert from "node:assert/strict";
import { chmod, mkdir, mkdtemp, readFile, rm, symlink, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join } from "node:path";
import test from "node:test";
import { candidateDispatch, githubApi } from "./dispatch-native-candidate.mjs";

const sourceCommit = "a".repeat(40);
const target = "aarch64-apple-darwin";
const row = { id: 123, event: "workflow_dispatch", head_sha: sourceCommit,
  head_branch: "main", path: ".github/workflows/final-candidate-macos.yml", run_attempt: 1 };

async function state(t) {
  const dir = await mkdtemp(join(tmpdir(), "candidate-dispatch-"));
  t.after(() => rm(dir, { recursive: true, force: true }));
  return dir;
}

function fixture({ runs = [], refs = [sourceCommit], send = (runs) => {
  runs.push(row);
  return { workflow_run_id: row.id };
} } = {}) {
  let dispatches = 0;
  let refReads = 0;
  const api = async (route, body) => {
    if (route === "/git/ref/heads/main") {
      return { ref: "refs/heads/main", object: { type: "commit", sha: refs[Math.min(refReads++, refs.length - 1)] } };
    }
    if (route.endsWith("/dispatches")) {
      assert.equal(route, "/actions/workflows/final-candidate-macos.yml/dispatches");
      assert.deepEqual(body, { ref: "main", inputs: { source_commit: sourceCommit } });
      dispatches++;
      return send(runs);
    }
    assert.equal(route, `/actions/workflows/final-candidate-macos.yml/runs?event=workflow_dispatch&head_sha=${sourceCommit}&per_page=2`);
    return { total_count: runs.length, workflow_runs: runs.slice(0, 2) };
  };
  return { run: (args) => candidateDispatch({ api, sourceCommit, target, ...args }), count: () => dispatches };
}

test("persists the exact claim before dispatch and observes the matching run", async (t) => {
  const stateDir = await state(t);
  const f = fixture({ send: async (runs) => {
    const claim = JSON.parse(await readFile(join(stateDir, `dispatch-${target}-${sourceCommit}.json`), "utf8"));
    assert.deepEqual(claim, { repository: "rcs2153/workflow-os", ref: "main",
      workflow: "final-candidate-macos.yml", sourceCommit, target });
    runs.push(row);
    return { workflow_run_id: row.id };
  } });
  assert.deepEqual(await f.run({ stateDir, dispatch: true }), {
    status: "dispatch_reply_matched", target, source_commit: sourceCommit, run_id: 123, run_attempt: 1,
  });
  assert.equal((await f.run({ stateDir, dispatch: true })).status, "dispatch_reply_matched");
  assert.equal(f.count(), 1);
});

test("competing callers send once even when the first response is lost and no run is visible", async (t) => {
  const stateDir = await state(t);
  const runs = [];
  const f = fixture({ runs, send: () => { throw new Error("transport lost"); } });
  const results = await Promise.all(Array.from({ length: 8 }, () => f.run({ stateDir, dispatch: true })));
  assert.equal(results.filter((r) => r.status === "dispatch_unresolved_reconcile_only").length, 1);
  assert.equal(results.filter((r) => r.status === "claim_present_reconcile_only").length, 7);
  assert.equal(f.count(), 1);
  assert.equal((await f.run({ stateDir, dispatch: true })).status, "claim_present_reconcile_only");
  runs.push(row);
  assert.equal((await f.run()).run_id, 123);
  assert.equal(f.count(), 1);
});

test("reconciles an accepted dispatch with a lost reply without resending", async (t) => {
  const f = fixture({ send: (runs) => { runs.push(row); throw new Error("reply lost"); } });
  const attempt = await f.run({ stateDir: await state(t), dispatch: true });
  assert.equal(attempt.run_id, 123);
  assert.equal(attempt.status, "inventory_only_run_no_dispatch");
  assert.equal(f.count(), 1);
});

test("a persisted reply survives replacement and rejects a different observed ID", async (t) => {
  const stateDir = await state(t);
  const f = fixture({ send: () => ({ workflow_run_id: row.id }) });
  const pending = await f.run({ stateDir, dispatch: true });
  assert.equal(pending.status, "dispatch_reply_pending_inventory");
  assert.equal(pending.reported_run_id, row.id);
  const wrong = fixture({ runs: [{ ...row, id: 124 }] });
  await assert.rejects(wrong.run({ stateDir }), /persisted dispatch reply and observed run disagree/);
  await assert.rejects(wrong.run({ stateDir, dispatch: true }), /persisted dispatch reply and observed run disagree/);
  assert.equal(wrong.count(), 0);
  const replacement = fixture({ runs: [row] });
  assert.equal((await replacement.run({ stateDir })).status, "dispatch_reply_matched");
  assert.equal(replacement.count(), 0);
});

test("a torn or misaddressed reply fails closed even when inventory contains one run", async (t) => {
  const stateDir = await state(t);
  const path = join(stateDir, `dispatch-${target}-${sourceCommit}.reply.json`);
  const f = fixture({ runs: [row] });
  for (const receipt of ["", JSON.stringify({ sourceCommit, target: "x86_64-unknown-linux-musl", runId: row.id })]) {
    await writeFile(path, receipt, { mode: 0o600 });
    await assert.rejects(f.run({ stateDir }), /invalid persisted candidate reply/);
  }
  assert.equal(f.count(), 0);
});

test("a torn claim never reopens the dispatch window", async (t) => {
  const stateDir = await state(t);
  await writeFile(join(stateDir, `dispatch-${target}-${sourceCommit}.json`), "");
  const f = fixture();
  assert.equal((await f.run({ stateDir, dispatch: true })).status, "claim_present_reconcile_only");
  assert.equal(f.count(), 0);
});

test("non-exact commits and unsupported targets never reach the transport", async () => {
  const api = () => assert.fail("transport reached");
  await assert.rejects(candidateDispatch({ api, sourceCommit: sourceCommit + "\n", target }), /exact source/);
  await assert.rejects(candidateDispatch({ api, sourceCommit, target: "custom.yml" }), /native target/);
});

test("read-only lookup does not dispatch and any already-bound run prevents a dispatch", async (t) => {
  const f = fixture({ runs: [row] });
  const empty = fixture();
  assert.equal((await empty.run()).status, "not_observed");
  assert.equal(empty.count(), 0);
  assert.equal((await f.run({ stateDir: await state(t), dispatch: true })).status, "inventory_only_run_no_dispatch");
  assert.equal(f.count(), 0);
});

test("duplicate, wrong-source, wrong-workflow, and malformed inventory fail closed", async () => {
  for (const runs of [
    [row, { ...row, id: 124 }],
    [{ ...row, head_sha: "b".repeat(40) }],
    [{ ...row, path: ".github/workflows/ci.yml" }],
    [{ ...row, run_attempt: null }],
  ]) {
    const f = fixture({ runs });
    await assert.rejects(f.run({ stateDir: "/unused", dispatch: true }), /reconciliation|invalid or mismatched/);
    assert.equal(f.count(), 0);
  }
  await assert.rejects(candidateDispatch({ api: async () => ({ total_count: 1, workflow_runs: [] }),
    sourceCommit, target }), /invalid or mismatched/);
});

test("drift before and after a durable claim never sends a dispatch", async (t) => {
  for (const refs of [["b".repeat(40)], [sourceCommit, "b".repeat(40)]]) {
    const stateDir = await state(t);
    const f = fixture({ refs });
    await assert.rejects(f.run({ stateDir, dispatch: true }), /does not match/);
    assert.equal(f.count(), 0);
    if (refs.length === 2) {
      const replacement = fixture();
      assert.equal((await replacement.run({ stateDir, dispatch: true })).status, "claim_present_reconcile_only");
      assert.equal(replacement.count(), 0);
    }
  }
});

test("a writable-by-group or symlink claim directory fails before dispatch", async (t) => {
  const dir = await state(t);
  const privateDir = join(dir, "private");
  const link = join(dir, "link");
  await mkdir(privateDir, { mode: 0o700 });
  await symlink(privateDir, link);
  const f = fixture();
  await assert.rejects(f.run({ stateDir: link, dispatch: true }));
  await chmod(privateDir, 0o770);
  await assert.rejects(f.run({ stateDir: privateDir, dispatch: true }), /private/);
  assert.equal(f.count(), 0);
});

test("transport fixes GitHub host and API version and refuses redirects", async () => {
  const api = githubApi("fixture", async (url, options) => {
    assert.match(url, /^https:\/\/api\.github\.com\/repos\/rcs2153\/workflow-os\/actions\//);
    assert.equal(options.redirect, "error");
    assert.equal(options.method, "POST");
    assert.equal(options.headers["x-github-api-version"], "2026-03-10");
    assert.equal(options.headers.authorization, "Bearer fixture");
    return { status: 302 };
  });
  await assert.rejects(api("/actions/workflows/final-candidate-linux.yml/dispatches", { ref: "main" }),
    /did not return/);
});
