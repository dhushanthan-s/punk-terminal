# Rust Coding Rules

## Dependency Policy (Strict)
- Do not use third-party crates for new code or when editing existing code.
- Do not add new dependencies to any `Cargo.toml`.
- Implement features using only the Rust standard library (`std`, `core`, `alloc`) and existing in-repo code.
- If shared capability is needed, build it as an internal module/framework inside this repository.
- If a requested change cannot be done without an external crate, stop and request explicit approval first.

## Generic Rust Code Rules
- Keep code idiomatic, readable, and strongly typed.
- Prefer explicit error handling with `Result` and clear error messages.
- Avoid `unwrap`/`expect` in production paths unless there is a documented invariant.
- Keep modules cohesive and functions focused.
- Add tests for non-trivial behavior changes when feasible.
- Use `cargo fmt`-compatible style and keep code compliant with `rustfmt.toml`.
