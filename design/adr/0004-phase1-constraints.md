# ADR 0004 — Phase 1 typed predicates and relation policies

Status: scoped implementation decision; verified in cross-platform [run 45067916962](https://github.com/sdhaight1-hash/Ludeforge/actions/runs/45067916962). Issues #3, #7, #10, #16. Based on [ontology v0.2](../ONTOLOGY-CONTRACT-v0.2.md), [mathematics v0.3](../MATHEMATICS-CONTRACT-v0.3.md), [source study](../../research/2026-11-22-kernel-source-study.md) and [dependency study](../../research/2026-11-22-dynamic-dependency-study.md).

Use one tagged, bounded AST shared by future rule work; for now it only validates authored object invariants. Full scan is the correct reference semantics. No cache coherence assumption or generalized inference engine. Existing pre-release JSON v1 files remain accepted; new optional fields are not silently understood by older readers. No release or migration from a released version implied.

## Implemented intended grammar

| op | operands | result |
|---|---|---|
| int / bool | safe integer / boolean literal | respective scalar |
| read | field on scope (int/bool) | declared scalar |
| ref_read | typed ref field and target field (int/bool) | declared scalar |
| present | any declared scope field | bool |
| not | bool | bool |
| and / or | bool, bool; short-circuit | bool |
| eq | two identical scalar kinds | bool |
| le / lt | int, int | bool |
| add / sub / mul | int, int checked against machine and safe range | int |

Each predicate must statically produce bool; both branches checked even if never executed. Missing optional reads are evaluation errors unless explicitly protected by presence/short-circuit. No implicit zero or coercion. No division until rounding semantics selected. Reference reads one hop, only scalar targets; no arbitrary path traversal, collection, enum/ref equality or derived field support yet. Expression depth 32, 4096 nodes per predicate, shared 1 million compile/evaluate/key steps per project; existing raw parsing limits also apply. Unknown operations/fields reject. Static reads include both branch operands and both field IDs in reference reads.

Constraint IDs part of global identity registry. Predicate scope is one authored type; validate all instances after ordinary schema/reference validation, including compile of constraints whose scope is empty. Distinguish CONSTRAINT_SCHEMA, CONSTRAINT_FALSE, CONSTRAINT_EVALUATION, CONSTRAINT_LIMIT. Unique keys (1–32 distinct required field IDs of scope) use exact tagged value tuple equality, missing key invalid, no merge/upsert. Each row checks one tuple. Report conflicting ID. Normalize constraints by ID; ordered operands and key field list preserved. Static dependency index exposes scope and possible field reads; not instance-level incremental dependency graph.

Directed relation types may specify max_in, max_out, acyclic. Symmetric relation types use max_degree; no directed acyclicity or in/out limits permitted. Count each edge (parallel edges count separately when allowed); symmetric self-edge counts once. Zero cardinality allowed. Iterative Kahn cycle detection scoped to each opted-in directed type, not global graph. Structural endpoint validation precedes topology. No implicit replacement.

Constraints are always checked in candidate data before normalize or reference index. The kernel does NOT perform actions, apply effects, resolve dice, schedule time or persist runtime state. Rule compiler and multi-request transaction contract remain Phase 2 work. No new dependencies.

## Evidence targets

Lantern rank vs role max; Council pledge strength vs delegate and unique tuple; independent fixture negative mutations; static type errors even empty scope, short-circuit, signed arithmetic and overflow, depth limits, topology policy, rebind, stable references and normalized bytes. Test via cross-platform locked CI. Future derived properties and aggregate predicates must have their own contract, read-set semantics and test evidence.
