#!/usr/bin/env bash

# Invokes the codegen project in "./codegen" to generate the crate in
# "./src". Afterwards it applies Rustfmt to it, Clippy, and builds everything.

set -e
set -x

# nice "hack" which make the script work, even if not executed from "./"
DIR=$(dirname "$(realpath "$0")")
cd "$DIR" || exit

cd "codegen" || exit
cargo run --bin codegen
cd ..

cargo fmt
cargo clippy --features all
cargo doc --features all
cargo check --features all
