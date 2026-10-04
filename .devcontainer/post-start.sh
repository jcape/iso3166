#!/bin/bash

rustup toolchain install 1.88.0 --profile default
rustup component add --toolchain 1.88.0 rustfmt
rustup toolchain install nightly

mkdir -p /workspaces/iso3166/.cache/cargo
ln -sf /usr/local/cargo/bin /workspaces/iso3166/.cache/cargo/

cargo binstall -q -y --force prek
cargo binstall -q -y --force action-validator
cargo binstall -q -y --force cargo-deny
cargo binstall -q -y --force cargo-llvm-cov
cargo binstall -q -y --force cargo-nextest

pushd /workspaces/iso3166 >/dev/null
prek install -f >/dev/null
popd >/dev/null
