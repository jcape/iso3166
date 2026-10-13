# AGENTS.md

This file provides guidance to generative AI agents when working with code in this repository.

## Commands

```sh
cargo build --workspace --all-features
cargo test --workspace --all-features            # CI uses `cargo nextest run --workspace` on MSRV 1.88.0
cargo test -p iso3166-static <test_name>         # single test in one crate
cargo clippy --all --all-features                # CI sets RUSTFLAGS="-D warnings"
cargo fmt --all -- --check
cargo doc --workspace --all-features --no-deps --document-private-items
cargo +nightly no-std-check --manifest-path static/Cargo.toml   # iso3166-static must stay no_std
cargo deny --all-features check
taplo format && taplo check                      # TOML formatting/linting
prek run --all-files                             # run all pre-commit hooks (see prek.toml)
```

CI also builds `--workspace --release --no-default-features` with each feature combination of `alloc`/`serde` on stable, beta and MSRV 1.88.0, so changes in `static` must compile with features off.

## Architecture

Cargo workspace of three crates sharing one version (`shared-version`, released together via release-plz; tags `v{version}`):

- **`parsers`** (`iso3166-parsers`): serde `Record` types for third-party ISO 3166 data distributions — `lukes` (Luke Duncalfe's `all.json`), `m49` (UN M49 CSV), `ip2location`. Dated source data files live next to each module (e.g. `src/lukes/2025-12-29.json`) and are used by unit tests via `include_str!`.
- **`macros`** (`iso3166-macros`): proc-macro `generate!(lukes_json = "...")`. It reads the JSON at compile time, resolved relative to the *calling* crate's `CARGO_MANIFEST_DIR/src/`, deserializes it with `iso3166_parsers::lukes::Record`, and emits the `Error`, `Numeric`, `Alpha2` and `Alpha3` enums plus their `const fn` conversions (`from_u16`, `from_alpha2`, `as_str`, `is_user_assigned`, cross-type `PartialEq`, …). Enum variant names come from `name_to_ident` in `macros/src/lukes.rs`, which PascalCases the country name and applies hand-written renames (e.g. `UnitedKingdom`, `SouthKorea`) — new or renamed countries may need a mapping there.
- **`static`** (`iso3166-static`): the published user-facing crate. `#![no_std]`, invokes `generate!(lukes_json = "all.json")` on `static/src/all.json` (a copy of the parsers' lukes JSON), and hand-writes trait impls (`Display`, `TryFrom`, `FromStr`) around the generated types in `lib.rs`; serde impls are in `serde_.rs` behind the `serde` feature.

Data flow: data file → `parsers` record types → `macros` codegen → `static` enums. Changing the generated API means editing the `quote!` blocks in `macros/src/lukes.rs`; the user-visible trait glue lives in `static/src/lib.rs`.

Each crate's `README.md` is included as its crate-level docs (`doc = include_str!("../README.md")`), so README code blocks run as doctests. Large data files are excluded from pre-commit hooks in `prek.toml`; add new ones there.

## Conventions

- Workspace lints are very strict (clippy `pedantic` plus many restriction lints, all `deny`; see root `Cargo.toml`). Notable: every item needs docs including private ones (`missing_docs_in_private_items`), public functions need `#[inline]`, no `unwrap()`, no `mod.rs` files (use `foo.rs` + `foo/`), `absolute_paths` denied, no `unsafe`. `#[allow]` is denied — use `#[expect(lint, reason = "...")]`.
- Commit messages must follow Conventional Commits (enforced by a commit-msg hook; release-plz derives changelogs from them).
- Import/module ordering (CONTRIBUTING.md): `extern crate`, then `pub use`, then `pub mod`, then `mod`, then `use`. Re-export types at the crate root; group related free functions in a `pub mod`; prefer `{}` scopes over `drop()`.
