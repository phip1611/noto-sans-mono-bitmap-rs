#!/usr/bin/env sh

# Invokes the codegen project in "./codegen" to generate the crate in
# "./src". Afterwards it applies Rustfmt to it, Clippy, and builds everything.

set -e
set -x

# nice "hack" which make the script work, even if not executed from "./"
DIR=$(dirname "$(realpath "$0")")
cd "$DIR" || exit

echo "This script generates the crate 'noto-sans-mono-bitmap', verifies the build, and applies Rustfmt and clippy afterwards."

# delete all generated raster files from previous run
find src/res_rasterized_characters -type f -name "*.txt" -exec rm {} +

cd "codegen" || exit
cargo update
cargo fmt
cargo test
RUSTFLAGS="-C target-cpu=native" cargo run --release --bin codegen
cd ..

# For simplifications, the generated code doesn't care too much about
# formatting. Hence, we always execute this step.
cargo fmt

if [ "$1" != "--skip-checks" ]; then
  cargo test --doc --features all
  cargo test --all-targets --features all
  cargo clippy --features all  --all-targets
  cargo doc --features all --document-private-items --no-deps
  cargo build --features all --all-targets
else
  echo "Skipping checks"
fi
