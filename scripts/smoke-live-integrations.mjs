import { spawnSync } from "node:child_process";
import { existsSync } from "node:fs";
import { resolve, join } from "node:path";

const repoRoot = resolve(new URL("..", import.meta.url).pathname);

const providers = {
  github: {
    label: "GitHub read-only live smoke",
    env: ["WORKFLOW_OS_LIVE_GITHUB_TESTS"],
    oneOf: [["WORKFLOW_OS_GITHUB_TOKEN", "GITHUB_TOKEN"]],
    packageName: "workflow-core",
    testTarget: "github_adapter",
    testName: "live_github_repo_metadata_read_is_opt_in",
  },
  jira: {
    label: "Jira read-only live smoke",
    env: ["WORKFLOW_OS_LIVE_JIRA_TESTS", "WORKFLOW_OS_JIRA_BASE_URL", "WORKFLOW_OS_JIRA_TEST_ISSUE_KEY"],
    packageName: "workflow-core",
    testTarget: "jira_adapter",
    testName: "live_jira_issue_metadata_read_is_opt_in",
  },
  ci: {
    label: "GitHub Actions read-only live smoke",
    env: [
      "WORKFLOW_OS_LIVE_GITHUB_ACTIONS_TESTS",
      "WORKFLOW_OS_GITHUB_ACTIONS_TEST_OWNER",
      "WORKFLOW_OS_GITHUB_ACTIONS_TEST_REPO",
      "WORKFLOW_OS_GITHUB_ACTIONS_TEST_RUN_ID",
    ],
    oneOf: [["WORKFLOW_OS_GITHUB_ACTIONS_TOKEN", "GITHUB_TOKEN"]],
    packageName: "workflow-core",
    testTarget: "ci_adapter",
    testName: "live_github_actions_workflow_run_read_is_opt_in",
  },
};

function main() {
  const requested = process.argv.slice(2);
  const selected = requested.length === 0 || requested.includes("all")
    ? ["github", "jira", "ci"]
    : requested;

  for (const provider of selected) {
    if (!providers[provider]) {
      throw new Error(`unknown live smoke provider: ${provider}`);
    }
  }

  for (const provider of selected) {
    runProvider(provider, providers[provider]);
  }
}

function runProvider(provider, config) {
  console.log(`checking: ${config.label}`);
  validateEnv(provider, config);
  runCargoTest(config);
}

function validateEnv(provider, config) {
  const missing = [];
  for (const name of config.env) {
    if (!process.env[name]) {
      missing.push(name);
    }
  }
  for (const group of config.oneOf ?? []) {
    if (!group.some((name) => process.env[name])) {
      missing.push(group.join(" or "));
    }
  }
  if (provider === "jira" && missingAuthMode()) {
    missing.push(
      "(WORKFLOW_OS_JIRA_EMAIL plus WORKFLOW_OS_JIRA_API_TOKEN) or WORKFLOW_OS_JIRA_BEARER_TOKEN",
    );
  }
  if (missing.length > 0) {
    throw new Error(
      `${config.label} is missing required environment: ${missing.join(", ")}. Values were not printed.`,
    );
  }
}

function missingAuthMode() {
  const hasBasicEmail = Boolean(process.env.WORKFLOW_OS_JIRA_EMAIL || process.env.JIRA_EMAIL);
  const hasBasicToken = Boolean(process.env.WORKFLOW_OS_JIRA_API_TOKEN || process.env.JIRA_API_TOKEN);
  const hasBearer = Boolean(process.env.WORKFLOW_OS_JIRA_BEARER_TOKEN || process.env.WORKFLOW_OS_JIRA_TOKEN);
  return !(hasBasicEmail && hasBasicToken) && !hasBearer;
}

function runCargoTest(config) {
  const cargo = cargoBin();
  const result = spawnSync(
    cargo,
    [
      "test",
      "-p",
      config.packageName,
      "--test",
      config.testTarget,
      config.testName,
      "--",
      "--ignored",
      "--exact",
    ],
    {
      cwd: repoRoot,
      env: commandEnv(),
      encoding: "utf8",
    },
  );
  if (result.status !== 0) {
    throw new Error(
      `${config.label} failed with status ${result.status}\nstdout:\n${redact(result.stdout)}\nstderr:\n${redact(result.stderr)}`,
    );
  }
  console.log(redact(result.stdout.trim()));
}

function cargoBin() {
  const localCargo = join(repoRoot, ".tools", "cargo", "bin", "cargo");
  return existsSync(localCargo) ? localCargo : "cargo";
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

function redact(value) {
  let output = value;
  for (const [name, secret] of Object.entries(process.env)) {
    if (!secret || secret.length < 4) {
      continue;
    }
    if (/TOKEN|SECRET|PASSWORD|API_KEY|AUTH/i.test(name)) {
      output = output.split(secret).join("[REDACTED]");
    }
  }
  return output.replace(/gh[pousr]_[A-Za-z0-9_]+/g, "[REDACTED]");
}

main();
