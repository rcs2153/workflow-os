import { spawnSync } from "node:child_process";
import { existsSync } from "node:fs";
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
  status: "show status for a run",
  inspect: "inspect event history for a run",
  approve: "grant an explicit approval checkpoint",
  prompt: "print the agent setup prompt",
};

export function parseArgs(argv) {
  const command = argv[0] ?? "commands";
  const rest = argv.slice(1);
  const options = {
    actor: "user/dogfood-reviewer",
    dryRun: false,
    json: false,
    noBuild: false,
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
      throw helperError(helperErrors.unsupported, `unsupported helper command ${command}`);
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
    default:
      throw helperError(helperErrors.unsupported, `unsupported helper command ${parsed.command}`);
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
  console.log("  npm run dogfood:benchmark -- approve <run-id> <approval-id> --reason reviewed-governance-task");
  console.log("  npm run dogfood:benchmark -- inspect <run-id>");
  console.log();
  console.log("Boundary: repo-local development helper only; no automatic approval, local checks, reports, artifacts, writes, or schema changes.");
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

if (process.argv[1] && resolve(fileURLToPath(import.meta.url)) === resolve(process.argv[1])) {
  process.exitCode = main();
}
