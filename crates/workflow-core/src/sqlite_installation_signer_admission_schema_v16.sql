ALTER TABLE installation_issuance_claims ADD COLUMN signer_admission TEXT
  CHECK (signer_admission IS NULL OR length(signer_admission) BETWEEN 2 AND 1024);

ALTER TABLE installation_issuance_claims ADD COLUMN signer_admission_hash TEXT
  CHECK ((signer_admission IS NULL AND signer_admission_hash IS NULL)
      OR (signer_admission IS NOT NULL AND signer_admission_hash IS NOT NULL
          AND length(signer_admission_hash) = 64));
