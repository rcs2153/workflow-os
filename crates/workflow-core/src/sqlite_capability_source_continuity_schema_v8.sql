CREATE TABLE capability_source_continuity (
    continuity_key TEXT PRIMARY KEY,
    source_registration_commitment TEXT NOT NULL,
    query_commitment TEXT NOT NULL,
    generation INTEGER CHECK (generation IS NULL OR generation > 0),
    watermark TEXT NOT NULL
);
