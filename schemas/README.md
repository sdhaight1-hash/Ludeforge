# Versioned project format

Current narrow authoring format v1: [ADR 0003](../design/adr/0003-phase1-json-authority.md), [Rust model](../crates/ludeforge-core/src/model.rs), [Lantern](../examples/lantern-academy/project.json), [Council](../examples/council-of-reeds/project.json). Rust strict decoder and semantic validation govern v1; no claim of a complete standalone JSON Schema yet.

Typed fields: bool, int (requires safe integer min/max), text, enum (nonempty stable options), ref (declared target type). Tagged values keyed by field ID. Omitted optional value allowed; null forbidden. Globally unique record/field/option IDs. Directed/symmetric typed relations; explicit optional unique pairs and self links. Rules must be empty.

Normalized format sorts unordered collections; exact normalized bytes within v1 are intended to be repeatable. Not JCS. Only new-file writes supported. Format migration, atomic replacement, rule IR, cross-object invariants, durable runtime and imports remain gated.
