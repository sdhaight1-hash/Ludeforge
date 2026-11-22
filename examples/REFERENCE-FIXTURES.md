# Original reference fixture specifications (draft)

All examples are original minimal mechanical exercises, not implementations or ports of commercial games. Names and values are invented here; expected values are independent hand-calculated test oracles. The first two gate Phase 1 schema expressivity; their executable cases gate Phase 2. Later cases expand coverage only as justified.

## A. Lantern Academy — class-track challenge

Types: Role (label, maximum rank integer 1..3), Recruit (role reference, rank 1..3, focus 0..4), Trial (difficulty 1..25). A Recruit's rank cannot exceed its Role maximum. Define a Lantern role with maximum rank 3 and recruit Neri with rank 2 and focus 3; trial threshold 12.

Procedure Attempt: require focus >= 1; consume one focus on valid attempt; supplied roll uniform integer 1..20; result roll + rank >= difficulty. Success emits Passed; otherwise emits Missed. Rejected action commits neither state nor events.
- Roll 10 -> 12, pass; focus 3 -> 2, one terminal outcome.
- Roll 9 -> 11, fail; focus 3 -> 2 (valid attempted failure).
- Focus 0 -> reject, no change; invalid roll 21 -> reject, no change.
- Exact pass distribution: 11 of 20 outcomes (10..20), 55%.
- Role reference to missing role, rank 4, wrong value type, duplicate ID all reject. Renaming role label preserves identity and references. Two distinct recruits may share a role.

## B. Council of Reeds — diceless deliberation

Types: Delegate (name, influence integer 0..5), Proposal (sponsor ref Delegate, status enum draft/passed/rejected, minimum support integer 0..5), Pledge (delegate and proposal refs, strength 0..5). Rule OfferPledge requires referenced delegates and proposal exist, draft status, strength <= delegate influence. Unique (delegate, proposal) pledge; influence is not spent in this fixture. Support = sum of valid pledges. Deliberation requires support >= minimum; authorized table participants choose approve/reject (the specific authority is optional project data, no GM requirement); engine records choice and changes status. No dice, levels, health, combat, or clock required.
- Proposal threshold 3, pledges 2 and 1 -> support 3; request decision, state remains draft.
- Authorized approve -> passed; duplicate resume cannot commit a second outcome.
- Unauthorized or malformed decision -> reject without mutation.
- Editing a premise after checkpoint -> stale decision rejection; re-evaluate against new revision.
- Submit unchanged proposal with support 2 -> reject before requesting a decision.
- Renaming delegate retains all pledge relations; dangling references rejected.

## C. Glass Market — point allocation and modifier stacking

Designer-defined package containing purchase options with integer cost, mutually exclusive group IDs and declared prerequisite refs. Budget 8; costs 3 + 5 valid, 5 + 5 invalid. Two options of same exclusion group reject. Define additive modifiers +2/+3 and a group where only greatest +4 of +1/+4 applies; base 10 -> 19. Explicitly order cap after accumulation. Detect missing prerequisite and direct derived-value cycle. No mandatory class structure.

## D. Tidemill Workshop — resource conservation and procedures

Material stock 6, workpiece consumes 4 stock units staged at start, unfinished record has 2 remaining operations. Completing each changes progress exactly once. Finalization converts 4 committed units to 3 product mass + 1 waste; stock 2 plus product 3 plus waste 1 equals initial 6. Interrupted procedure persists; duplicate completion cannot generate another product. Cancellation has explicit recovery policy; rejected action cannot spend materials. Temporal/resource feedback is permitted, provided test invariants and bounds hold.

## E. Wayfarer Chronicle — narrative progress and lifepaths

An ordered record of life stages with designer-defined type/age interval and optional prompt. Stage 1: duration 2; stage 2: duration 3 -> final age increment 5. No time unit imposed globally. A pending opportunity can be accepted or dismissed; progress increments on confirmation exactly once; progress is derived from committed acceptances. Capacity 2; shortening below confirmed count rejected. Human chooses outcome from bounded options; no accidental RNG. Test forked alternatives preserve prior history and independent state.

## F. Mosswater Crossing — future percentile/anatomical stress case

Original skill threshold 63 on uniform 1..100; 63 successes. A targeted injury affects a named body-part reference and modifies movement; declared condition influences next attempt, which may create another injury. This is a temporal feedback loop, not a circular same-step formula. Do not import published hit-location tables. Compare baseline and variant under fixed policy, bounded encounters and explicit initial states.

## Competency questions

Can the same kernel represent both A and B without engine-specific type additions? Can it express C's allocation, D's transactions, E's human procedures and F's feedback without hidden scripts? Can every reference, rule read/write set, constraint and rejection be inspected? Can a designer author these from empty projects eventually? Record missing primitives rather than adding specialized categories preemptively.

## Test protocol

Schema fixtures: serialize/deserialize semantically unchanged, reject mutations independently (duplicates, dangling endpoint, wrong kind, missing required field, unsupported version). Runtime fixtures: assert exact state delta, events ordered, rejected actions unchanged, duplicate/stale action handling, deterministic replay and explicit human decision input. No benchmarks or implementation results claimed yet.