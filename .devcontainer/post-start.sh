#!/bin/bash

rustup toolchain install nightly

mkdir -p /workspaces/iso3166/.cache/cargo
ln -sf /usr/local/cargo/bin /workspaces/iso3166/.cache/cargo/

cargo binstall -q -y --force --locked prek
cargo binstall -q -y --force --locked action-validator
cargo binstall -q -y --force --locked cargo-deny
cargo binstall -q -y --force --locked cargo-llvm-cov
cargo binstall -q -y --force --locked cargo-nextest
cargo binstall -q -y --force --locked cargo-no-std-check
cargo binstall -q -y --force --locked release-plz
cargo binstall -q -y --force --locked taplo-cli

pushd /workspaces/iso3166 >/dev/null
prek install -f >/dev/null
popd >/dev/null
