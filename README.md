# Punk Terminal

Punk terminal is AI powered rust based terminal with integrated terminal multiplexer.

## Install

### Prebuilt binaries (macOS + Linux)

Download the latest archive from [GitHub Releases](https://github.com/dhushanthan-s/punk-terminal/releases).

Available targets:

- `x86_64-unknown-linux-gnu`
- `x86_64-apple-darwin`
- `aarch64-apple-darwin`

Install by extracting and copying `punk` to your PATH:

```sh
tar -xzf punk-<version>-<target>.tar.gz
sudo install -m 0755 punk-<version>-<target>/punk /usr/local/bin/punk
```

### Install from source (cargo)

From a local checkout:

```sh
cargo install --path punk_gui --bin punk --locked
```

From GitHub directly:

```sh
cargo install --git https://github.com/dhushanthan-s/punk-terminal --package punk_gui --bin punk --locked
```

### Command naming policy

- The installable system command is `punk`.
- Outside punk-terminal sessions:
  - `punk` launches the GUI terminal
  - `punk --help` and `punk --version` are available
  - other `punk <...>` command functionality is blocked
- Inside punk-terminal sessions (`PUNK_SESSION=1`), non-launch `punk` command functionality is available.

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
cargo run -p punk_gui --bin punk
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