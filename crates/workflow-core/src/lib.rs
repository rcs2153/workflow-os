#![deny(unsafe_code)]
#![doc = "Canonical Rust core crate for Workflow OS."]
#![doc = ""]
#![doc = "This crate intentionally contains no workflow runtime logic yet. It is the"]
#![doc = "reserved home for the local-first kernel described in the project charter."]

/// Human-readable name for the canonical Rust core crate.
pub const CRATE_NAME: &str = "workflow-core";

/// Current pre-release maturity marker for the core crate.
pub const MATURITY: &str = "foundation";
