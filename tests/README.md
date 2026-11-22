# Validation

Executable Rust conformance tests live alongside their owners:
- [Core](../crates/ludeforge-core/tests/conformance.rs): original divergent fixtures, raw strict decoding, data mutations, relations, normalization and references.
- [CLI](../apps/cli/tests/cli.rs): input validation, exclusive new file output and exit codes.

Run cargo test --workspace. See CI for actual results. Research-only JS prototypes remain in research/prototypes; they are not integrated Rust tests.
