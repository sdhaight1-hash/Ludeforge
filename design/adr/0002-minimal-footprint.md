# ADR 0002 — Minimal implementation footprint

Status: proposed baseline, 2026-11-22.

Context: the thesis describes many potential subsystems, but premature crate and UI scaffolding creates maintenance overhead.

Alternatives: build all studios first; build many independent crates; implement one headless vertical slice.

Decision: start with a single Rust kernel crate and CLI once implementation begins. Add new crates only at demonstrated dependency boundaries. Evaluate local persistence and desktop framework separately; no dependency installation or empty build manifests in this documentation-only bootstrap.

Consequences: smaller initial scope and easier verification; later splitting may be needed. Reconsider on measurable coupling or separate integration needs.
