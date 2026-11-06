import { createHash, randomUUID } from "node:crypto";
import { execFileSync } from "node:child_process";
import {
  chmodSync,
  existsSync,
  linkSync,
  mkdirSync,
  readFileSync,
  rmSync,
  statSync,
  writeFileSync,
} from "node:fs";
import { join, resolve } from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";
import { gzipSync } from "node:zlib";

const supportedTargets = new Map([
  ["aarch64-apple-darwin", { hostPlatform: "darwin", hostArch: "arm64", filePattern: /Mach-O.*arm64/i }],
  [
    "x86_64-unknown-linux-musl",
    { hostPlatform: "linux", hostArch: "x64", filePattern: /ELF.*x86-64.*static/i },
  ],
]);

export const releasePackagePostures = Object.freeze({
  provisionalRehearsal: "provisional_rehearsal",
  finalCandidate: "final_candidate",
});

const validPostures = new Set(Object.values(releasePackagePostures));

function sha256(buffer) {
  return createHash("sha256").update(buffer).digest("hex");
}

function writeString(header, offset, length, value) {
  const encoded = Buffer.from(value, "utf8");
  if (encoded.length > length) {
    throw new Error(`tar header value is too long: ${value}`);
  }
  encoded.copy(header, offset);
}

function writeOctal(header, offset, length, value) {
  const encoded = value.toString(8).padStart(length - 1, "0");
  writeString(header, offset, length, `${encoded}\0`);
}

function tarHeader({ path, mode, size, type }) {
  const header = Buffer.alloc(512);
  writeString(header, 0, 100, path);
  writeOctal(header, 100, 8, mode);
  writeOctal(header, 108, 8, 0);
  writeOctal(header, 116, 8, 0);
  writeOctal(header, 124, 12, size);
  writeOctal(header, 136, 12, 0);
  header.fill(0x20, 148, 156);
  writeString(header, 156, 1, type);
  writeString(header, 257, 6, "ustar\0");
  writeString(header, 263, 2, "00");
  writeString(header, 265, 32, "root");
  writeString(header, 297, 32, "root");
  const checksum = header.reduce((sum, byte) => sum + byte, 0);
  writeString(header, 148, 8, `${checksum.toString(8).padStart(6, "0")}\0 `);
  return header;
}

function deterministicTarGzip(rootName, files) {
  const chunks = [tarHeader({ path: `${rootName}/`, mode: 0o755, size: 0, type: "5" })];
  for (const file of [...files].sort((left, right) => left.path.localeCompare(right.path))) {
    const content = Buffer.isBuffer(file.content) ? file.content : Buffer.from(file.content, "utf8");
    chunks.push(
      tarHeader({ path: `${rootName}/${file.path}`, mode: file.mode, size: content.length, type: "0" }),
      content,
    );
    const padding = (512 - (content.length % 512)) % 512;
    if (padding > 0) {
      chunks.push(Buffer.alloc(padding));
    }
  }
  chunks.push(Buffer.alloc(1024));
  return gzipSync(Buffer.concat(chunks), { level: 9, mtime: 0 });
}

function cargoLockVersions(projectDir) {
  const selectedCrates = new Set(["workflow-cli", "workflow-core", "workflow-hosted", "workflow-local-host"]);
  const versions = new Map();
  const cargoLock = readFileSync(join(projectDir, "Cargo.lock"), "utf8");
  for (const block of cargoLock.split("[[package]]").slice(1)) {
    const name = block.match(/^name = "([^"]+)"$/m)?.[1];
    const version = block.match(/^version = "([^"]+)"$/m)?.[1];
    if (name && version && selectedCrates.has(name)) {
      versions.set(`lock:cargo:${name}`, version);
    }
  }
  for (const crate of selectedCrates) {
    if (!versions.has(`lock:cargo:${crate}`)) {
      versions.set(`lock:cargo:${crate}`, "missing");
    }
  }
  return versions;
}

