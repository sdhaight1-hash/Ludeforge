# ADR 0003 — Minimal Phase 1 JSON authority

Status: scoped implementation decision (2026-11-22). Issues #3, #6, #7, #12. Advanced persistence research remains open.

Choose a single UTF-8 JSON project file as authoritative authored data for the first kernel, following [v0.1](../KERNEL-CONTRACT-v0.1.md). The two current fixtures already use this shape; it supports direct inspection, review, Git diffs and portable inputs without a database service. The SQLite alternative may become worthwhile for indexed large edits or transactional runtime records, but there is no measured workload requiring it yet. No dual writable stores. This is not a completed persistence benchmark.

## Concrete contract

- Format v1, one flat project. Project ID identifies the envelope and is separate from the globally unique contained identifiers. IDs restricted to 1–64 ASCII characters. Labels nonblank Unicode, never identity.
- All numbers in raw v1 JSON must be signed safe integers, no exponent or fractional spelling. No null, duplicate keys, unknown typed fields, unsupported schema versions or nonempty rules. Booleans, integer, text, enum selection and typed object reference values are tagged. Optional property omission is distinct from zero. Integer field requires both bounds, enum nonempty options, reference exactly one target type. Empty option list is equivalent to absence on non-enum declarations. For other kinds, reject nonapplicable target/bounds/nonempty options.
- Relations support declared endpoint type restrictions, directed or symmetric mode, optional unique pair and self-edge policy. Omitted unique_pair defaults false; omitted allow_self defaults true. Symmetric endpoints require same declared type and ascending IDs; reversed records reject rather than silently correct. No universal acyclicity.
- Normalization sorts unordered types, fields, enum options, objects and relations by stable ID; value map keys sorted; field serialization order determined by v1 Rust structs; pretty UTF-8 JSON with trailing LF. It is NOT RFC 8785/JCS. No semantic hash guarantee yet. Explicit defaults may appear in normalized output.
- Input limit 16 MiB, depth 64, JSON value node limit 1 million, logical record/value count 100,000, diagnostics capped 256 plus truncation indication. Strict parsing occurs before semantic validation; shape errors currently point to root, semantic errors use JSON pointer paths. Validation reports a first failing layer and may cap findings.
- No destructive saves. Normalize to a newly created file (exclusive create, sync, best-effort cleanup of observed write failure); crash durability, directory fsync, concurrent edits, backups and atomic replacement not yet supported. File creation may leave a partial NEW file after a crash; original input remains unchanged. Never claim fault-tolerant writes.
- No runtime state, general constraints, expression evaluation, import, migration or module execution. Cross-object rank/pledge/aggregate rules remain explicit deferred requirements, not implemented checks. Reference index contains authored references only, not inferred rule dependencies.

Dependencies: serde and serde_json, commonly dual MIT/Apache-2.0; use minimal standard serialization, no copyleft source reuse. Exact resolved dependency/license inventory and Cargo.lock verification required when toolchain/CI can resolve dependencies; project license remains undecided (#5). No third-party code copied.

Exit gate: strict raw parser negative corpus, original fixtures and normalized round trips, typed reference diagnostics, no existing-file replacement, Rust fmt/clippy/tests on Windows and Linux. Record unrun checks. This decision authorizes narrow bootstrap, not closure of #3/#12 or all Phase 0.
