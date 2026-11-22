# Kernel feasibility follow-up — 2026-11-22

Inspected revisions: initial baseline b8522a44ab4d1791aedf81a23c15b1c2f6dff9a1; source research 0945bce8e22c1d7299a263b8f49004e75e6b6fcf; corrected models/fixture study cf7486323c94ff97a4534a9500bea223165a7255.

## Evidence

- Four external repositories examined at selected source paths (Quest Bound, LoreKit, Mechflow, Icepool); repository license metadata and blob IDs recorded. Primary standards reviewed: CEL, SHACL, JSON Schema, RFC 8259/8785, SQLite and Ludii.
- Candidate versioned authoring and runtime contract written. No accepted license or new external dependency.
- Finite abstract protocol: 22 states, 154 transitions, no specified invariant violation in corrected baseline; 5 mutation counterexamples found. Revision zero checkpoint overwrite discovered during review and fixed.
- Probability: 182 exact outcome-count bins and 200 tail threshold comparisons across six dice configurations agreed with independent enumeration.
- Two original fixture files: schema prototype accepts both, verifies rename, normalization and round trip, rejects 13 seeded malformed structured cases.
- Prototypes ran as JavaScript functions with exact authored fixture inputs. Standalone Node entry points were not executed. No Rust engine, CI run, platform durability test, or performance benchmark exists.

## Remaining gates

1. Finalize schema structure, cross-field constraints and type-specific relationship rules, diagnostic codes and parser limits. Eliminate duplicate authoritative representations; supports edges in Council are test-only and must not duplicate Pledge strength.
2. Verify strict raw JSON parsing (duplicate names), canonical serialization including Unicode, and persistence through failure injection. Compare single-file versus SQLite with an explicit workload before stronger conclusions.
3. Extend abstract protocol to multiple requests, conflicting writes, cancellation, event replay, and recovery; distinguish bounded evidence from full correctness.
4. Resolve licensing with owner and inspect exact upstream license texts before any dependency reuse.
5. Refine evaluation plan with exact test corpus and workload, then implement the Rust headless vertical slice (#6/#7).

Issues remain open because research evidence is preliminary, not acceptance of production behavior. See [register](../research/EVIDENCE-REGISTER.md).
