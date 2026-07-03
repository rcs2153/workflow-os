import { spawnSync } from "node:child_process";
import { existsSync, realpathSync } from "node:fs";
import { tmpdir } from "node:os";
import { basename, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const repoRoot = resolve(new URL("..", import.meta.url).pathname);
const dogfoodProject = join(repoRoot, "dogfood", "workflow-os-self-governance");
const defaultStateDir = join(tmpdir(), "workflow-os-self-governance-state");
const workflowId = "dg/d";

const helperErrors = {
  usage: "dogfood.helper.usage",
  binaryMissing: "dogfood.helper.binary_missing",
  commandFailed: "dogfood.helper.command_failed",
  unsupported: "dogfood.helper.unsupported",
};

const commandSpecs = {
  commands: "print this command guide",
  validate: "validate the dogfood project",
  start: "start the dogfood governance workflow",
  "phase-start": "validate and start the mapped governed phase workflow",
  "phase-close": "inspect and summarize a governed phase run",
  status: "show status for a run",
  inspect: "inspect event history for a run",
  approve: "grant an explicit approval checkpoint",
  prompt: "print the agent setup prompt",
};

const phaseWorkflowSpecs = {
  planning: {
    workflowId: "dg/d",
    approvalReason: "approved-planning-phase",
    description: "planning/docs benchmark work",
  },
  docs: {
    workflowId: "dg/d",
    approvalReason: "approved-docs-phase",
    description: "planning/docs benchmark work",
  },
  implementation: {
    workflowId: "dg/implement",
    approvalReason: "approved-implementation-phase",
    description: "accepted implementation phase",
  },
  review: {
    workflowId: "dg/review",
    approvalReason: "approved-review-phase",
    description: "maintainer review phase",
  },
  blocker: {
    workflowId: "dg/blocker",
    approvalReason: "approved-blocker-fix-phase",
    description: "focused blocker fix phase",
  },
  pr: {
    workflowId: "dg/pr",
    approvalReason: "approved-pr-hygiene-phase",
    description: "PR hygiene and conflict-avoidance phase",
  },
  release: {
    workflowId: "dg/release",
    approvalReason: "approved-release-hygiene-phase",
    description: "release hygiene phase",
  },
  "runtime-composition": {
    workflowId: "dg/runtime-composition",
    approvalReason: "approved-runtime-composition-phase",
    description: "runtime-composition phase",
  },
  "branch-cleanup": {
    workflowId: "dg/branch-cleanup",
    approvalReason: "approved-branch-cleanup-phase",
    description: "merged-branch cleanup readiness phase",
  },
  "workflow-discovery": {
    workflowId: "dg/workflow-discovery",
    approvalReason: "approved-workflow-discovery-phase",
    description: "recommendation-only workflow discovery phase",
  },
  "spec-field-operationalization": {
    workflowId: "dg/spec-field-operationalization",
    approvalReason: "approved-spec-field-operationalization-phase",
    description: "scaffold/spec field operationalization phase",
  },
};

export function parseArgs(argv) {
  const command = argv[0] ?? "commands";
  const rest = argv.slice(1);
  const options = {
    actor: "user/dogfood-reviewer",
    dryRun: false,
    json: false,
    noBuild: false,
    phase: undefined,
    reason: undefined,
    runId: undefined,
    stateDir: defaultStateDir,
  };
  const positionals = [];

  for (let index = 0; index < rest.length; index += 1) {
    const value = rest[index];
    switch (value) {
      case "--actor":
        options.actor = readFlagValue(rest, index, value);
        index += 1;
        break;
      case "--dry-run":
        options.dryRun = true;
        break;
      case "--json":
        options.json = true;
        break;
      case "--no-build":
        options.noBuild = true;
        break;
      case "--phase":
        options.phase = readFlagValue(rest, index, value);
        index += 1;
        break;
      case "--reason":
        options.reason = readFlagValue(rest, index, value);
        index += 1;
        break;
      case "--run-id":
        options.runId = readFlagValue(rest, index, value);
        index += 1;
        break;
      case "--state-dir":
        options.stateDir = resolve(readFlagValue(rest, index, value));
        index += 1;
        break;
      default:
        if (value.startsWith("--")) {
          throw helperError(helperErrors.usage, `unknown option ${value}`);
        }
        positionals.push(value);
        break;
    }
  }

  if (containsSecretLike(options.reason) || containsSecretLike(options.actor)) {
    throw helperError(
      helperErrors.usage,
      "secret-like approval metadata is not allowed in benchmark helper arguments",
    );
  }
  if (containsSecretLike(options.phase)) {
    throw helperError(helperErrors.usage, "secret-like phase value is not allowed");
  }

  return { command, options, positionals };
}

export function buildWorkflowCommand(parsed, workflowOsBin = workflowOsPath()) {
  const { command, options, positionals } = parsed;
  const base = [
    workflowOsBin,
    "--project-dir",
    dogfoodProject,
    "--state-dir",
    options.stateDir,
  ];
  if (options.json) {
    base.push("--json");
  }

  switch (command) {
    case "validate":
      return [...base, "validate"];
    case "start": {
      const args = [...base, "--mock-all-local-skills", "run", workflowId];
      if (options.runId) {
        args.push("--run-id", options.runId);
      }
      return args;
    }
    case "phase-start": {
      const phase = phaseWorkflow(options.phase);
      const args = [...base, "--mock-all-local-skills", "run", phase.workflowId];
      if (options.runId) {
        args.push("--run-id", options.runId);
      }
      return args;
    }
    case "phase-close":
      requirePositionals(positionals, 1, "phase-close requires <run-id>");
      return [...base, "inspect", positionals[0]];
    case "status":
      requirePositionals(positionals, 1, "status requires <run-id>");
      return [...base, "status", positionals[0]];
    case "inspect":
      requirePositionals(positionals, 1, "inspect requires <run-id>");
      return [...base, "inspect", positionals[0]];
    case "approve":
      requirePositionals(positionals, 2, "approve requires <run-id> <approval-id>");
      if (!options.reason) {
        throw helperError(helperErrors.usage, "approve requires --reason");
      }
      return [
        ...base,
        "--mock-all-local-skills",
        "approve",
        positionals[0],
        positionals[1],
        "--actor",
        options.actor,
        "--reason",
        options.reason,
      ];
    default:
      throw helperError(helperErrors.unsupported, "unsupported helper command");
  }
}

export function displayCommand(args) {
  const displayed = args.map((arg, index) => {
    if (args[index - 1] === "--reason") {
      return "<redacted-reason>";
    }
    return shellQuote(displayPath(arg));
  });
  return displayed.join(" ");
}

export function containsSecretLike(value) {
  if (!value) {
    return false;
  }
  return /(?:authorization|bearer|credential|password|private[_-]?key|secret|token|api[_-]?key|sk-[a-z0-9_-]+)/i.test(
    value,
  );
}

export function main(argv = process.argv.slice(2)) {
  try {
    const parsed = parseArgs(argv);
    return runParsed(parsed);
  } catch (error) {
    if (error instanceof HelperError) {
      console.error(`error[${error.code}]: ${error.message}`);
      return 2;
    }
    console.error(`error[${helperErrors.commandFailed}]: benchmark helper failed`);
    return 1;
  }
}

function runParsed(parsed) {
  switch (parsed.command) {
    case "commands":
      printCommands();
      return 0;
    case "prompt":
      printPrompt();
      return 0;
    case "validate":
    case "start":
    case "status":
    case "inspect":
    case "approve":
      return runWorkflowCommand(parsed);
    case "phase-start":
      return runPhaseStart(parsed);
    case "phase-close":
      return runPhaseClose(parsed);
    default:
      throw helperError(helperErrors.unsupported, "unsupported helper command");
  }
}

function runWorkflowCommand(parsed) {
  const bin = ensureWorkflowOsBinary(parsed.options);
  const command = buildWorkflowCommand(parsed, bin);
  printBoundary(parsed.options);
  console.log(`command: ${displayCommand(command)}`);
  if (parsed.options.dryRun) {
    console.log("dry_run: true");
    return 0;
  }

  const [executable, ...args] = command;
  const result = spawnSync(executable, args, {
    cwd: repoRoot,
    env: commandEnv(),
    stdio: "inherit",
  });
  if (result.status !== 0) {
    throw helperError(
      helperErrors.commandFailed,
      `${parsed.command} failed with status ${result.status ?? "unknown"}`,
    );
  }
  printNextStep(parsed.command);
  return 0;
}

function runPhaseStart(parsed) {
  const { options } = parsed;
  const phase = phaseWorkflow(options.phase);
  const bin = ensureWorkflowOsBinary(options);
  const validateCommand = workflowBaseCommand(bin, options, false).concat("validate");
  const runCommand = buildWorkflowCommand(parsed, bin);

  printBoundary(options);
  printPhaseBoundary(options.phase, phase);
  console.log(`validation_command: ${displayCommand(validateCommand)}`);
  console.log(`start_command: ${displayCommand(runCommand)}`);
  if (options.dryRun) {
    console.log("dry_run: true");
    console.log("approval_outcome: not_requested");
    console.log("next_action: run without --dry-run, review the printed scope, then approve explicitly");
    return 0;
  }

  runCapturedCommand("validation", validateCommand);
  const runOutput = runCapturedCommand("phase_start", runCommand);
  const summary = parseRunSummary(runOutput.stdout);
  console.log("governed_phase_started: true");
  console.log(`phase: ${options.phase}`);
  console.log(`workflow_id: ${phase.workflowId}`);
  console.log(`run_id: ${summary.runId ?? "unknown"}`);
  console.log(`status: ${summary.status ?? "unknown"}`);
  console.log(`approval_id: ${summary.approvalId ?? "not_available"}`);
  console.log("approval_outcome: pending");
  console.log("approval_required: true");
  if (summary.runId && summary.approvalId) {
    const approveCommand = workflowBaseCommand(bin, options, false).concat(
      "--mock-all-local-skills",
      "approve",
      summary.runId,
      summary.approvalId,
      "--actor",
      options.actor,
      "--reason",
      phase.approvalReason,
    );
    console.log("next_action: review phase scope, then run the approval command explicitly");
    console.log(`approval_command: ${displayCommand(approveCommand)}`);
  } else {
    console.log("next_action: inspect run output before approval; approval ID was not detected");
  }
  return 0;
}

function runPhaseClose(parsed) {
  const { options, positionals } = parsed;
  requirePositionals(positionals, 1, "phase-close requires <run-id>");
  const runId = positionals[0];
  const bin = ensureWorkflowOsBinary(options);
  const statusCommand = workflowBaseCommand(bin, options, true).concat("status", runId);
  const inspectCommand = workflowBaseCommand(bin, options, true).concat("inspect", runId);

  printBoundary(options);
  if (options.phase) {
    printPhaseBoundary(options.phase, phaseWorkflow(options.phase));
  }
  console.log(`run_id: ${runId}`);
  console.log(`status_command: ${displayCommand(statusCommand)}`);
  console.log(`inspect_command: ${displayCommand(inspectCommand)}`);
  if (options.dryRun) {
    console.log("dry_run: true");
    console.log("next_action: run without --dry-run after implementation/review work is complete");
    return 0;
  }

  const statusOutput = runCapturedCommand("status", statusCommand, { printOutput: false });
  const inspectOutput = runCapturedCommand("inspect", inspectCommand, { printOutput: false });
  const status = parseJsonOutput(statusOutput.stdout, "status");
  const inspection = parseJsonOutput(inspectOutput.stdout, "inspect");
  const eventCounts = countEventsByKind(inspection.events ?? []);
  console.log("governed_phase_close_summary: true");
  console.log(`workflow_id: ${status.workflow_id ?? "not_available"}`);
  console.log(`status: ${status.status ?? inspection.status ?? "unknown"}`);
  console.log(`terminal: ${status.terminal === true ? "true" : "false"}`);
  console.log(`events_total: ${(inspection.events ?? []).length}`);
  console.log(`approvals: ${inspection.approvals ?? 0}`);
  console.log(`retries: ${inspection.retries ?? 0}`);
  console.log(`escalations: ${inspection.escalations ?? 0}`);
  console.log(`event_kinds: ${formatEventCounts(eventCounts)}`);
  console.log("phase_report_required: true");
  console.log("phase_report_fields: dogfood_workflow_id, run_id, approval_id, approval_outcome, event_summary, validation_summary, out_of_kernel_work");
  console.log("out_of_kernel_work_disclosure: disclose repo edits, shell commands, validation commands, skipped checks, git/PR actions, and report posture performed outside the kernel");
  console.log("missing_coverage_policy: disclose missing handler/check/report coverage; do not simulate it");
  return 0;
}

function ensureWorkflowOsBinary(options) {
  const bin = workflowOsPath();
  if (existsSync(bin)) {
    return bin;
  }
  if (options.noBuild) {
    throw helperError(
      helperErrors.binaryMissing,
      "target/debug/workflow-os is missing; rerun without --no-build to build it",
    );
  }

  const cargo = cargoPath();
  const buildCommand = [cargo, "build", "-p", "workflow-cli", "--bin", "workflow-os"];
  console.log(`build: ${displayCommand(buildCommand)}`);
  if (options.dryRun) {
    return bin;
  }
  const result = spawnSync(cargo, ["build", "-p", "workflow-cli", "--bin", "workflow-os"], {
    cwd: repoRoot,
    env: commandEnv(),
    stdio: "inherit",
  });
  if (result.status !== 0) {
    throw helperError(
      helperErrors.commandFailed,
      `workflow-os build failed with status ${result.status ?? "unknown"}`,
    );
  }
  return bin;
}

function printCommands() {
  console.log("Self-Governed Build Benchmark helper");
  console.log();
  console.log("Usage:");
  for (const [command, description] of Object.entries(commandSpecs)) {
    console.log(`  npm run dogfood:benchmark -- ${command.padEnd(8)} # ${description}`);
  }
  console.log();
  console.log("Examples:");
  console.log("  npm run dogfood:benchmark -- validate");
  console.log("  npm run dogfood:benchmark -- start --state-dir /tmp/workflow-os-self-governance-state");
  console.log("  npm run dogfood:benchmark -- phase-start --phase implementation");
  console.log("  npm run dogfood:benchmark -- phase-close <run-id> --phase implementation");
  console.log("  npm run dogfood:benchmark -- approve <run-id> <approval-id> --reason reviewed-governance-task");
  console.log("  npm run dogfood:benchmark -- inspect <run-id>");
  console.log();
  console.log("Phase types:");
  for (const [phase, spec] of Object.entries(phaseWorkflowSpecs)) {
    console.log(`  ${phase.padEnd(28)} -> ${spec.workflowId.padEnd(30)} # ${spec.description}`);
  }
  console.log();
  console.log("Boundary: repo-local development helper only; no hidden approval, no automatic approval, local checks, reports, artifacts, writes, git operations, PR actions, shell execution, or schema changes.");
}

function printPrompt() {
  console.log("Use Workflow OS as the governing layer for this Workflow OS phase.");
  console.log();
  console.log("Agent executes. Workflow OS governs.");
  console.log();
  console.log("Before editing, read docs/ENGINEERING_STANDARD.md, ROADMAP.md, and the relevant plan/report/review docs.");
  console.log("Start or resume the governed dogfood workflow when this phase requires dogfooding.");
  console.log("Treat approval checkpoints as mandatory.");
  console.log("Run required validation outside the kernel unless a reviewed explicit handler exists.");
  console.log("Do not invent run IDs, approvals, evidence, local check results, reports, or command output.");
  console.log("Runbook: docs/user-guide/self-governed-build-benchmark.md");
}

function printBoundary(options) {
  console.log(`project_dir: ${relativeToRepo(dogfoodProject)}`);
  console.log(`state_dir: ${options.stateDir}`);
  console.log("helper_scope: repo-local development helper");
}

function printPhaseBoundary(phase, spec) {
  console.log(`phase: ${phase}`);
  console.log(`workflow_id: ${spec.workflowId}`);
  console.log(`phase_description: ${spec.description}`);
  console.log("approval_policy: explicit_human_approval_required");
  console.log("runner_boundary: governance coordination only");
}

function printNextStep(command) {
  if (command === "validate") {
    console.log("next: npm run dogfood:benchmark -- start");
  } else if (command === "start") {
    console.log("next: approve the printed approval_id only after reviewing scope");
  } else if (command === "approve") {
    console.log("next: npm run dogfood:benchmark -- inspect <run-id>");
  }
}

function readFlagValue(args, index, flag) {
  const value = args[index + 1];
  if (!value || value.startsWith("--")) {
    throw helperError(helperErrors.usage, `${flag} requires a value`);
  }
  return value;
}

function requirePositionals(positionals, count, message) {
  if (positionals.length < count) {
    throw helperError(helperErrors.usage, message);
  }
}

function workflowOsPath() {
  return join(repoRoot, "target", "debug", process.platform === "win32" ? "workflow-os.exe" : "workflow-os");
}

function workflowBaseCommand(workflowOsBin, options, forceJson) {
  const args = [
    workflowOsBin,
    "--project-dir",
    dogfoodProject,
    "--state-dir",
    options.stateDir,
  ];
  if (forceJson || options.json) {
    args.push("--json");
  }
  return args;
}

function cargoPath() {
  const localCargo = join(repoRoot, ".tools", "cargo", "bin", process.platform === "win32" ? "cargo.exe" : "cargo");
  return existsSync(localCargo) ? localCargo : "cargo";
}

function commandEnv() {
  const env = { ...process.env };
  const localCargoHome = join(repoRoot, ".tools", "cargo");
  const localRustupHome = join(repoRoot, ".tools", "rustup");
  const localCargoBin = join(localCargoHome, "bin");
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

function runCapturedCommand(label, command, { printOutput = true } = {}) {
  const [executable, ...args] = command;
  const result = spawnSync(executable, args, {
    cwd: repoRoot,
    env: commandEnv(),
    encoding: "utf8",
  });
  if (printOutput && result.stdout) {
    process.stdout.write(result.stdout);
  }
  if (printOutput && result.stderr) {
    process.stderr.write(result.stderr);
  }
  if (result.status !== 0) {
    throw helperError(
      helperErrors.commandFailed,
      `${label} failed with status ${result.status ?? "unknown"}`,
    );
  }
  return result;
}

function phaseWorkflow(phase) {
  if (!phase) {
    throw helperError(
      helperErrors.usage,
      `--phase is required; expected one of ${Object.keys(phaseWorkflowSpecs).join(", ")}`,
    );
  }
  const spec = phaseWorkflowSpecs[phase];
  if (!spec) {
    throw helperError(
      helperErrors.usage,
      `unsupported phase; expected one of ${Object.keys(phaseWorkflowSpecs).join(", ")}`,
    );
  }
  return spec;
}

function parseRunSummary(output) {
  return {
    runId: matchLine(output, /^run_id:\s*(.+)$/m),
    status: matchLine(output, /^status:\s*(.+)$/m),
    approvalId: matchLine(output, /^approval_id:\s*(.+)$/m),
  };
}

function matchLine(output, pattern) {
  const match = output.match(pattern);
  return match ? match[1].trim() : undefined;
}

function parseJsonOutput(output, label) {
  const trimmed = output.trim().split(/\r?\n/).filter(Boolean).at(-1) ?? "";
  try {
    return JSON.parse(trimmed);
  } catch {
    throw helperError(helperErrors.commandFailed, `${label} JSON output could not be parsed`);
  }
}

function countEventsByKind(events) {
  const counts = new Map();
  for (const event of events) {
    const kind = String(event.kind ?? "Unknown");
    counts.set(kind, (counts.get(kind) ?? 0) + 1);
  }
  return counts;
}

function formatEventCounts(counts) {
  if (counts.size === 0) {
    return "none";
  }
  return [...counts.entries()]
    .sort(([left], [right]) => left.localeCompare(right))
    .map(([kind, count]) => `${kind}:${count}`)
    .join(",");
}

function shellQuote(value) {
  if (/^[A-Za-z0-9_./:=@+-]+$/.test(value)) {
    return value;
  }
  return `'${value.replaceAll("'", "'\\''")}'`;
}

function relativeToRepo(path) {
  if (path.startsWith(repoRoot)) {
    return path.slice(repoRoot.length + 1);
  }
  return basename(path);
}

function displayPath(value) {
  if (value === repoRoot) {
    return ".";
  }
  if (value.startsWith(`${repoRoot}/`)) {
    return `.${value.slice(repoRoot.length)}`;
  }
  return value;
}

function helperError(code, message) {
  return new HelperError(code, message);
}

class HelperError extends Error {
  constructor(code, message) {
    super(message);
    this.code = code;
  }
}

if (isEntrypoint()) {
  process.exitCode = main();
}

function isEntrypoint() {
  if (!process.argv[1]) {
    return false;
  }
  return realpathSync(fileURLToPath(import.meta.url)) === realpathSync(process.argv[1]);
}
