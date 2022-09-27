#!/usr/bin/env bash

# Helper to estimate the final crate size by looking into "src/res_rasterized_characters" and
# calculating its total size.

set -e

ANSI_RED="\e[31m"
ANSI_BOLD="\e[1m"
ANSI_CLEAR="\e[0m"
SIZE=$(du -sh src/res_rasterized_characters | sed 's/\s.*$//')

echo -e "The final (uncompiled) crate size will be at least $ANSI_BOLD$ANSI_RED$SIZE$ANSI_CLEAR in size".
echo "Note that"
echo "  1) crates that use this library will only use parts that they require"
echo "  2) when the Rust source code describing the raster is compiled, the amount of memory"
echo "     used is significantly smaller".
echo "Hence, the memory impact for third party crates will be significantly lower than this"
echo "number may indicate. However, this number indicates the upload/download size of this crate."
