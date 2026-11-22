# Phase 0–1 kernel implementation audit — 2026-11-22

Initial baseline 0e44ad55c97c4e7d60b3776c5257988e346dc96f. First implementation 4e34bfd41e078bc1a0394fbb541958f7e3ea5aa1, conformance suite 11320227708eb7ccbb8e8341252ee8998d8487d6, exact formatter changes aa7e914aa1b908b8f6def2528e736d4a2dc14f24, CI dependency inventory a71af64f70bc2b5ffe113cde3ca0580acfc9c888.

## Delivered

Rust workspace with one core library and CLI. Strict 16 MiB byte-limited JSON decoder with depth/node guard, duplicate-name rejection before mapping (including escaped alias), rejection of null, noninteger or out-of-range numbers, unsupported versions and unknown fields. Flat typed authored definitions, field kinds, explicit references, local bounds and enum checks, directed/symmetric relationship policy, deterministic diagnostics, normalized output and authored reference index. CLI validate/references/normalize; exclusive-create for new normalized output. No UI, rule runtime or destructive save.

## Actual validation evidence

- [First run](https://github.com/sdhaight1-hash/Ludeforge/actions/runs/45028765474): format failed; later checks skipped.
- [Second run](https://github.com/sdhaight1-hash/Ludeforge/actions/runs/45028818638): Windows and Linux Clippy, Rust tests and two fixture CLI validations passed; formatting failed. Applied exact formatter diff from Linux logs.
- [Formatted Linux run](https://github.com/sdhaight1-hash/Ludeforge/actions/runs/45028861295/job/137227771124): all lint, tests, fixture validation and format steps passed. Linux log for dependency inventory run [137227854433](https://github.com/sdhaight1-hash/Ludeforge/actions/runs/45028890266/job/137227854433) also shows success for lint, test, both fixtures, format and metadata.
- Rust test suite: 10 core conformance tests + 2 CLI integration tests, plus multiple input mutations within tests. Actual Linux log reports all 12 passing. This is not an assertion that all semantics or security are proven.
- Cargo-generated lock and manifest license metadata captured from CI; see [inventory](../research/DEPENDENCY-INVENTORY.md). Final locked CI must be checked separately. Local execution environment lacks Rust and cannot access rustup; checks ran on GitHub-hosted agents, not here.

## Boundaries and next work

Phase 1 is in development. Scope is ADR 0003. #6 bootstrap is implemented, verification of final locked revision pending; #7 local ontology validation implemented, cross-object/composite constraints and additional fixture coverage remain. #3/#12 have narrow JSON authority, not full persistence failure/recovery or migration evidence. #10 typed expressions and #13 multi-request transaction research remain necessary before #8. Do not conflate previous JS prototypes with integrated Rust checks. No in-place writes, crash safety, general constraint or replay implementation, GUI, performance claim or project license decision. Continue with production typed predicate/constraint slice and explicit runtime contract after verifying this baseline.
