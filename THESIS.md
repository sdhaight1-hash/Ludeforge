# Toward a Universal TTRPG System Engineering Environment

**Ludeforge** — An ontology-driven framework for the design, formalization, simulation, validation, testing and publication of original tabletop role-playing games.

Status: foundational research thesis and software design proposal, 2026-11-22. This Markdown edition adapts the original conversation thesis for durable repository use; interactive illustrations are represented in text. Proposals, illustrative figures, and hypotheses are not implemented features or experimental results.

## 1. Abstract

Modern tabletop role-playing game development remains predominantly document-driven. Designers rely on word processors, spreadsheets, notebooks, probability calculators and manually coordinated playtesting to develop increasingly interconnected mechanical systems.

Unlike modern software engineering, TTRPG development lacks a mature integrated environment in which rules can be formally defined, their relationships mapped, their interactions simulated, their behavior tested and their documentation generated from authoritative mechanical definitions.

This thesis proposes a system-agnostic environment founded upon a configurable mechanical ontology, executable rule definitions, relationship graphs, mathematical analysis and deterministic simulation. Rather than imposing classes, levels, attributes, hit points or dice, it provides foundational primitives from which designers construct their own systems. Its objective is not to automate creativity or replace human judgment, but to give designers the analytical and architectural capabilities that modern IDEs provide to software engineers.

## 2. Central thesis statement

**Tabletop role-playing games can be represented as networks of interconnected mechanical entities, state variables, relationships, constraints, stochastic processes and human-mediated decisions. Through a unified environment that formally represents these components, designers can develop, analyze, test, maintain and publish increasingly complex RPG systems with greater consistency, transparency and reliability than conventional document-driven methods alone permit.**

The supporting hypothesis is that understanding and maintaining interactions between mechanics is a major obstacle to complex system development. Changing armor effectiveness can alter weapon effectiveness, combat duration, injury severity, crafting costs, character builds, equipment economics, healing demand and encounter difficulty. These interactions must become first-class design objects.

## 3. Academic and technological foundations

| Foundation | Application |
|---|---|
| Systems theory | Interdependent mechanics and emergent behavior |
| Graph theory | Relationships and dependencies |
| Ontology engineering | Custom mechanical concepts and classifications |
| Probability theory | Resolution and uncertainty |
| Game theory | Incentives and dominant strategies |
| Discrete-event simulation | Combat, progression and resources |
| Agent-based modeling | Simulated players, adversaries and populations |
| Formal methods | Correctness and constraints |
| Information architecture | Rulebooks, references and compendia |
| Software engineering | Testing, debugging, versioning and modularity |
| Human-computer interaction | Accessible visual design |

### 3.1 Mechanics, Dynamics and Aesthetics

The MDA framework of Hunicke, LeBlanc and Zubek distinguishes mechanics (rules), dynamics (behavior produced by rules interacting) and aesthetics (player experience). Ludeforge engineers mechanics, observes dynamics and collects evidence about experience. Mathematical balance is not synonymous with enjoyable gameplay.

### 3.2 Existing research and software

