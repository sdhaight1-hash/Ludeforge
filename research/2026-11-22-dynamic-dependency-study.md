# Dynamic dependency invalidation — 2026-11-22

Follow-up to [ontology composition study](2026-11-22-ontology-composition-study.md). Hypothesis: a conservative dynamic read index with reference-existence and membership dependencies can reproduce full validation after edits, including rebinding and insertion/deletion. A field-only cache cannot safely do this.

## Design

Static AST compilation validates types and collects possible read paths. At evaluation time record actual object IDs, field IDs, existence checks, and membership dependencies of aggregate queries. Record dependencies before attempting a possibly missing read, so restoring that ID invalidates errors. Rebinding a reference removes old subscription edges and replaces them with new ones. A candidate insert/delete marks its type membership dirty; aggregate scans subscribe to that membership and relevant predicate fields. A constraint on a removed object loses its cache entry. Initial implementation should prefer full validation; incremental mode requires parity tests over edits and safe fallback on unsupported queries, schema changes and type changes.

The experiment uses an original trial AST with field, reference read, comparison, and filtered sum operations. It maintains a reverse index and reevaluates only dirty entries; compares outcomes to a separately implemented fixture-specific oracle, not a filtered full-scan evaluation. It does not use static read-set compilation or commit invalid candidate states. Cross-object uniqueness requires a separate index or full scan; not tested here.

## Measured finite result

The function executed against a fixed initial data set (two roles, two recruits, one delegate, one proposal, one pledge). Initial parity and 16 successive edit comparisons passed. The exact sequence includes reference rebinding, old/new target edits, missing target restoration/deletion, aggregate membership insertion/removal, predicate-field update, new proposal, scope deletion and new recruit. See [source](prototypes/dependency-check.mjs) and [results](prototypes/dependency-check-results.json).

The experiment evaluates invalid intermediate candidates deliberately to verify diagnostics and repair, not to authorize commit. Counts of reevaluated entries are recorded; no elapsed time or performance advantage claimed. This is bounded evidence only, not proof of general incremental correctness. Standalone Node invocation has not been independently run.

## Production gates

Typed AST and access policy; correct dependency tracking under missing/optional values and short-circuit evaluation; uniqueness and aggregates including predicate changes; transactionally update caches only on successful revision; full scan parity for randomized edit sequences and changes of schema; limit graph traversals; filter results by authority when used in runtime. Never trust a stale derived cache in a committed ruleset.