function releaseVersion(projectDir) {
  const versions = new Map();
  for (const crate of ["workflow-cli", "workflow-core", "workflow-hosted", "workflow-local-host"]) {
    const cargo = readFileSync(join(projectDir, `crates/${crate}/Cargo.toml`), "utf8");
    versions.set(`crate:${crate}`, cargo.match(/^version = "([^"]+)"$/m)?.[1] ?? "missing");
  }
  for (const packagePath of ["package.json", "packages/sdk-typescript/package.json"]) {
    versions.set(`package:${packagePath}`, JSON.parse(readFileSync(join(projectDir, packagePath), "utf8")).version);
  }
  const packageLock = JSON.parse(readFileSync(join(projectDir, "package-lock.json"), "utf8"));
  versions.set("lock:npm:top-level", packageLock.version ?? "missing");
  versions.set("lock:npm:root", packageLock.packages?.[""]?.version ?? "missing");
  versions.set(
    "lock:npm:packages/sdk-typescript",
    packageLock.packages?.["packages/sdk-typescript"]?.version ?? "missing",
  );
  for (const [name, version] of cargoLockVersions(projectDir)) {
    versions.set(name, version);
  }
  const uniqueVersions = new Set(versions.values());
  if (uniqueVersions.size !== 1 || uniqueVersions.has("missing")) {
    throw new Error(
      `release version mismatch: ${[...versions.entries()].map(([name, value]) => `${name}=${value}`).join(" ")}`,
    );
  }
  return versions.get("crate:workflow-cli");
}

function verifySourceCheckout(projectDir, sourceCommit) {
  const head = execFileSync("git", ["rev-parse", "HEAD"], { cwd: projectDir, encoding: "utf8" }).trim();
  if (head.toLowerCase() !== sourceCommit.toLowerCase()) {
    throw new Error(`source commit mismatch: checkout=${head} requested=${sourceCommit}`);
  }
  return execFileSync("git", ["status", "--porcelain"], { cwd: projectDir, encoding: "utf8" }).trim() === "";
}

function installScript(version) {
  return `#!/bin/sh
set -eu
install_dir="\${WORKFLOW_OS_INSTALL_DIR:-\${HOME}/.local/bin}"
force=false
if [ "$#" -gt 1 ]; then echo "usage: ./install.sh [--force]" >&2; exit 2; fi
if [ "\${1:-}" = "--force" ]; then force=true; elif [ "\${1:-}" != "" ]; then echo "usage: ./install.sh [--force]" >&2; exit 2; fi
destination="\${install_dir}/workflow-os"
mkdir -p "\${install_dir}"
if [ -L "\${destination}" ]; then echo "refusing destination symlink \${destination}" >&2; exit 1; fi
if [ -e "\${destination}" ] && [ ! -f "\${destination}" ]; then echo "refusing non-regular destination \${destination}" >&2; exit 1; fi
if [ -e "\${destination}" ] && [ "\${force}" != "true" ]; then echo "refusing to replace \${destination}; rerun with --force" >&2; exit 1; fi
umask 077
temporary="$(mktemp "\${install_dir}/.workflow-os.install.XXXXXX")"
trap 'rm -f "\${temporary}"' 0 HUP INT TERM
cp "$(dirname "$0")/workflow-os" "\${temporary}"
chmod 755 "\${temporary}"
if [ "\${force}" = "true" ] && [ -e "\${destination}" ]; then
  rm -f "\${destination}"
fi
if ! ln "\${temporary}" "\${destination}" 2>/dev/null; then
  echo "refusing to replace \${destination}; rerun with --force" >&2
  exit 1
fi
rm "\${temporary}"
trap - 0 HUP INT TERM
echo "installed workflow-os ${version} at \${destination}"
`;
}

