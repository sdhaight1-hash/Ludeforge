# ADR 0001 — System-agnostic canonical core

Status: proposed baseline, 2026-11-22.

Context: the thesis requires divergent RPGs, multiple editors and reproducible execution.

Alternatives: hard-coded RPG concepts; script-only engine; configurable typed model plus rule representation.

Decision: begin with a typed designer-defined model and stable IDs. No mandated RPG conventions. Keep authored definitions, runtime instances, and generated outputs distinct. Visual and textual rules must share one validated intermediate representation.

Consequences: stronger consistency and inspectable dependencies, but schema evolution, typing and author usability require research. Informal prose and human decisions remain supported. Revisit if divergent fixtures cannot be represented without kernel-specific exceptions.
