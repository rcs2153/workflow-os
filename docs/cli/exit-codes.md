# CLI Exit Codes

`workflow-os` uses stable v0 exit code categories.

| Code | Meaning |
| --- | --- |
| `0` | Command completed successfully. |
| `1` | Parse or validation failure. |
| `2` | CLI usage error or unsupported command shape. |
| `3` | Runtime, policy, security, invalid state, or internal error. |

Diagnostics are printed for validation failures where available. Machine-readable output is available for selected commands with `--json`.
