# Phase 2 executable rule entry contract — Phase 0 specification

Status: accepted as scoped design target, no execution implementation. Issues #4, #8, #10, #13, #16, #22. Preserves [ADR 0003](adr/0003-phase1-json-authority.md), [ADR 0004](adr/0004-phase1-constraints.md) and [multi-request evidence](../research/2026-11-22-multi-request-replay-study.md). This is the smallest runnable action slice required by Lantern and Council. No broad scripting or specialized RPG subsystem.

## Boundary and version

Freeze existing authoring format version 1 and its exact normalized semantics. New executable project envelope version 2 is the proposed Phase 2 implementation target; migrate explicitly to a newly written file, never implicitly overwrite v1. Preserve all stable IDs on supported conversion. The current v1 validator must still reject v2 until implemented. Version 2 reuses v1 types, objects, relations and constraints; adds typed rules. Keep two original v1 examples and create separate executable variants; no parallel independent authority. Loaded executable ruleset is immutable for the run. Bind to exact canonical versioned model snapshot and engine/IR version, not an unverified string named revision. A durable cryptographic content hash is a separate future mechanical identity contract. Do not load scripts, network, plugins or arbitrary extension opcodes in this baseline.

## State and authority

Authored objects are definitions/initial values. A test run seeds one runtime instance per authored object; runtime IDs equal seed IDs only within that run's separate namespace. One-to-one binding explicit; no spawn/delete/relationship mutation in the minimum slice. Runtime state contains mutable value maps keyed by instance ID, revision and event journal. Reads and writes target runtime copies; source JSON and rule definitions are never changed. References resolve to seeded instance IDs under the pinned model. Model fields, type, identity, relation definitions and authored labels are not writable. Existing local/global constraints validate candidate runtime value maps using same core validator.

Trusted invocation context is supplied by host: run identity, authorized actor/principal and allowed decision sources. A rule parameter containing a reference never grants permission by itself. No mandatory player, GM or turn. Define rule invocation permission as explicit scope supplied by caller; no security guarantee from a raw actor string. Hosted/multi-user authentication is out of scope.

## Expression and statement IR

Reuse pure expression nodes and type-checking in ADR 0004. Extend context rather than maintaining a second formula grammar. Each rule has stable ID, ordered parameter list with stable IDs and declared nominal types; guard expressions return Boolean; ordered statements with stable instruction IDs, nested branch arms; all symbols resolved at compile time. Scalar Int and Bool as before; typed Ref(target type), typed Enum(declaring field ID) for Council status. Reference and enum equality only within same nominal type. Runtime argument binding must be type-correct and point to actual instances. Literal enum binds option ID to declaring field; no implicit text conversion.

New pure nodes: arg(parameter ID), local(local ID), field_of(ref expression, field ID), enum_literal(field ID, option ID), ref_eq of same declared target type (or eq after nominal type checking). A field_of receiver type is resolved statically. Existing read/ref_read/present remain scoped in predicates only; bind scope when compiling rule expressions or reject unbound reads. A local must be definitely initialized on every path before read; simplest first subset: immutable locals scoped to statement sequence or branch, no branch-defined local escapes. Validate all paths, even unreachable branch.

Council prerequisite needs a single bounded filtered integer sum, not general-purpose queries: sum_where_ref(row_type, match_field, target_expr, value_field). match_field must be a ref declared on row_type whose target matches target_expr; value_field is integer. Evaluate target exactly once then scan candidate rows of row_type ordered by ID. Missing required operand is error; sum empty=0; checked addition in order (including intermediate safe domain); spend one or more steps per scanned row. Static references include row_type, match/value fields and target expression; membership invalidation belongs to later dependency optimization. Never assume support is an independently editable field. No derived fields/materialized formula cache in minimum action slice.

| Statement | Semantics |
|---|---|
| bind(id, expression) | evaluate pure expression once, immutable typed local |
| require(id, condition) | false is reject; no committed effects |
| set(id, target expression, field ID, expression) | evaluate target and RHS on current tentative state; type check against field; stage write; validation of all final invariants at transaction boundary |
| branch(id, condition, then, else) | evaluate bool; execute one ordered arm |
| draw_uniform(id, min/max literal ints, local ID) | one recorded in-range value; identity rule/instruction/occurrence; explicit supplied receipt required for first fixture; no uncontrolled provider call |
| choose_bool(id, authorized decision source, local ID) | consume one authorized bool receipt or suspend at checkpoint; no default GM |
| emit(id, event kind ID, typed payload) | stage event in statement order; kind/payload declared or strictly schema-checked; no recursive scheduling |

