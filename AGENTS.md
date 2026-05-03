# Rust Coding Rules

## Writing
- No fancy words. Keep text simple, easy to understand, and minimal. English is not the first language for many readers (comments, docs, errors, and agent replies).

## Dependency Policy (Strict)
- Do not use third-party crates for new code or when editing existing code.
- Do not add new dependencies to any `Cargo.toml`.
- Implement features using only the Rust standard library (`std`, `core`, `alloc`) and existing in-repo code.
- If shared capability is needed, build it as an internal module/framework inside this repository.
- If a requested change cannot be done without an external crate, stop and request explicit approval first.

### Approved exception (terminal GUI)
These crates are allowed only for `punk_gui`, `punk_terminal` (GUI/PTY/VT paths), and shared `[workspace.dependencies]` entries used by them:

- **winit, wgpu, pollster, bytemuck, fontdue, vte** — window, GPU, sync init, buffer casts, font raster, escape parsing.
- **libc** — Unix PTY and TTY `ioctl` in `punk_terminal::pty` (no PTY wrapper crate).
- **windows-sys** (narrow `features` only) — Win32 ConPTY, pipes, I/O, and process startup in `punk_terminal::pty`.

PTY logic stays in-repo. Do not add other third-party crates under this exception without updating this list.

## Generic Rust Code Rules
- Keep code idiomatic, readable, and strongly typed.
- Prefer explicit error handling with `Result` and clear error messages.
- Avoid `unwrap`/`expect` in production paths unless there is a documented invariant.
- Keep modules cohesive and functions focused.
- Add tests for non-trivial behavior changes when feasible.
- Use `cargo fmt`-compatible style and keep code compliant with `rustfmt.toml`.
