#!/bin/bash

mkdir -p /workspaces/iso3166/.cache/cargo
ln -sf /usr/local/cargo/bin /workspaces/iso3166/.cache/cargo/

cargo binstall -q -y --force prek
cargo binstall -q -y --force action-validator
cargo binstall -q -y --force cargo-deny
cargo binstall -q -y --force cargo-llvm-cov
cargo binstall -q -y --force cargo-semver-checks
cargo binstall -q -y --force cargo-nextest
cargo binstall -q -y --force release-plz

pushd /workspaces/iso3166 >/dev/null
prek install -f >/dev/null
popd >/dev/null
