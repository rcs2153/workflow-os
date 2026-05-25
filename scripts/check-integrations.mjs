import { spawnSync } from "node:child_process";
import { existsSync, mkdtempSync, readFileSync, rmSync } from "node:fs";
import { join, resolve } from "node:path";
import { tmpdir } from "node:os";

const repoRoot = resolve(new URL("..", import.meta.url).pathname);

const adapterTests = [
  ["workflow-core", "github_adapter"],
  ["workflow-core", "jira_adapter"],
  ["workflow-core", "ci_adapter"],
];

const exampleTests = [
  ["workflow-cli", "github_read_only_example"],
  ["workflow-cli", "jira_read_only_example"],
  ["workflow-cli", "ci_read_only_example"],
];

const examples = [
  {
    label: "GitHub read-only review context",
    projectDir: "examples/github-read-only-review-context",
    workflowId: "ex/gh",
    reason: "integration-contract-gate-github",
  },
  {
    label: "Jira read-only intake quality",
    projectDir: "examples/jira-read-only-intake-quality",
    workflowId: "ex/jira",
    reason: "integration-contract-gate-jira",
  },
  {
    label: "CI read-only failure summary",
    projectDir: "examples/ci-read-only-failure-summary",
    workflowId: "ex/ci",
    reason: "integration-contract-gate-ci",
  },
];

function main() {
  const cargo = cargoBin();
  const workflowOs = workflowOsBin(cargo);
  checkCiConfig();
  checkAdapterTelemetryPostureDocs();

  for (const [packageName, testName] of [...adapterTests, ...exampleTests]) {
    run(`${packageName} ${testName}`, cargo, ["test", "-p", packageName, "--test", testName]);
  }

  for (const example of examples) {
    checkExample(workflowOs, example);
  }
}

function checkAdapterTelemetryPostureDocs() {
  console.log("checking: adapter telemetry posture is explicit");
  const requiredDocs = [
    "docs/integrations/adapter-contracts.md",
    "docs/concepts/auditability.md",
    "docs/concepts/observability.md",
    "docs/operations/audit-log.md",
    "docs/operations/metrics-and-alerts.md",
    "docs/release/V0_KNOWN_LIMITATIONS.md",
    "examples/github-read-only-review-context/README.md",
    "examples/jira-read-only-intake-quality/README.md",
    "examples/ci-read-only-failure-summary/README.md",
  ];
  for (const path of requiredDocs) {
    const content = readFile(join(repoRoot, path));
    assert(
      content.includes("contract-level adapter telemetry"),
      `${path} must explicitly describe contract-level adapter telemetry`,
    );
    assert(
      content.includes("runtime-visible adapter telemetry"),
      `${path} must explicitly describe scoped runtime-visible adapter telemetry`,
    );
  }

  const exampleDocs = [
    "examples/github-read-only-review-context/README.md",
    "examples/jira-read-only-intake-quality/README.md",
    "examples/ci-read-only-failure-summary/README.md",
  ];
  for (const path of exampleDocs) {
    const content = readFile(join(repoRoot, path));
    assert(
      content.includes("not a generic adapter execution framework"),
      `${path} must describe the fixture telemetry mapping scope`,
    );
    assert(
      content.includes("not production telemetry export"),
      `${path} must explicitly deny production telemetry export`,
    );
  }
}

function checkCiConfig() {
  console.log("checking: CI includes Phase 2 read-only integration gate");
  const ciPath = join(repoRoot, ".github", "workflows", "ci.yml");
  const content = readFile(ciPath);
  assert(
    content.includes("npm run check:integrations"),
    "CI workflow must run npm run check:integrations",
  );
  assert(
    content.includes("Phase 2 Read-Only Integration Contracts"),
    "CI workflow must name the read-only integration contract job",
  );
}

function cargoBin() {
  const localCargo = join(repoRoot, ".tools", "cargo", "bin", "cargo");
  return existsSync(localCargo) ? localCargo : "cargo";
}

function workflowOsBin(cargo) {
  const bin = join(repoRoot, "target", "debug", "workflow-os");
  if (!existsSync(bin)) {
    run("build workflow-os", cargo, ["build", "-p", "workflow-cli", "--bin", "workflow-os"]);
  }
  return bin;
}

function checkExample(workflowOs, example) {
  const project = join(repoRoot, example.projectDir);
  const state = mkdtempSync(join(tmpdir(), "workflow-os-integration-gate-"));
  try {
    run(`${example.label} validate`, workflowOs, ["--project-dir", project, "--state-dir", state, "validate"]);

    const runResult = run(
      `${example.label} fixture run`,
      workflowOs,
      [
        "--project-dir",
        project,
        "--state-dir",
        state,
        "--mock-all-local-skills",
        "run",
        example.workflowId,
      ],
    );
    const runId = extract(runResult.stdout, "run_id");
    const approvalId = extract(runResult.stdout, "approval_id");
    assert(runResult.stdout.includes("status: WaitingForApproval"), `${example.label} did not pause for approval`);

    const approveResult = run(
      `${example.label} fixture approve`,
      workflowOs,
      [
        "--project-dir",
        project,
        "--state-dir",
        state,
        "--mock-all-local-skills",
        "approve",
        runId,
        approvalId,
        "--actor",
        "user/integration-contract-gate",
        "--reason",
        example.reason,
      ],
    );
    assert(approveResult.stdout.includes("status: Completed"), `${example.label} did not complete after approval`);

    const inspectResult = run(
      `${example.label} inspect`,
      workflowOs,
      ["--project-dir", project, "--state-dir", state, "inspect", runId],
    );
    assert(inspectResult.stdout.includes("SkillInvocationSucceeded"), `${example.label} did not record skill success`);
    assert(!containsTokenLikeSecret(inspectResult.stdout), `${example.label} inspect output contains a token-like secret`);
  } finally {
    rmSync(state, { recursive: true, force: true });
  }
}

function run(label, command, args) {
  console.log(`checking: ${label}`);
  const result = spawnSync(command, args, {
    cwd: repoRoot,
    env: commandEnv(),
    encoding: "utf8",
  });
  if (result.status !== 0) {
    throw new Error(
      `${label} failed with status ${result.status}\ncommand: ${command} ${args.join(" ")}\nstdout:\n${result.stdout}\nstderr:\n${result.stderr}`,
    );
  }
  return result;
}

function commandEnv() {
  const localCargoHome = join(repoRoot, ".tools", "cargo");
  const localRustupHome = join(repoRoot, ".tools", "rustup");
  const localCargoBin = join(localCargoHome, "bin");
  const env = { ...process.env };
  if (!env.CARGO_HOME && existsSync(localCargoHome)) {
    env.CARGO_HOME = localCargoHome;
  }
  if (!env.RUSTUP_HOME && existsSync(localRustupHome)) {
    env.RUSTUP_HOME = localRustupHome;
  }
  if (existsSync(localCargoBin)) {
    env.PATH = `${localCargoBin}:${env.PATH ?? ""}`;
  }
  return env;
}

function extract(stdout, key) {
  const prefix = `${key}: `;
  const value = stdout
    .split("\n")
    .find((line) => line.startsWith(prefix))
    ?.slice(prefix.length)
    .trim();
  assert(value, `missing ${key} in command output\n${stdout}`);
  return value;
}

function containsTokenLikeSecret(value) {
  return /gh[pousr]_[A-Za-z0-9_]+|hunter2|secret_token|example_secret_should_be_redacted/i.test(value);
}

function readFile(path) {
  return readFileSync(path, "utf8");
}

function assert(condition, message) {
  if (!condition) {
    throw new Error(message);
  }
}

main();
