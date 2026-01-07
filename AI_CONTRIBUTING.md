# AI Coding Guidelines (Rust)

These guidelines apply to *all* AI-assisted code changes in this repository.

## Workflow
- Read the full contents of any file you plan to change, plus directly related modules.
- Summarize current behavior and invariants before proposing edits.
- Propose a minimal patch plan (files + rationale) before modifying code.

## Rust Style & Design
- Correctness first; then idiomatic, reviewable Rust.
- Prefer clarity over cleverness: small functions, early returns, shallow nesting.
- Keep diffs small and reviewable; avoid cosmetic churn.
- Do not include expository or 'my way' style comments
- Do not include any comments that focus on the change itself and lack suitable generality ('low overhead version' or 'fully optimal version', etc.)
- Comments should document the code not the change we are making

## Naming
- Naming must be semantic, not pattern-based.
- Avoid suffixes like `State`, `Context`, `Manager` unless there is a real contrast
  (e.g., `Config` vs `Runtime`, `Snapshot` vs `Live`).
- Do not use either prefixes or suffixes as namespaces.
- Rust is strongly typed, do not express type information through naming

## Abstraction
- Abstract only when it removes duplication or encodes invariants.
- Prefer concrete domain types over generic wrappers.

## Style
- prefer the standard library
- use external packages only with approval
- never duplicate code

## Quality Gates
- Always add test coverage for added functionality
- Code must compile, pass tests, respect schema stability.
- Run `cargo fmt`, `cargo clippy`, and relevant tests after edits.
- Unit tests must not access external files and data, however integration tests may