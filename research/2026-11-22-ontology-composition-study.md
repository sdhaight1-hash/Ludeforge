# Ontology composition, constraints and schema evolution — 2026-11-22

Baseline: kernel contract v0.1 and prior original fixtures. Method: compare primary public specifications and the earlier ConWorld review; execute an original finite JavaScript model. This study proposes a v0.2 contract delta; not an implemented Rust feature.

## Research questions

1. Can designers combine reusable types without hidden inheritance order or overriding rules?
2. Can constraints refer to other objects and catch changes that invalidate dependent facts?
3. Do relationship kinds need different topology/cardinality policies?
4. Can imports and schema evolution avoid silently changing historical meaning?

## Primary references and transfer

| Source | Observed mechanism | Ludeforge decision candidate |
|---|---|---|
| [CUE specification, unification](https://cuelang.org/docs/reference/spec/) | Unification is commutative, associative and idempotent; inconsistent values meet at bottom. | Intersect compatible declared integer domains; reject empty domains and conflicting kinds. No arbitrary universal lattice engine. |
| [SHACL W3C specification](https://www.w3.org/TR/shacl/) | Shape validation supports graph paths, qualified cardinality and diagnostic results. | Typed pure constraints and explicit graph policy, not RDF inference or arbitrary SPARQL. Keep shape checking separate from rule execution. |
| [Datomic identity and uniqueness](https://docs.datomic.com/schema/identity.html), [schema reference](https://docs.datomic.com/schema/schema-reference.html) | Stable identities, separate unique-value constraints, composite tuples and cardinality. | Stable opaque IDs; composite uniqueness for Pledge(delegate, proposal), no implicit upsert or merging identities. |
| [Flecs relationships](https://www.flecs.dev/flecs/Relationships.html), [component traits](https://www.flecs.dev/flecs/ComponentTraits.html) | Relationship pairs, inheritance/sharing, and exclusive relationship replacement. | Relation-specific semantics, but reject cardinality violations rather than silently replacing an edge. Component sharing must not imply game-specific inheritance. |
| [Protocol Buffers evolution guide](https://protobuf.dev/programming-guides/editions/) | Stable field numbers; deleted identifiers reserved against reuse. | Retire field IDs permanently within a lineage; rename labels not IDs. Breaking changes require an explicit migration and new revision. |
| [ConWorld governed semantic extensibility](https://github.com/sdhaight1-hash/ConWorld-Tools/blob/main/Design-Docs/Architectural-Evolutions/universal-worldbuilding-ontology-and-governed-semantic-extensibility.md) | Prior reviewed design: governed versioned descriptors, stable identity, reject unrestricted EAV. | Avoid a universal schema scripting layer. Pin descriptors and modules; validate before committing changes. |

Sources consulted 2026-11-22. This is conceptual transfer, not code reuse or dependency selection.

## 1. A minimal compositional ontology

Option A: nominal type inheritance, overrides ordered by linearization. Convenient for class-like hierarchies, but conflicts, diamonds and substitutability become hidden mechanics. Option B: arbitrary open-world graph inference. Expressive but difficult for finite authored constraints or reproducible runtime. Option C: explicit reusable field groups composed by stable field identity with no implicit replacement. Prefer C only after Phase 1 single-type proof.

A field group is reusable schema metadata, not a new runtime entity category. Fields shared intentionally by identity can intersect compatible constraints; independent fields sharing a label remain distinct. Incompatible primitive kind, reference target, unit or enum semantics are errors, not last-writer-wins. For bounded integers, effective lower bound=max(lower bounds), upper=min(upper bounds); required if any contributor requires it. Reject an empty domain. Do not use this limited merge policy for arbitrary executable predicates or defaults. Keep original source contributors and expose conflict diagnostics.

## 2. Structural constraints versus mechanical predicates

Distinguish local type validation (required/kind/range), referential validation (target exists/type), compound uniqueness, relation graph constraints, pure cross-object predicates, and action-specific guards. All are needed eventually; not all belong in the first kernel slice. Lantern rank<=role.max_rank and Council Pledge.strength<=delegate.influence expose the need for cross-reference reads. Pledge(delegate, proposal) must be unique; no hidden creation or merger. Council support is derived from Pledge records; decorative supports edges are a projection only, not authoritative independent data.

Compile expressions with stable field IDs and declared types. Read paths must be bounded; follow typed references only. Missing operand is an explicit error unless a separately typed presence/branch operator handles it. Never silently interpret missing as zero. Evaluate against a consistent candidate snapshot after structural validation. A field edit can invalidate an object elsewhere, so maintain reverse dependencies by specific object+field (for actual reads) plus static possible dependencies. Invalidate reference rebinding and query membership, not just existing field values. Cross-object uniqueness/aggregate changes require index invalidation or full validation. Full scan remains the reference oracle; optimize only after differential parity.

## 3. Relationship semantics

A relationship definition declares endpoint types, direction, self-edge policy, duplicate policy, cardinality and optionally acyclicity. Directed A->B and B->A are different; symmetric A-B is one canonical pair. For symmetric max-degree, count both endpoints, not merely stored source; define self-edge behavior separately. Opposite-direction directed edges are legitimate; ancestry cycles can be refused without banning friendships or runtime feedback. Do not allow acyclicity on symmetric undirected relation via the directed-cycle algorithm; specify undirected forest semantics independently if ever needed. Reified relation objects (e.g. Pledge with strength, a temporal employment relation) use typed references and composite constraints; don't duplicate into a separately editable graph edge.

## 4. Module composition

Compare copy/fork (simple but loses shared revision semantics), floating imports (convenient but unreproducible), exact pinned imports (preferred future model). Package ID and immutable revision/digest; explicit dependency DAG; deterministic resolution; namespaces and stable local ID->project ID mapping; conflict diagnostics; no floating updates in a published ruleset. No network resolution at MVP. First usable option: an explicit project-local import preview producing a flattened candidate file, detect reference collisions, validate then commit. No transitive lock resolver needed until reusable module cases demand it. Reject multiple revisions of one package unless a later isolated namespace contract proves side-by-side use. Distinguish content license from package code license.

## 5. Schema migration

Separate project file format version, authored mechanical revision and runtime revision. Preserve old immutable snapshots and their rule meaning. Label renames preserve stable references. Adding optional fields can be compatible if no behavior depends on presence; adding required fields, narrowing domains, changing units/type, changing meaning or deleting referenced fields require explicit migration. Deprecated ID is never reassigned. Candidate-copy migrate, validate all references and constraints, show diffs and affected records, commit once or leave old input unchanged. Do not infer a conversion or silently clamp values. Runtime games remain pinned to old mechanical revision unless independently migrated; format migration is not gameplay migration.

## 6. Executed feasibility model

[Source](prototypes/ontology-check.mjs) and [results](prototypes/ontology-check-results.json). Executed original JavaScript function in research environment; standalone Node launcher not separately invoked. 32 named assertions passed and 24 bounded edit comparisons agreed. Assertions cover order-independent intersection, rejection of conflicting domains, cross-object constraints, composite uniqueness, stable diagnostics, relation-specific policy, pinned module identity and atomic candidate migrations. The reverse index is manually specified and the parity check uses a full-scan oracle on existing fixed reference paths; no claim of efficient incremental implementation. Does not cover cycles in composition graphs, complex predicates, real package resolution, parser robustness, concurrent transactions, persistent history or production migration durability.

## 7. Next experiments

Extend model to predicate type checking, read-set extraction from AST, reference rebinding, insert/delete changing aggregates and composite keys, dynamic query dependencies, unit/decimal representation and multiple overlapping constraints. Validate all six original fixtures as they evolve. Before implementing module reuse, test stable ID namespace mapping and dependency graph cycles. Establish storage failure/recovery experiments separately rather than interpreting a copy-on-write toy model as disk durability.

See [v0.2 ontology contract delta](../design/ONTOLOGY-CONTRACT-v0.2.md).