Research references include [Machinations](https://machinations.io/) (resource flows), [Micro-Machinations](https://ir.cwi.nl/pub/21923) (formal game mechanics), [Ludii](https://ludii.games/) (composable game descriptions), [Quest Bound](https://questbound.com/) (programmable modular rulesets), [Mechflow](https://github.com/Yalton/mechflow) (mechanical analysis), [Icepool](https://github.com/HighDiceRoller/icepool) (exact dice probabilities), [RuleBindery](https://rulebindery.com/) (structured publication), [Grimory](https://natteens.itch.io/grimory) (designer-oriented workflows), and [LoreKit](https://github.com/matluz1/lorekit) (data-driven computation). The proposed contribution is conceptual integration and extension into a TTRPG-specific methodology. Reference capabilities, availability and licenses require verification in Phase 0; listing a project is not a dependency commitment.

## 4. Foundational design principles

System agnosticism; ontology-first authoring; composability; inspectable relationships; executable formal rules; explainable outcomes; reproducibility; modularity; human authority; canonical mechanical definitions; local-first operation; open and portable project formats.

### 4.1 No predefined RPG assumptions

The engine must not require classes, races, levels, experience, conventional attributes, hit points, damage, armor classes, dice, turns, action economies, GMs, individual player characters, combat, magic, spatial positioning or victory conditions. These may be optional templates. A diceless political game must be as valid as a detailed combat simulator.

## 5. The mechanical ontology

Three levels separate engine primitives (entity, type, property, relationship, state, event, expression, rule, constraint, action, context), designer-defined ontology (character, skill, wound, weapon, material, culture, spell, faction, reputation, currency, profession) and game content (longsword, blacksmithing, broken femur, silver coin, elven culture, Blackwood Guild).

### 5.1 Foundational primitives

| Primitive | Function |
|---|---|
| Entity/type | Stable identity and classification |
| Component | Reusable properties or behavior |
| Property | Descriptive or mechanical information |
| State | Mutable value at a particular time |
| Relationship | Typed connection |
| Expression | Mathematical or logical result |
| Rule | Permitted behavior or state transition |
| Action | Attempted operation |
| Event | Occurrence |
| Effect | State modification |
| Constraint | Required condition |
| Query | Retrieves information |
| Context | Circumstances of evaluation |
| Procedure | Coordinates steps and decisions |
| Distribution | Uncertain outcomes |
| Authority | Who may make or resolve decisions |

### 5.2 Relationships as mechanical objects

Relationships can have properties, conditions, cardinality, direction, duration, scope, provenance and executable significance. A character can contain an attack action that requires a combat skill, consumes stamina and uses a weapon; the skill influences accuracy and weapon influences damage. Descriptive relationships (belongs to) must be distinguishable from executable relationships (modifies).

## 6. Formal rule representation

A rule describes a permitted state transition: (S', E) = R(S, A, C, X, H), where S is state, A action, C context, X random inputs, H human decisions, R rule, S' resulting state and E emitted events.

### 6.1 Rule anatomy

An illustrative melee attack receives actor, defender and weapon; requires accessibility, reach and sufficient stamina; costs two stamina; resolves 2d10 + skill against defense; applies injury on success or records a miss; considers parry, armor and abilities; and permits authorized GM intervention. Rejected prerequisites commit no costs.

### 6.2 Visual rule programming

Typed executable nodes form a graph: attack attempted -> prerequisites -> stamina deduction -> roll comparison -> damage or miss -> commit events. Every node has validated inputs and outputs. Structured forms, visual graphs and textual rules compile into the same canonical intermediate representation.

### 6.3 Domain-specific language (illustrative, not implemented)

~~~text
rule MeleeAttack:
    actor: Character
    target: Character
    weapon: Weapon
    require weapon.accessible_by(actor)
    require distance(actor, target) <= weapon.reach
    require actor.stamina >= 2
    consume actor.stamina by 2
    attack = roll(2d10) + actor.skill("Melee")
    defense = evaluate(target.defense)
    if attack >= defense:
        damage = evaluate(weapon.damage)
        apply InjuryResolution(target, damage)
        emit AttackSucceeded
    else:
        emit AttackMissed
~~~

## 7. Proposed software architecture

Designer workspace (forms, graphs, code, documents) <-> canonical repository (ontology, rules, content, relationships, versions) -> ontology/rule compiler, dependency analyzer and schema validator -> mechanical runtime (state, actions, events, effects, randomness, decisions) <-> probability engine, simulation, debugger, tests, playtesting and balance analyzer -> publishing and interoperability.

### 7.1 Architectural separation

Maintain distinct design intent, formal specification, runtime state, observational evidence and presentation. An intention that combat be dangerous does not establish that it is dangerous; one simulated result does not establish human play experience.

## 8. Development studios

Workspaces are different editing interfaces over the same ontology and engine, not independent implementations.

| Studio | Responsibilities |
|---|---|
| Ontology | Types, components, properties, relationships, constraints, terminology |
| Mechanics | Executable nodes, conditions, expressions, events, decisions, reusable rules |
| Resolution | Dice, cards, tokens, opposed checks, degrees, deterministic and custom distributions |
| Character | Lifepaths, traits, ancestry, backgrounds, professions, advancement |
| Content | Equipment, abilities, spells, creatures, materials, factions and custom categories |
| Systems | Resource economies, crafting, survival, injury, disease, social mechanics, time |
| Simulation laboratory | Controlled experiments, comparison, statistical distributions |
| Debugger | Step execution, modifiers, event traces and explanations |
| Rulebook | Prose, generated references and stat blocks, editions and publication |

Specialized combat, physiology, magic, economy, ecology and social workspaces may derive from reusable capabilities as evidence justifies.

## 9. Mathematical and probabilistic engineering

Inspect complete distributions rather than just means. Compare uniform 1d20, triangular 2d10 and bell-shaped 3d6 at varying targets and modifiers. Support exact calculation when tractable and explicitly labeled sampling otherwise. Analyze success, expected value, variance, percentiles, critical outcomes, opposed resolution, sensitivity, conditional probabilities, resource depletion and cumulative risk.

## 10. Mechanical dependency engineering

Continuously inspect circular derived dependencies, unreachable content, dangling references, repeatable resource-generation loops, unbounded stacking, dead or redundant mechanics, conflicting rules, dimensional errors, ambiguous ordering, dominant strategies and unstable feedback. Findings must distinguish proven faults, bounded checks and heuristic evidence. Intentional cycles or asymmetry may be accepted by designers.

### 10.1 Feedback loops

Injury -> reduced mobility -> reduced defense -> further injury is a reinforcing loop. Detect its structure and quantify consequences under defined scenarios rather than declaring all cycles defective.

## 11. Simulation and automated playtesting

Simulation complements, never replaces, human playtesting.

### 11.1 Categories

Atomic rules, interactions, encounters, progression, campaigns, economies, stress tests, adversarial search, regression and comparative experiments.

### 11.2 Simulated behavior

Provide configurable policies for cautious, aggressive, optimizing, cooperative, resource-conserving, inexperienced and risk-seeking players. Test character builds with competing advancement strategies, crafting agents maximizing wealth, or combatants maximizing survival. Identify incentives under explicit assumptions.

### 11.3 Experimental dashboard

Illustrative experiment only: 100,000 encounters across six armor configurations and eight behavioral policies. Increased armor may reduce serious injury while increasing duration. Retain version, random seed, parameters, policy, sample size, uncertainty, metrics and environment. Never represent hypothetical data as actual results.

## 12. Formal verification and rule testing

Support unit, integration, property-based, scenario, regression and bounded state-space tests. Distinguish mathematical proof, exhaustive bounded checks and samples.

~~~text
test "Cannot spend unavailable stamina":
    character.stamina = 1
    attempt MeleeAttack(character, target)
    expect action.rejected
    expect character.stamina == 1
    expect no_damage_applied

invariant "Currency conservation":
    total_after = total_before + issued - destroyed
~~~

## 13. Rulebook engineering and publication

One canonical longsword (damage 2d6, weight 1.4 kg, reach 1 m) can supply values to equipment chapters, combat examples, sheets, compendia, crafting references and VTT exports. A change updates generated references; designers retain control of explanatory prose, illustrations and page design. The rulebook is a human-readable representation, not the sole database. Manual prose may disagree with mechanics and requires review.

## 14. Mechanical version control

Retain semantic changes and experimental branches. An example progression: v0.1 opposed combat; v0.2 armor changes from avoidance to reduction; v0.3 anatomical injuries; v0.4 alternative stamina experiments; v1.0 published snapshot with evidence. Report effects on rules, entities, builds, scenarios, documents and saved games; provide explicit migrations and rollback.

## 15. AI-assisted development

Optional AI can propose mechanical definitions from prose, entities and relationships, flag ambiguity, generate tests and policies, suggest alternative mechanics, explain simulations and find undocumented behavior. It works from structured project context, produces inspectable proposals and requires validation and approval before canonical changes. The deterministic runtime does not depend on an AI model.

## 16. Human adjudication

No universal engine can formalize every human action or fictional situation. Support fully executable rules, parameterized rules (GM chooses difficulty), assisted rules (engine suggests consequences), human-adjudicated rules (persuasion judgment) and descriptive rules (fictional conventions). A procedure can suspend, request an authorized decision, record it and resume. Preserve narrative freedom without pretending informal text is executable.

## 17. Technical implementation

Proposed: Rust core and CLI; Tauri with React/TypeScript desktop UI; React Flow or comparable graph editor; SQLite for local indexing/storage; structured JSON interchange; typed Rust interpreter; native exact probability validated against independent references; headless parallel CPU simulation; FTS5 search; Git-compatible project representation; HTML/CSS/PDF publishing; later sandboxed extensions and structured AI/MCP interface. These are candidates to evaluate, not final dependencies. No mandatory GPU or cloud. Establish a single authoritative project representation; derived indexes and runtime data cannot become competing sources of truth.

### 17.1 Repository structure

Proposed apps/desktop and apps/cli; crates for ontology, project, rules, expressions, runtime, probability, dependencies, validation, simulation, agents, testing and publishing; modules, examples, schemas, tests, benchmarks, research, design and Development-Audits. Start with fewer crates, split when boundaries need independent enforcement.

## 18. Research methodology

Use design-science research: build and evaluate the artifact rather than assume its efficacy.

### 18.1 Research questions

- RQ1: Can a sufficiently small ontology represent divergent TTRPGs?
- RQ2: Can visual graphs represent complex mechanics without conventional programming?
- RQ3: Can dependency analysis identify defects missed by conventional authoring?
- RQ4: Can simulation expose meaningful balance and interaction problems before playtests?
- RQ5: Can structured definitions prevent documentation inconsistencies?
- RQ6: Does formalization improve or impede creative iteration?
- RQ7: Can the same engine serve procedural and narrative games?
- RQ8: Can AI assistance improve productivity without compromising correctness?

### 18.2 Hypotheses and evaluation

| Hypothesis | Evaluation |
|---|---|
| H1 Structured rules reduce inconsistencies | Compare equivalent document-based and structured projects |
| H2 Graphs improve change-impact analysis | Time and accuracy of identifying effects |
| H3 Testing improves defect detection | Seed known faults and measure detection |
| H4 Simulation improves understanding of dynamics | Compare controlled experiments and playtests |
| H5 Generated references reduce errors | Repeated rule changes and discrepancy counts |
| H6 Visual authoring improves accessibility | Tasks for programmers and non-programmers |
| H7 Generality is achievable | Implement divergent reference games |

These are investigable hypotheses, not conclusions. For RQ8 compare accepted change rate, time, and independently detected defects with and without optional AI assistance.

## 19. Case studies

Use original fixtures and appropriately licensed material inspired by distinct philosophies: D&D-style class/level and action economy; RuneQuest-style percentile skills and anatomical damage; GURPS-style point buy and modifiers; Traveller-style lifepaths and aging; Blades in the Dark-style clocks, stress and narrative authority; The Broken Empires-style simulationist conflict; Amber-style diceless adjudication. Finally create an entirely original RPG from an empty ontology. Avoid redistributing commercial game text without permission.

## 20. Development roadmap

| Phase | Scope | Validation gate |
|---|---|---|
| 0 | Research, references, architecture | Define evidence and acceptance criteria |
| 1 | Mechanical kernel | Two divergent character schemas without kernel changes |
| 2 | Rules, actions, events, randomness, decisions | Execute and reproduce complete encounter |
| 3 | Designer workspace | Original small ruleset without Rust coding |
| 4 | Probability, dependency analysis, testing/debugging | Detect and explain seeded defects |
| 5 | Simulation, policies, comparisons | Reproduce experiments and known exploit fixtures |
| 6 | Character procedures, progression, modules | Reusable systems across divergent fixtures |
| 7 | Publishing, sheets, playtest feedback | Consistent exports and traceable feedback |
| 8 | Evaluation | Case studies, usability, performance and workflow comparison |

## 21. Proposed acceptance criteria

At least five divergent reference systems; custom types without kernel changes; probability agreement with independent calculations; identical traces given equivalent versions, inputs, seeds and decisions; all seeded dangling references detected; explicit effect ordering; reproducible regression comparisons; 100,000 lightweight encounter trials under a defined benchmark; consistent generated values; creation of a small game without general-purpose programming; lossless project interchange; suspension and resumption at human decision points. Specify hardware, workload and statistical error before measuring performance.

## 22. Risks and limitations

| Risk | Mitigation |
|---|---|
| Overgeneralized kernel | Add primitives only when fixtures justify them |
| Unwieldy graphs | Reusable subgraphs and structured expressions |
| Formalization discourages creativity | Informal and partly formalized rules |
| Misleading simulations | Explicit policies, uncertainty and human evidence |
| Computational expense | Bounded analysis, incremental evaluation, caching where measured |
| Balance mistaken for enjoyment | Separate quantitative and experiential objectives |
| Scope creep | Prove kernel before specialized workspaces |
| AI inconsistencies | Review and validate changes |
| Breaking editions | Version schemas and explicit migrations |

## 23. Primary research bibliography

Initial research corpus; verify bibliographic metadata and applicability in Phase 0.

- Hunicke, LeBlanc & Zubek (2004), [MDA: A Formal Approach to Game Design and Game Research](https://aaai.org/papers/ws04-04-001-mda-a-formal-approach-to-game-design-and-game-research/).
- Dormans (2011), [Simulating Mechanics to Study Emergence in Games](https://ojs.aaai.org/index.php/AIIDE/article/view/12477).
- Dormans (2012), Engineering Emergence: Applied Theory for Game Design.
- Adams & Dormans (2012), Game Mechanics: Advanced Game Design.
- Klint & van Rozen, [Micro-Machinations](https://ir.cwi.nl/pub/21923).
- Stephenson et al. (2019), [An Overview of the Ludii General Game System](https://doi.org/10.1109/CoG.2019.8847949).
- Piette et al. (2020), [Ludii — The Ludemic General Game System](https://doi.org/10.3233/FAIA200120).
- [Icepool project and associated dice probability research](https://github.com/HighDiceRoller/icepool).
- Hevner et al. (2004), [Design Science in Information Systems Research](https://aisel.aisnet.org/misq/vol28/iss1/10/).

## 24. Conclusion

Ludeforge represents a transition from document-oriented RPG development toward formal, computationally assisted system engineering. Character builders, probability calculators, graphs, simulation, knowledge representation, documentation and test frameworks already exist independently. The opportunity is their integration around a shared configurable representation.

A successful designer can begin with an empty project, invent a mechanical vocabulary, build interconnected rules, examine consequences, conduct controlled experiments, refine, playtest and publish without maintaining contradictory definitions. Ludeforge should become to TTRPG system design what an IDE is to software engineering, CAD to mechanical design and a DAW to music production.

**Not merely a place to write a game, but an environment in which to engineer one.**
