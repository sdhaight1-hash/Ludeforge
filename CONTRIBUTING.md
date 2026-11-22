# Development workflow

1. Read the thesis and roadmap, assess existing files and active issues.
2. Open or select an issue describing the problem, dependencies, scope and acceptance tests.
3. Record consequential design choices in an ADR; preserve superseded decisions.
4. Implement the smallest coherent vertical slice on a focused branch. Add failure and success fixtures.
5. Run formatting, linting and tests as applicable. Record exact commands and observed results, including unrun checks.
6. Submit changes linking issues, documentation and evidence. Close issues only when acceptance criteria are met.

## Definition of done

Acceptance criteria met; relevant tests actually pass; public APIs, schemas and migration implications documented; no unrelated changes; no fabricated results; reproducible failures and limitations recorded. Update ROADMAP.md or FEATURES.md only on evidence. Proposed features remain proposed.

Use descriptive commit messages. Keep source text UTF-8, use relative documentation links, and avoid committing generated build outputs, local databases or secrets. Do not add runtime dependencies without a concrete need and license review.
