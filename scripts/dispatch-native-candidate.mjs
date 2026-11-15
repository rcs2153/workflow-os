import { constants } from "node:fs";
import { open } from "node:fs/promises";
import { join, resolve } from "node:path";
import { pathToFileURL } from "node:url";
import { parseArgs } from "node:util";

const repository = "rcs2153/workflow-os";
const workflows = new Map([
  ["aarch64-apple-darwin", "final-candidate-macos.yml"],
  ["x86_64-unknown-linux-musl", "final-candidate-linux.yml"],
]);
const isCommit = (value) => typeof value === "string" && value.length === 40 && /^[0-9a-f]+$/.test(value);
const isRunId = (value) => Number.isSafeInteger(value) && value > 0;

async function lookup(api, route, sourceCommit, target) {
  const inventory = await api(`${route}/runs?event=workflow_dispatch&head_sha=${sourceCommit}&per_page=2`);
  const rows = inventory?.workflow_runs;
  const total = inventory?.total_count;
  if (!Array.isArray(rows) || !Number.isSafeInteger(total) || total < 0 ||
      rows.length !== Math.min(total, 2) ||
      rows.some((row) => !isRunId(row?.id) || row.event !== "workflow_dispatch" ||
        row.head_sha !== sourceCommit || row.head_branch !== "main" ||
        row.path !== `.github/workflows/${workflows.get(target)}` ||
        !isRunId(row.run_attempt))) {
    throw new Error("GitHub returned invalid or mismatched candidate inventory");
  }
  if (total > 1) {
    throw new Error("multiple bound candidate runs require operator reconciliation");
  }
  return rows[0];
}

async function assertMain(api, sourceCommit) {
  const ref = await api("/git/ref/heads/main");
  if (ref?.ref !== "refs/heads/main" || ref.object?.type !== "commit" ||
      ref.object.sha !== sourceCommit) {
    throw new Error("GitHub main does not match the approved source commit");
  }
}

async function claimOnce(stateDir, sourceCommit, target, workflow) {
  const directory = await open(stateDir, constants.O_RDONLY | constants.O_DIRECTORY | constants.O_NOFOLLOW);
  try {
    const stat = await directory.stat();
    if (typeof process.getuid !== "function" || stat.uid !== process.getuid() || (stat.mode & 0o077)) {
      throw new Error("claim directory must be owned by the operator and private");
    }
    let claim;
    try {
      claim = await open(join(stateDir, `dispatch-${target}-${sourceCommit}.json`), "wx", 0o600);
    } catch (error) {
      if (error.code === "EEXIST") return false;
      throw error;
    }
    try {
      await claim.writeFile(JSON.stringify({ repository, ref: "main", workflow, sourceCommit, target }) + "\n");
      await claim.sync();
    } finally {
      await claim.close();
    }
    await directory.sync();
    return true;
  } finally {
    await directory.close();
  }
}

export async function candidateDispatch({ api, sourceCommit, target, stateDir, dispatch = false }) {
  const workflow = workflows.get(target);
  if (!workflow || !isCommit(sourceCommit) || (dispatch && !stateDir)) {
    throw new Error("exact source commit and native target required; dispatch also requires a private claim directory");
  }
  const route = `/actions/workflows/${workflow}`;
  const observed = (run, status) => ({ status, target, source_commit: sourceCommit,
    ...(run && { run_id: run.id, run_attempt: run.run_attempt }) });
  const existing = await lookup(api, route, sourceCommit, target);
  if (existing) return observed(existing, "existing_run_no_dispatch");
  if (!dispatch) return observed(null, "not_observed");

  await assertMain(api, sourceCommit);
  const won = await claimOnce(stateDir, sourceCommit, target, workflow);
  const afterClaim = await lookup(api, route, sourceCommit, target);
  if (afterClaim) return observed(afterClaim, "existing_run_no_dispatch");
  if (!won) return observed(null, "claim_present_reconcile_only");
  await assertMain(api, sourceCommit);

  let runId;
  try {
    const receipt = await api(`${route}/dispatches`, { ref: "main", inputs: { source_commit: sourceCommit } });
    runId = receipt?.workflow_run_id;
  } catch {
    // The request may have reached GitHub. Never release the claim or resend.
  }
  const reconciled = await lookup(api, route, sourceCommit, target);
  if (reconciled && (!isRunId(runId) || runId === reconciled.id)) {
    return observed(reconciled, "run_observed_after_dispatch");
  }
  if (reconciled) throw new Error("dispatch reply and observed run disagree; operator reconciliation required");
  return { ...observed(null, "dispatch_unresolved_reconcile_only"),
    ...(isRunId(runId) && { reported_run_id: runId }) };
}

export function githubApi(token, fetchRequest = fetch) {
  return async (route, body) => {
    const headers = { accept: "application/vnd.github+json", "x-github-api-version": "2026-03-10" };
    if (token) headers.authorization = `Bearer ${token}`;
    if (body) headers["content-type"] = "application/json";
    const response = await fetchRequest(`https://api.github.com/repos/${repository}${route}`, {
      method: body ? "POST" : "GET",
      headers,
      body: body && JSON.stringify(body),
      redirect: "error",
      signal: AbortSignal.timeout(15_000),
    });
    if (response.status !== 200) throw new Error("GitHub candidate API did not return a successful JSON response");
    return response.json();
  };
}

async function main() {
  const { values } = parseArgs({ options: {
    "source-commit": { type: "string" },
    target: { type: "string" },
    "state-dir": { type: "string" },
    dispatch: { type: "boolean" },
  } });
  const token = process.env.GH_TOKEN;
  if (values.dispatch && !token) throw new Error("explicit dispatch requires GH_TOKEN");
  const result = await candidateDispatch({
    api: githubApi(token),
    sourceCommit: values["source-commit"],
    target: values.target,
    stateDir: values["state-dir"] ? resolve(values["state-dir"]) : undefined,
    dispatch: values.dispatch,
  });
  console.log(JSON.stringify(result));
  if (!result.run_id) process.exitCode = 2;
}

if (process.argv[1] && pathToFileURL(resolve(process.argv[1])).href === import.meta.url) {
  main().catch(() => {
    console.error("Candidate request failed closed. Check arguments, approved source, claim custody and GitHub inventory before any further action; do not remove an existing claim.");
    process.exitCode = 2;
  });
}
