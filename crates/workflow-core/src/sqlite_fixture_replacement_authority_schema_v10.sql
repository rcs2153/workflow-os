CREATE TABLE local_fixture_principal_registrations (
    principal_id TEXT PRIMARY KEY NOT NULL,
    verifier_payload TEXT NOT NULL,
    revision INTEGER NOT NULL CHECK (revision IN (1, 2))
);

CREATE TABLE local_fixture_replacement_grants (
    grant_id TEXT PRIMARY KEY NOT NULL,
    principal_id TEXT NOT NULL REFERENCES local_fixture_principal_registrations(principal_id),
    grant_payload TEXT NOT NULL,
    intent_commitment TEXT NOT NULL,
    revision INTEGER NOT NULL CHECK (revision IN (1, 2))
);
