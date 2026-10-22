# Birdsite

[![build](https://github.com/travisbrown/birdsite/actions/workflows/ci.yml/badge.svg)](https://github.com/travisbrown/birdsite/actions/workflows/ci.yml)
[![codecov](https://codecov.io/gh/travisbrown/birdsite/branch/main/graph/badge.svg)](https://codecov.io/gh/travisbrown/birdsite)
[![crates.io](https://img.shields.io/crates/v/birdsite.svg)](https://crates.io/crates/birdsite)
[![docs.rs](https://docs.rs/birdsite/badge.svg)](https://docs.rs/birdsite)

This project is a small Rust library for working with Twitter data.

## Command-line tools

`birdsite-cli` processes compact snapshot files; `birdsite-hacks` contains experimental commands.
Run `cargo run -p birdsite-cli -- --help` or `cargo run -p birdsite-hacks -- --help` for usage.

Both tools log warnings and errors by default. Use `-v` for informational diagnostics, `-vv` for
debug output, `-vvv` for trace output, or `-q` / `--quiet` for errors only. These flags apply before
or after subcommands. Quiet mode preserves command output, including JSON summaries and CSV rows.

Successful commands exit with status 0. `birdsite-cli validate` exits with status 1 when its summary
reports problems. Operational failures and invalid command-line arguments exit with status 2.

## Development

Rust 1.98 or later is required. The committed lockfile fixes the dependency versions used in CI;
a separate CI job tests the newest compatible dependencies.

Run the main checks with:

```console
cargo +nightly fmt --all -- --check
cargo clippy --locked --all-targets --all-features -- -D warnings
cargo test --locked --all-features
RUSTDOCFLAGS="-D warnings" cargo doc --locked --all-features --no-deps
rumdl check .
taplo fmt --check --diff
taplo lint
cargo deny --workspace check
```

Rust formatting uses nightly for import grouping. CI uses rumdl 0.2.79 and Taplo 0.10.
The default workspace members include the libraries and CLI. The `birdsite-hacks` experiments
are checked separately on Linux because building RocksDB requires a C++ compiler and libclang.
Use `cargo test --locked -p birdsite-hacks` to run their tests locally.

The transaction ID integration test contacts the live X website and is opt-in:

```console
cargo test --locked -p birdsite-graphql-ctid --test integration_test -- --ignored
```

Archived snapshot tests use a local cache and may download captures from the Wayback Machine.
Private fixture tests skip when their data is absent. CI decrypts the fixture bundles only when
the fixture secret is available, then runs those tests with all output suppressed.

## License

This project is licensed under the [GNU Affero General Public License, version 3
only](https://www.gnu.org/licenses/agpl-3.0.html). See [LICENSE](LICENSE) for the full text.
