# WorkReport Body Commitment Read-Path Enforcement Review

## Verdict

Accepted for the bounded ordinary read/list scope.

The implementation closes the approved report-only substitution gap across all
supported artifact stores without broadening serde, migration, inspection,
recovery, replay, or product surfaces.

## Reviewed Properties

1. Local filesystem, SQLite, and PostgreSQL reads invoke one shared commitment
   validator before returning new-format artifact content.
2. List operations enforce the same boundary for every returned artifact.
3. SQLite relational identity validation remains ordered before the shared
   body-commitment check.
4. A missing commitment remains `legacy_unverified`; no expected value is
   inferred from mutable report content.
5. Commitment mismatch and other artifact validation failures map to the stable
   payload-free `work_report_artifact.read.corrupt` error.
6. Error output omits report text, identifiers, paths, and commitment values.
7. Serde remains structural and does not itself claim durable read assurance.
8. Write-path validation from the persistence-anchor phase remains unchanged.

## Boundary Assessment

The shared helper belongs at the durable store boundary rather than inside the
`Deserialize` implementation. Putting verification in serde would silently
affect fixtures, import/export, migration inventory, and later recovery paths
that require independent review. Explicit store enforcement makes the security
claim inspectable and keeps legacy classification possible.

Mapping all validation failures to one read-corruption error is also accepted.
It prevents an attacker from using the error surface to learn whether a report
body, metadata field, identity, or commitment changed.

## Residual Risks

- Existing inspect and metadata projection consumers must be checked to ensure
  they cannot bypass the enforced store path or overstate verified posture.
- Recovery, duplicate reconciliation, and replay still require separate
  end-to-end composition review.
- Legacy artifacts remain readable but unverified; no migration policy exists.
- A storage owner who can rewrite both the body and envelope commitment remains
  inside the current trust boundary.
- Independent security review is still required before release admission.

## Validation Reviewed

- focused local read substitution test;
- focused local list substitution test;
- focused legacy read/list posture test;
- explicit commitment validation regression coverage;
- formatter and warnings-denied Clippy;
- full `workflow-core` suite;
- documentation checks; and
- diff whitespace validation.

## Next Phase

Proceed only to bounded inspect and metadata projection enforcement. Do not
combine that work with recovery, duplicate reconciliation, replay, migration,
signing, witnesses, provider mutation, publication, or broader product claims.
