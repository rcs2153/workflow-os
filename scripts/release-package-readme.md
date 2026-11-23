# Workflow OS @VERSION@

This archive contains the prebuilt Workflow OS CLI for `@TARGET@`.
No Rust, Node.js, or source checkout is needed to use the binary.
See [PACKAGE-MANIFEST.json](PACKAGE-MANIFEST.json) for source and binary identity.

## Install

In the extracted archive directory, run:

```sh
./install.sh
export PATH="$HOME/.local/bin:$PATH"
workflow-os --version
```

The default destination is `$HOME/.local/bin/workflow-os`. The PATH change is
for the current shell. To use a different directory, set it before installing
and keep that same override when uninstalling:

```sh
export WORKFLOW_OS_INSTALL_DIR="$HOME/tools/workflow-os-bin"
./install.sh
export PATH="$WORKFLOW_OS_INSTALL_DIR:$PATH"
workflow-os --version
```

The installer refuses an existing binary unless you explicitly use
`./install.sh --force`. It does not replace symlinks or directories.

## First Run

From a project you want to govern, preview and then create the local scaffold.
Omit the second command if you only want to see what would change.

```sh
workflow-os init-repo-governance --agent codex --dry-run
workflow-os init-repo-governance --agent codex
workflow-os validate
workflow-os first-run
```

Existing unmanaged AGENTS.md guidance is preserved. The first-run summary
analyzes bounded project metadata; it does not execute project build/test
commands or contact a provider. To write a governed local WorkReport, run
`workflow-os first-run --write-report-artifact`.

## Uninstall

Return to the extracted archive directory and run `./uninstall.sh`. If you
installed to a custom directory, set `WORKFLOW_OS_INSTALL_DIR` to that directory
again first. Uninstall removes the installed binary, not repository state.

## Documentation

- [Release scope](@SOURCE@/docs/release/@VERSION@_RELEASE_SCOPE_DECISION.md)
- [Current product contract](@SOURCE@/docs/user-guide/current-product-contract.md)
- [Agent harness quickstart](@SOURCE@/docs/user-guide/agent-harness-quickstart.md)
- [Workflow OS explainer](@SOURCE@/docs/concepts/WORKFLOW_OS_EXPLAINER.md)
- [Approval example](@SOURCE@/examples/vertical-slice-approval/README.md)
- [Repository metadata fixture example](@SOURCE@/examples/github-repository-metadata-fixture/workflows/read.workflow.yml)
- [Engineering standard](@SOURCE@/docs/ENGINEERING_STANDARD.md)
- [Roadmap](@SOURCE@/ROADMAP.md)
- [Contributing](@SOURCE@/CONTRIBUTING.md)
- [Security policy](@SOURCE@/SECURITY.md)
- [License](LICENSE)
