# Mechanical kernel research decisions (proposed)

Derived from [cross-repository study](../research/2026-11-22-cross-repository-study.md), [external prior art](../research/2026-11-22-external-prior-art.md) and [original fixtures](../examples/REFERENCE-FIXTURES.md). No implementation or finalized architecture decision is claimed.

## 1. Scope: govern extensibility without universal inference

A Type has stable ID, label, property definitions and explicit constraints; EntityDefinition has stable ID and type reference; runtime Instance has own ID and version-bound definition reference (or permitted instance type). Relationships have declared endpoint restrictions and cardinality. No fixed concepts like character, weapon or HP. A property ID survives rename; type change requires compatible migration. Missing, explicit unknown (if permitted), and zero are distinct. Start with Boolean, bounded integer, text, enum and typed reference. Evaluate lists, units and exact decimal/rational values when fixtures require them; never silently coerce strings or round arithmetic.

Schema fields should not become independent universally addressable facts unless actual requirements demand reification. MVP avoids unrestricted EAV, inheritance diamonds, arbitrary predicates with hidden side effects, and recursive rules. Descriptive relationships are not execution instructions.

## 2. Keep four graphs distinct

- Semantic relationship graph: designer-declared edges, may be cyclic.
- Derived-value dependency DAG: dependencies extracted from expressions; reject cycles unless an explicit future solver is introduced.
- Rule interaction graph: reads, writes, triggers and effect edges; feedback may be intentional. Classify potential shared writes, cycles, conflicting effects and missing ordering as findings.
- View graph: layout, grouping, pan and zoom, not mechanical semantics.

Queries require bounded depth/visits and stable ordering; partial results signal truncation. Change-impact traversals use typed reverse dependencies; dependency source records identify rule, expression and property. UI coordinates and labels do not alter compiled semantic identity; semantic version changes invalidate run receipts.

## 3. One resolution authority per decision

Model ActionRequest(id, expected_revision, rule_id, arguments, actor_or_authority), then evaluate:
1. resolve stable IDs and permissions; validate argument types and guards;
2. execute bounded typed operations into a tentative effect transaction;
3. obtain explicit stochastic inputs or suspend at a human decision boundary;
4. validate resulting constraints, complete causal trace, then atomically commit state and event receipt;
5. reject without partial mutation on error, stale revision or exhausted execution budget.

A suspended action retains immutable proposal data, rule version, expected state revision, authorized decision source and allowed result schema. Do not commit costs yet. Resume checks revision and single-use decision ID; duplicate request handling must be idempotent, while changed payload under same ID is an error. Rejected outcomes must not grant success or consume runtime resources unless explicitly modeled as a valid completed failure. Distinguish rejected action from failed attempted action.

Re-execution uses recorded random inputs and human choices, not live providers. Event application reconstructs state without recomputing historical rolls. Capture deterministic operation ordering, numeric overflow errors, initial state hash, model version and runtime version. MVP can use an in-memory candidate copy and ordered event vector; no full event-store architecture until fixture proof.

## 4. Persistence alternatives

A single canonical versioned structured JSON project file is the candidate for Phase 1 because it supports review, fixtures and diffs. Normalize ordering and reject duplicate IDs, unknown versions and malformed references before modifying an open project. File replacement requires a same-directory temporary file, appropriate flush/sync, backups and failure injection tests; document platform-specific limits. A hash must use specified canonical bytes or semantic canonicalization, never unspecified serializer ordering.

Alternative SQLite authority: transactionally enforce updates and enable large indexed queries, but needs export/diff/migration semantics and backup strategy. Do not use dual independent stores. SQLite may later act as rebuildable search index or separate runtime store with explicit contract. Do not introduce CRDTs, distributed services, graph DB, GPU or plugin ABI for MVP. Evaluate thresholds using fixtures, not assumptions.

## 5. Probability and simulation

Sampler and exact evaluator consume the same typed finite distribution description where supported. External choice and correlated draws need explicit state. Uniform integer distribution, sums and threshold tests first; compare to enumerated outcomes and Icepool. Decks without replacement, reroll/explosion and larger dice pools follow fixture evidence. Limit finite support, recursion, steps, memory and sampling; report approximate/truncated results explicitly. Agent policies see authorized observations and choose legal actions; no arbitrary writes. Record independent seed derivation, sample sizes and failed trials. Report Monte Carlo uncertainty and bounded counterexamples, not proofs about human play.

## 6. Interface and authoring

The CLI is the first user-visible proof of model/validation. Desktop authoring later uses typed forms and graph operations through shared project services. Validate changed structures before expensive evaluation, schedule cancellable tasks bound to exact project revision, discard stale results. Do not require a visual graph for simple numeric expressions. Publishing references bind stable IDs to formatted views; narrative prose, truth assertions and artwork remain independently reviewable.

## 7. Acceptance evidence

- Original empty ontology has no required RPG concepts.
- Rename a type/property without breaking stable references; malformed type/ID/value rejects.
- Resource transfer and event emission commit together, or neither does.
- Diceless adjudication suspends/resumes without randomness or mandatory GM.
- Confirmed progress cannot be duplicated or erased by shorter capacity.
- Derived cycle rejected; narrative and intentional temporal feedback allowed.
- Unsupported operation is explicit, not guessed or routed to AI.
- Every trial retains revision, inputs, decisions and outcome trace.
- No external repository code and no mandatory AI, network, GUI in first kernel.

These are input to issues #2–4 and #6–8, not additional implementation commitments. Freeze exact model before writing production code; keep unresolved questions visible.