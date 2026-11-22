# Phase 1 dependency inventory

Source: actual Cargo lock and metadata emitted by [Linux CI job 137227854433](https://github.com/sdhaight1-hash/Ludeforge/actions/runs/45028890266/job/137227854433) at commit a71af64f70bc2b5ffe113cde3ca0580acfc9c888. Captured Cargo.lock without manually inventing checksums or versions. Build resolves the locked dependency graph. Metadata license expressions are screening evidence, not a legal opinion or complete license-text review. Owner's project license decision remains open (#5); no external source copied.

| Package | Locked version | Manifest license expression |
|---|---|---|
| itoa | 1.0.19 | MIT OR Apache-2.0 |
| memchr | 2.8.3 | Unlicense OR MIT |
| proc-macro2 | 1.0.107 | MIT OR Apache-2.0 |
| quote | 1.0.47 | MIT OR Apache-2.0 |
| serde | 1.0.229 | MIT OR Apache-2.0 |
| serde_core | 1.0.229 | MIT OR Apache-2.0 |
| serde_derive | 1.0.229 | MIT OR Apache-2.0 |
| serde_json | 1.0.154 | MIT OR Apache-2.0 |
| syn | 3.0.10 | MIT OR Apache-2.0 |
| unicode-ident | 1.0.27 | (MIT OR Apache-2.0) AND Unicode-3.0 |
| zmij | 1.0.23 | MIT |

Two direct crate dependencies: serde and serde_json; other registry packages are transitive. No UI, network client, AI runtime or database dependency. Preserve exact lock, inspect license texts and current advisories before redistribution; update only through reviewed dependency changes. CI reports resolved metadata. This is not a vulnerability scan.