Guards evaluated before effects, draw and human decision. Require inside a procedure also rejects whole uncommitted candidate. Checked intermediate expression arithmetic; may stage a temporarily inconsistent value across multiple writes, but final resulting fields and constraints must be valid. No arbitrary loops/recursion, time, event trigger or user code. Source IDs globally unique inside model (including rule/instruction/local/parameter IDs), explicit scope for bindings. Preserve ordered statements and argument/key lists on normalization; only unordered sets sorted. Limits: 32 expression depth and 4096 nodes per expression, 4096 statements/nesting 32 per rule, 10000 evaluation steps per request (counts scans and expression evaluation, not whole project validation; project validator separately bounded). Rejected execution never consumes authoritative resources. Unsupported operation fails at compile.

## Request, transaction and journal

Request includes run_id, request_id (opaque stable ID), model snapshot identity/version, engine version, rule_id, expected_revision, typed args and trusted principal; canonical content equality checked for repeated identity. Record status Pending/Committed/Rejected/Cancelled/Stale plus original request and explicit input receipts. For duplicate same content return existing receipt after authority check. Different content under same ID or conflicting decision fails closed and cannot replace receipt. For invalid initial request an implementation may reject without storing if ID/authority invalid; after accepting identity terminal outcome is immutable.

Pseudocode:

~~~text
submit(ctx, request, explicit_inputs):
    validate run/model/runtime version and trusted authority
    if stored request ID:
        return immutable receipt iff equivalent payload and authorized
    check expected_revision and typed args
    compile or reuse validated pinned rule
    evaluate all entry guards against current state
    evaluate ordered steps on candidate copy, collect tentative writes/events
    if missing required human choice:
        store immutable checkpoint with base revision and receipt sequence
        return Pending (no mechanical effects)
    if missing/invalid draw, invalid effect, budget, constraint or guard:
        record rejection when identity accepted (no mechanical effects)
    validate candidate using model constraints
    commit candidate + ordered effects/events + terminal receipt + revision increment once
~~~

Resume requires matching immutable checkpoint, request ID, model/runtime version, authorized actor and valid choice input. Check current mechanical revision equals saved base before resuming; otherwise store terminal Stale with no effects. Recompute tentative candidate from original snapshot plus recorded inputs or retain immutable candidate and continuation; first implementation may recompute to avoid storing AST program counters, but must not re-request recorded inputs, reconsume choices or duplicate effects. Cancellation is administrative receipt, no mechanical revision increment; stale/cancel cannot commit. Denial can be a valid chosen outcome with explicit state change/event, not necessarily invalid. Commit compares expected revision again under one run owner/mutex; do not claim multi-writer persistent transaction. Commit and receipt indivisible in memory. Stage ordered effect records (target, field, before, after), event kind/payload, instruction ID, input occurrence and transaction ordinal; revision increments once per committed action including legitimate zero writes. Administrative status changes separate from mechanical journal. Replay application verifies old values, revisions, IDs, ordinals and receipts; re-execution verifies exact effect/event log against recorded input list, no live RNG/AI. Failure to validate log is error, not correction.

## Explicit first scenarios

| Case | Inputs and expectation |
|---|---|
| Lantern Attempt | args recruit=neri, trial=first_trial; require focus>=1; set focus to old focus-1; draw uniform [1,20]; if roll+rank>=difficulty emit Passed else Missed. Roll 10 -> focus 2, Passed; 9 -> focus 2, Missed; focus 0 -> reject unchanged; roll 21 or missing -> reject unchanged. Invariant violation on final candidate -> rollback all. |
| Council Deliberate | arg proposal=bridge; guard draft and sum_where_ref(pledge,pledged_proposal,bridge,strength)>=minimum_support. Authorized human choice true sets status passed, false sets rejected; respective event. Pending state remains draft; cancel unchanged; unauthorized choice/reused differing payload rejects; changed base revision stale; duplicate equivalent resume returns prior outcome. With pledge strengths 2 and 1 threshold 3 passes guard; threshold 4 rejects without human query. No randomness. |
| Competing requests | A and B checkpoint at revision 0; commit A -> rev 1; resolve B -> Stale, no second effect; retry A returns same committed receipt. |
| Replay | apply ordered effects to initial snapshot, compare final state/revision; independently execute original inputs, compare trace; corrupt before/after/duplicate sequence rejects. |

## Explicit exclusions

No persistent run store or filesystem atomic replacement, seed algorithm guarantee (supplied receipts first, named RNG later), multi-step scheduler, general query language, open-world rule inference, generalized scripting, arbitrary state edits in UI or AI. Those are later research gates. All statements should use canonical compiler/validator and independent oracle tests. This contract closes the initial design task only; #8/#10/#13 and #16 must verify real behavior before claiming Phase 2 completion.
