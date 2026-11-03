import { createHash } from "node:crypto";
import { execFileSync } from "node:child_process";
import { chmodSync, mkdirSync, readFileSync, statSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";
import { gzipSync } from "node:zlib";

const supportedTargets = new Map([
  ["aarch64-apple-darwin", { hostPlatform: "darwin", hostArch: "arm64", filePattern: /Mach-O.*arm64/i }],
  ["x86_64-unknown-linux-musl", { hostPlatform: "linux", hostArch: "x64", filePattern: /ELF.*x86-64/i }],
]);

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

function releaseVersion(projectDir) {
  const versions = new Map();
  for (const crate of ["workflow-cli", "workflow-core", "workflow-hosted", "workflow-local-host"]) {
    const cargo = readFileSync(join(projectDir, `crates/${crate}/Cargo.toml`), "utf8");
    versions.set(`crate:${crate}`, cargo.match(/^version = "([^"]+)"$/m)?.[1] ?? "missing");
  }
  for (const packagePath of ["package.json", "packages/sdk-typescript/package.json"]) {
    versions.set(`package:${packagePath}`, JSON.parse(readFileSync(join(projectDir, packagePath), "utf8")).version);
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
if [ -e "\${destination}" ] && [ "\${force}" != "true" ]; then echo "refusing to replace \${destination}; rerun with --force" >&2; exit 1; fi
mkdir -p "\${install_dir}"
cp "$(dirname "$0")/workflow-os" "\${destination}"
chmod 755 "\${destination}"
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
  provisional = true,
  verifyBinary = true,
  verifyRepository = true,
}) {
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
  if (verifyBinary) {
    verifyNativeBinary(binaryPath, target, version);
  }
  const rootName = `workflow-os-v${version}-${target}`;
  const manifest = {
    schema_version: "workflowos.dev/release-package/v0",
    product: "workflow-os",
    version,
    target,
    source_commit: sourceCommit.toLowerCase(),
    working_tree_clean: workingTreeClean,
    provisional,
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
  mkdirSync(outputDir, { recursive: true });
  const archiveName = `${rootName}.tar.gz`;
  const archivePath = join(outputDir, archiveName);
  const checksum = sha256(archive);
  writeFileSync(archivePath, archive);
  writeFileSync(`${archivePath}.sha256`, `${checksum}  ${archiveName}\n`, "utf8");
  return { archivePath, checksum, manifest };
}

function parseArguments(argv) {
  const options = { provisional: false };
  for (let index = 0; index < argv.length; index += 1) {
    const argument = argv[index];
    if (argument === "--provisional") {
      options.provisional = true;
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
  if (!options.provisional) {
    throw new Error("Phase 1 packaging requires --provisional; final release packaging is not authorized");
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
    provisional: true,
  });
  chmodSync(result.archivePath, 0o644);
  console.log(`archive: ${result.archivePath}`);
  console.log(`sha256: ${result.checksum}`);
  console.log(`target: ${result.manifest.target}`);
  console.log(`version: ${result.manifest.version}`);
  console.log("posture: provisional_not_for_publication");
}

if (process.argv[1] && import.meta.url === pathToFileURL(resolve(process.argv[1])).href) {
  try {
    main();
  } catch (error) {
    console.error(`error[release.package.failed]: ${error.message}`);
    process.exitCode = 1;
  }
}
