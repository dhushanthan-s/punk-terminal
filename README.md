# Punk Terminal

Punk terminal is AI powered rust based terminal with integrated terminal multiplexer.

## Development setup

### Prerequisites

- **Rust** toolchain **1.85** or newer (see `rust-version` in the root `Cargo.toml`).
- **`rustfmt`** and **`clippy`** if you want to match CI and the optional pre-commit hook:

  ```sh
  rustup component add rustfmt clippy
  ```

### Clone and build

```sh
git clone https://github.com/dhushanthan-s/punk-terminal.git
cd punk-terminal
cargo build --workspace
```

### Run the GUI prototype

The `punk_gui` crate opens a window with a real PTY and the in-repo grid renderer:

```sh
cargo run -p punk_gui
```

You need a normal desktop session (display server and GPU drivers that **wgpu** can use). On minimal Linux installs, install your distro’s Vulkan/Mesa (or equivalent) packages if the window fails to open.

### Run tests

```sh
cargo test --workspace
```

### Formatting and lints (same as CI)

```sh
cargo fmt --all -- --check
cargo clippy --all-targets
```

To apply formatting:

```sh
cargo fmt --all
```