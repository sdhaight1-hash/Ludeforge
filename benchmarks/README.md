# Benchmarks

Store reproducible workload definitions and measured results. Specify fixture, revision, seed, execution mode, hardware, trial count, metrics and uncertainty. The thesis target of 100,000 lightweight encounters is provisional. No measured baseline yet.

## Phase 1 baseline

[Predeclared protocol LF-B01 and future LF-B02](PHASE1-PROTOCOL.md). Reproduce LF-B01: `cargo run --locked --release -p ludeforge-core --example phase1_bench`. This checks six-fixture kernel validation capacity on a generated Lantern subset; do not mistake it for a playable encounter benchmark.

- [First observed LF-B01 release baseline](2026-11-22-phase1-baseline.md) and [raw samples](2026-11-22-phase1-baseline.json).