function uninstallScript(version) {
  return `#!/bin/sh
set -eu
install_dir="\${WORKFLOW_OS_INSTALL_DIR:-\${HOME}/.local/bin}"
destination="\${install_dir}/workflow-os"
if [ ! -e "\${destination}" ]; then echo "workflow-os is not installed at \${destination}"; exit 0; fi
rm "\${destination}"
echo "uninstalled workflow-os ${version} from \${destination}"
echo "local .workflow-os state was not removed"
`;
}

function verifyNativeBinary(binaryPath, target, version) {
  const targetContract = supportedTargets.get(target);
  if (!targetContract) {
    throw new Error(`unsupported release target: ${target}`);
  }
  if (process.platform !== targetContract.hostPlatform || process.arch !== targetContract.hostArch) {
    throw new Error(
      `target ${target} must be packaged on its native host (${targetContract.hostPlatform}/${targetContract.hostArch}); current host is ${process.platform}/${process.arch}`,
    );
  }
  const fileOutput = execFileSync("file", [binaryPath], { encoding: "utf8" });
  if (!targetContract.filePattern.test(fileOutput)) {
    throw new Error(`binary does not match target ${target}: ${fileOutput.trim()}`);
  }
  const versionOutput = execFileSync(binaryPath, ["--version"], { encoding: "utf8" }).trim();
  if (versionOutput !== `workflow-os ${version}`) {
    throw new Error(`binary version mismatch: expected workflow-os ${version}, received ${versionOutput}`);
  }
}

export function packageRelease({
  projectDir,
  binaryPath,
  outputDir,
  target,
  sourceCommit,
  posture,
  verifyBinary = true,
  verifyRepository = true,
}) {
  if (!validPostures.has(posture)) {
    throw new Error(
      `release package posture must be exactly one of: ${[...validPostures].join(", ")}`,
    );
  }
  if (
    posture === releasePackagePostures.finalCandidate &&
    (!verifyBinary || !verifyRepository)
  ) {
    throw new Error("final candidate admission cannot disable binary or repository verification");
  }
  if (!/^[0-9a-f]{40}$/i.test(sourceCommit)) {
    throw new Error("source commit must be an exact 40-character Git commit SHA");
  }
  if (!supportedTargets.has(target)) {
    throw new Error(`unsupported release target: ${target}`);
  }
  const binary = readFileSync(binaryPath);
  if (!statSync(binaryPath).isFile()) {
    throw new Error(`release binary is not a file: ${binaryPath}`);
  }
  const version = releaseVersion(projectDir);
  const workingTreeClean = verifyRepository ? verifySourceCheckout(projectDir, sourceCommit) : false;
  if (posture === releasePackagePostures.finalCandidate && !workingTreeClean) {
    throw new Error("final candidate requires a clean exact source checkout");
  }
  if (verifyBinary) {
    verifyNativeBinary(binaryPath, target, version);
  }
  const rootName = `workflow-os-v${version}-${target}`;
  const manifest = {
    schema_version: "workflowos.dev/release-package/v1",
    product: "workflow-os",
    version,
    target,
    source_commit: sourceCommit.toLowerCase(),
    working_tree_clean: workingTreeClean,
    posture,
    binary_sha256: sha256(binary),
    install_default: "$HOME/.local/bin/workflow-os",
    install_override: "WORKFLOW_OS_INSTALL_DIR",
    state_posture: "install and uninstall do not create or remove repository .workflow-os state",
  };
  const archive = deterministicTarGzip(rootName, [
    { path: "LICENSE", mode: 0o644, content: readFileSync(join(projectDir, "LICENSE")) },
    { path: "PACKAGE-MANIFEST.json", mode: 0o644, content: `${JSON.stringify(manifest, null, 2)}\n` },
    { path: "README.md", mode: 0o644, content: readFileSync(join(projectDir, "README.md")) },
    { path: "install.sh", mode: 0o755, content: installScript(version) },
    { path: "uninstall.sh", mode: 0o755, content: uninstallScript(version) },
    { path: "workflow-os", mode: 0o755, content: binary },
  ]);
  const archiveName = `${rootName}.tar.gz`;
  const archivePath = join(outputDir, archiveName);
  const checksumPath = `${archivePath}.sha256`;
  if (existsSync(archivePath) || existsSync(checksumPath)) {
    throw new Error(`release output already exists: ${existsSync(archivePath) ? archivePath : checksumPath}`);
  }
  const checksum = sha256(archive);
  mkdirSync(outputDir, { recursive: true });
  const temporarySuffix = `.tmp-${process.pid}-${randomUUID()}`;
  const temporaryArchivePath = `${archivePath}${temporarySuffix}`;
  const temporaryChecksumPath = `${checksumPath}${temporarySuffix}`;
  let archiveLinked = false;
  let checksumLinked = false;
  try {
    writeFileSync(temporaryArchivePath, archive, { flag: "wx" });
    writeFileSync(temporaryChecksumPath, `${checksum}  ${archiveName}\n`, { encoding: "utf8", flag: "wx" });
    linkSync(temporaryArchivePath, archivePath);
    archiveLinked = true;
    linkSync(temporaryChecksumPath, checksumPath);
    checksumLinked = true;
    rmSync(temporaryArchivePath);
    rmSync(temporaryChecksumPath);
  } catch (error) {
    rmSync(temporaryArchivePath, { force: true });
    rmSync(temporaryChecksumPath, { force: true });
    if (archiveLinked) {
      rmSync(archivePath, { force: true });
    }
    if (checksumLinked) {
      rmSync(checksumPath, { force: true });
    }
    throw error;
  }
  return { archivePath, checksum, manifest };
}

