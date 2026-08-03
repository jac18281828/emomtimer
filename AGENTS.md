# AI Coding Guidelines (Rust)

These guidelines apply to all AI-assisted code changes in this repository.

## Workflow
1. Summarize current behavior and invariants before proposing edits.
2. **Ask each time** — `Cargo.toml` deps, cross-module or public-API refactors, file deletions,
   CI or release changes.
3. **Always ask** — merging to `main`, opening a PR, tags, force ops, anything that touches
   shared state or `main`.

## Rust Style & Design
- Correctness first; then idiomatic, reviewable Rust.
- Prefer clarity over cleverness: small functions, early returns, shallow nesting.
- Keep diffs small and reviewable; avoid cosmetic churn.
- Do not include expository or 'my way' style comments.
- Do not include comments that focus on the change itself and lack suitable generality ('low overhead version', 'fully optimal version', etc.).
- Comments should document the code, not the change being made.

## Naming
- Naming must be semantic, not pattern-based.
- Avoid suffixes like `State`, `Context`, `Manager` unless there is a real contrast (e.g., `Config` vs `Runtime`, `Snapshot` vs `Live`).
- Do not use prefixes or suffixes as namespaces. If everything starts with or ends with `_name_`, nothing should.
- Rust is strongly typed; do not express type information through naming.

## Abstraction
- Abstract only when it removes duplication or encodes invariants.
- Prefer concrete domain types over generic wrappers.
- Avoid `unwrap`/`expect` outside of tests; truly-infallible uses with a justifying comment are acceptable.
- Use effective error handling patterns including `Result` and `Option`.

## Dependencies and Imports
- Prefer the standard library.
- Declare imports at the top of each module; keep them explicit and organized so dependencies are clear.

## Tests
- Test project behavior and contracts, not language or dependency internals.
- Avoid vacuous tests: removing or breaking target code must cause a test to fail.
- Unit tests must be hermetic: no network, no external files or assets.
- Integration tests may access external files.
- Add or update tests for every behavior change.

## Completion Gates

Before marking work complete, run and report:

1. `cargo check`
2. `cargo fmt --check`
3. `cargo clippy --all-targets --all-features --no-deps -- -D warnings`
4. All tests pass (unit, doc, and integration)

Do not mark work complete until all gates pass.

## Release

All commits land on the branch; `main` only ever sees a fast-forward.

1. Branch as `claude/<topic>`; never commit to `main` directly.
2. Add an `X.Y.Z` entry to `CHANGELOG.md` and commit — this is the release commit.
3. Tag `X.Y.Z` (signed, annotated) on the release commit.
4. Bump `Cargo.toml` to `X.Y.(Z+1)` and commit as `docs: X.Y.(Z+1)`.
5. FF-merge the branch into `main`; push `main` and the tag — the `deploy-crate` workflow publishes on tag push.
6. Delete the feature branch (local and remote).

The tag version matches the code version *at the tagged commit*; the `docs: X.Y.(Z+1)` commit prepares the *next* release.
