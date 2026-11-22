# Ontology contract v0.2 — proposed delta

Applies to [kernel contract v0.1](KERNEL-CONTRACT-v0.1.md); v0.1 remains authoritative candidate for unchanged parts. Status: research proposal, not accepted implementation. [Evidence](../research/2026-11-22-ontology-composition-study.md).

## Scope gates

Phase 1 minimum: authored type, declared field, stable ID, value, typed reference, relation and local schema constraints; two different valid projects plus negative fixture corpus and strict parse/save-to-new-file. No composition/module machinery or general predicate runtime required for initial bootstrap. Next Phase 1/2 slice: compound uniqueness, cross-field/cross-reference constraints, relation policy; Phase 2 action guards and typed expressions; later module reuse and migration after evidence. Keep compatibility with flat authored v1 examples; new fields or format additions require explicit spec changes, not silently accepted unknown keys.

## Identity and ownership

Type declares field instances directly; a reusable group is a future source of field definitions, flattened for validation. Each global ID denotes one schema object or record; two appearances of the same shared field ID refer to the same declaration, not duplicate definitions. Different labels may coincide; resolution never uses a label. Field group identity and effective field identity must remain explicit to avoid contradicting global uniqueness. No implicit inheritance, no last-writer-wins. Store origin references separately from effective normalized schema.

## Validation layers

1. Structural format: strict parsing, known version, limits, correct shapes.
2. Local semantic: stable IDs, fields, value kind/range, requiredness.
3. Referential: typed endpoints and object references.
4. Global constraints: per-type unique key tuple, declared relationship cardinality and topology, bounded pure predicates.
5. Runtime constraints: validate writes on candidate state; action guards concern the proposed action, not every existing object. A raw state cannot enter runtime if its declared invariant is false.

Diagnostic draft: FORMAT_DUPLICATE_KEY; FORMAT_VERSION; ID_DUPLICATE; FIELD_REQUIRED; VALUE_KIND; VALUE_RANGE; REF_MISSING; REF_TYPE; REL_DUPLICATE; REL_CARDINALITY; REL_SELF; REL_CYCLE; CONSTRAINT_FALSE; CONSTRAINT_EVALUATION; UNIQUE_KEY; COMPOSE_CONFLICT; IMPORT_COLLISION; MIGRATION_INVALID. Include stable code, source/record/field, constraint ID, related IDs, deterministic path/order. A predicate evaluation error differs from false. Missing optional value is not numerical zero. Unique keys initially require all participating fields present and key-compatible; no implicit null semantics.

## Example pure constraint syntax (illustrative)

~~~json
{
  "id": "rank_limit",
  "scope_type": "recruit",
  "expr": {
    "op": "le",
    "left": {"op":"read","field":"rank"},
    "right": {"op":"ref_read","field":"role_ref","target_field":"max_rank"}
  }
}
~~~

Resolve each read statically against scope type and reference target type; both operands integer, result Boolean. Reject malformed or unknown op, invalid binding, incompatible operand type or exceeding evaluation budget. No mutation, random inputs, dynamic name lookup or AI. Re-evaluate on candidate snapshot. For first predicate implementation prefer full validation of the affected project; add reverse dependency index with conservative invalidation and differential reference tests. Changes to referent identity, membership, uniqueness or aggregate bounds require wider invalidation. Test absent values, dangling refs and multiple failures.

## Relationship policy

Use stable relation types with explicit directed/symmetric flag, endpoint restrictions, self-edge, unique pair and optional per-endpoint max cardinality. Symmetric endpoints must be same declared type in current candidate. Normalize unordered pair identity; reject duplicate pair rather than merge. Reject constraint violations without silent replacement. Apply directed acyclicity only to declared directed subsets; no implicit graph-wide acyclicity. For executable relation properties or temporal facts, use typed relation-as-entity pattern. Derived display edges are rebuildable, not independent truth. Add richer relation schemas only after actual use cases.

## Migration and import preview (deferred)

Exact pinned package revision and source provenance. Resolve user-approved import into candidate project using explicit mapping; no automatic update, remote code, hidden ID replacement or unresolved references. Identifiers retired in a lineage never reused. Migration document names source revision, target schema, operation list, mappings and expected diagnostic/report. Validate full candidate with new schema, preserve original snapshot; commit only after reviewed success. Keep mechanical/runtime revision pin across migration until explicit separate runtime conversion. Future persistence contract defines atomic filesystem transaction and stale revision handling; no disk safety implied by research model.

## Acceptance before claiming advanced ontology support

- Generic constraints express Lantern and Council without code changes for new game terms.
- Rebinding a reference, deleting target, changing membership detects the same violations as full reference evaluation.
- Ordered and reversed relation declarations give identical semantic verdicts; cardinality and cycle policies apply only to specified kinds.
- Module import collision/missing pinned revision yields no partial writes.
- Rename preserves references; incompatible change rejected without losing old revision; retired ID cannot be reused.
- All limits explicit, diagnostics stable; no new fixed RPG categories.
