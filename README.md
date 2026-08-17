# Punk Terminal

Punk terminal is AI powered rust based terminal with integrated terminal multiplexer.

## Install

The installable command is `punk` (the GUI terminal). Pick any method below.

### Quick install (macOS + Linux)

```sh
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/dhushanthan-s/punk-terminal/releases/latest/download/punk_gui-installer.sh | sh
```

This downloads the right build for your machine and puts `punk` on your PATH.

### Prebuilt archives (GitHub Releases)

Download an archive for your platform from [GitHub Releases](https://github.com/dhushanthan-s/punk-terminal/releases/latest).

Available targets:

- `x86_64-unknown-linux-gnu`
- `aarch64-unknown-linux-gnu`
- `x86_64-apple-darwin`
- `aarch64-apple-darwin`

Extract and copy `punk` to a directory on your PATH:

```sh
tar -xf punk_gui-<target>.tar.xz
sudo install -m 0755 punk_gui-<target>/punk /usr/local/bin/punk
```

On macOS, if you download the archive with a browser, Gatekeeper may block it. Remove the quarantine flag once:

```sh
xattr -dr com.apple.quarantine /usr/local/bin/punk
```

The `curl` and `cargo install` methods do not set this flag, so they are not affected.

### Install from source (cargo)

From a local checkout:

```sh
cargo install --path punk_gui --bin punk --locked
```

From GitHub directly:

```sh
cargo install --git https://github.com/dhushanthan-s/punk-terminal --package punk_gui --bin punk --locked
```

### Linux runtime note

`punk` renders with the GPU (**wgpu**). You need a desktop session with working GPU drivers. On minimal Linux installs, install your distro's Vulkan/Mesa (or equivalent) packages if the window fails to open.

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

### Cutting a release

Releases are produced by [cargo-dist](https://axodotdev.github.io/cargo-dist/). Tagging a version builds all Linux and macOS targets and publishes a GitHub Release with archives, checksums, and the `curl` installer.

```sh
git tag v0.1.0
git push origin v0.1.0
```

To change targets or installers, edit `[workspace.metadata.dist]` in the root `Cargo.toml`, then run `dist generate` to regenerate `.github/workflows/release.yml`.
