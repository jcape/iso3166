#!/bin/bash

mkdir -p /workspaces/iso3166/.cache/cargo
ln -sf /usr/local/cargo/bin /workspaces/iso3166/.cache/cargo/

RUSTC_WRAPPER_save="$RUSTC_WRAPPER"
unset RUSTC_WRAPPER
cargo binstall -q -y --force cargo-deny
cargo binstall -q -y --force cargo-semver-checks
cargo binstall -q -y --force release-plz
cargo binstall -q -y --force action-validator
cargo binstall -q -y --force sccache
export RUSTC_WRAPPER="$RUSTC_WRAPPER_save"

pushd /workspaces/iso3166 >/dev/null
pre-commit install >/dev/null
popd >/dev/null
