#!/usr/bin/env bash

# Invokes the codegen project in "./codegen" to generate the crate in
# "./src". Afterwards it applies Rustfmt to it, Clippy, and builds everything.

set -e
set -x

# nice "hack" which make the script work, even if not executed from "./"
DIR=$(dirname "$(realpath "$0")")
cd "$DIR" || exit

echo "This script generates the crate 'noto-sans-mono-bitmap', verifies the build, and applies Rustfmt and clippy afterwards."

cd "codegen" || exit
# Needs rustc 1.58 or above
cargo +stable run --bin codegen
cd ..

cargo fmt
cargo +stable clippy --features all
cargo +stable doc --features all
cargo +stable build --features all
