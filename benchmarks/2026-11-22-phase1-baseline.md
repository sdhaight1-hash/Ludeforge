# LF-B01 Phase 1 baseline — 2026-11-22

[Predeclared protocol](PHASE1-PROTOCOL.md), [source](../crates/ludeforge-core/examples/phase1_bench.rs), [actual CI job](https://github.com/sdhaight1-hash/Ludeforge/actions/runs/45068090726/job/137348669772), commit 978284c3afc5ed8da20d9369e17fe98f89c5f642. Locked release Rust 1.100.0. Hosted Linux, AMD EPYC 7763 reported, 4 visible vCPUs, 15,983 MiB RAM. Single thread. 1 warmup + 7 samples per operation, all correctness checks and deterministic round trip passed.

| Generated recruits | Normalized bytes | Validation median µs | Decode+validate median µs | Normalize median µs |
|---:|---:|---:|---:|---:|
| 100 | 37423 | 101 | 423 | 197 |
| 1000 | 356923 | 1156 | 4026 | 2141 |
| 10000 | 3560923 | 13705 | 58552 | 27162 |

[Raw samples and metadata](2026-11-22-phase1-baseline.json). Generated Lantern subset retains one role and trial; each recruit has one rank predicate. No disk I/O measured; decode includes strict JSON and semantic validation, normalize includes validation. No peak memory measurements, no Windows timing, no simulated gameplay or UI frame-time inference. Hosted runner variability and repetitive data limit comparisons. This is first observed baseline, not a guaranteed target or a claim about the user's hardware. No arbitrary optimization added. Retain full-scan oracle until real editing/analysis workloads justify incremental evaluation, compare with this protocol only under comparable conditions.

Both Windows and Linux passed all 25 tests, six CLI fixtures, Clippy and formatting at this revision. Linux release benchmark also passed. See recorded workflow.