function parseArguments(argv) {
  const options = { posture: undefined };
  for (let index = 0; index < argv.length; index += 1) {
    const argument = argv[index];
    if (argument === "--provisional") {
      if (options.posture) {
        throw new Error("release package posture flags are mutually exclusive");
      }
      options.posture = releasePackagePostures.provisionalRehearsal;
      continue;
    }
    if (argument === "--final-candidate") {
      if (options.posture) {
        throw new Error("release package posture flags are mutually exclusive");
      }
      options.posture = releasePackagePostures.finalCandidate;
      continue;
    }
    if (!["--binary", "--output-dir", "--source-commit", "--target"].includes(argument)) {
      throw new Error(`unknown option: ${argument}`);
    }
    const value = argv[index + 1];
    if (!value) {
      throw new Error(`missing value for ${argument}`);
    }
    options[argument.slice(2).replaceAll("-", "_")] = value;
    index += 1;
  }
  for (const required of ["binary", "output_dir", "source_commit", "target"]) {
    if (!options[required]) {
      throw new Error(`missing required option --${required.replaceAll("_", "-")}`);
    }
  }
  if (!options.posture) {
    throw new Error("exactly one release package posture is required: --provisional or --final-candidate");
  }
  return options;
}

function main() {
  const projectDir = resolve(fileURLToPath(new URL("..", import.meta.url)));
  const options = parseArguments(process.argv.slice(2));
  const result = packageRelease({
    projectDir,
    binaryPath: resolve(options.binary),
    outputDir: resolve(options.output_dir),
    target: options.target,
    sourceCommit: options.source_commit,
    posture: options.posture,
  });
  chmodSync(result.archivePath, 0o644);
  console.log(`archive: ${result.archivePath}`);
  console.log(`sha256: ${result.checksum}`);
  console.log(`target: ${result.manifest.target}`);
  console.log(`version: ${result.manifest.version}`);
  console.log(`posture: ${result.manifest.posture}`);
}

if (process.argv[1] && import.meta.url === pathToFileURL(resolve(process.argv[1])).href) {
  try {
    main();
  } catch (error) {
    console.error(`error[release.package.failed]: ${error.message}`);
    process.exitCode = 1;
  }
}
