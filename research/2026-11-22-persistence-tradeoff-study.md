# Phase 0–1 persistence trade-off probe — 2026-11-22

Addresses R03 #12 and #3. Primary storage semantics from [SQLite atomic commit](https://www.sqlite.org/atomiccommit.html) previously reviewed in [kernel source study](2026-11-22-kernel-source-study.md); no new dependency or code copied. Compare chosen single portable authored file vs indexed SQLite transaction. This is an isolated Python storage experiment, not equivalent mechanical validation, a controlled production benchmark or a crash-recovery proof.

Executed [storage-probe.py](prototypes/storage-probe.py) using Python 3.13.5 and SQLite 3.46.1 in Linux container, temporary directory. Generate 100/1000/10000 simple rows. Each edit (1 warm-up + 5 measurements per size) either reads whole JSON, updates one ID and exclusive-creates a new synced file or updates one indexed SQLite row and commits with DELETE journal and FULL synchronous. DB creation outside timing. Compare every restored row outside timers and confirm original JSON bytes unchanged. One simulated exception inside a SQLite transaction verifies rollback; separately write a truncated candidate and verify it cannot parse, original still intact. 18 equivalent edits, 3 successful transaction rollbacks and 3 partial-new-file rejection cases. No physical crash/power-loss, concurrency, stale-write protection, journal-mode sweep, Windows storage timings or security proof. Hardware/filer not controlled; timings only illustrate this workload and cannot be extrapolated.

| Rows | JSON median (microseconds) | SQLite median (microseconds) |
|---:|---:|---:|
| 100 | 1116 | 1495 |
| 1000 | 3769 | 1384 |
| 10000 | 28787 | 1651 |

All raw samples: [results](prototypes/storage-probe-results.json). Both APIs lack actual Ludeforge constraints/strict parser in this comparison. One new-file write and one in-place DB transaction have different storage and durability semantics. Do not interpret timing ratio as a fair product benchmark. SQLite's indexed single-row update avoids full file rewrite; that is its intended benefit, but doesn't establish actual authoring needs or total cost.

Decision: retain JSON as current authored source as specified in ADR 0003. SQLite may become a derived index or separate durable runtime store; reconsider authority only after measured actual editor workload and transactional recovery requirements. Existing Rust CI has verified exclusive-create/overwrite rejection and strict parsing on Windows and Linux. Destructive saves remain explicitly unsupported. Use immutable originals and validate candidate export; no power-failure durability claim. Update #12 if actual in-place edits or database authority are proposed. No SQLite dependency added.