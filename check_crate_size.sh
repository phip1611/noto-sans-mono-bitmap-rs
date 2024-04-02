#!/usr/bin/env sh

# Helper to estimate the final crate size by looking into "src/res_rasterized_characters" and
# calculating its total size.

set -e

ANSI_RED="\e[31m"
ANSI_BOLD="\e[1m"
ANSI_CLEAR="\e[0m"

SIZE=$(du -sh src | sed 's/\s.*$//')

echo -e "The final crate size of all sources is roughly $ANSI_BOLD$ANSI_RED$SIZE$ANSI_CLEAR in size".
echo "Note that the compiled size will be significantly smaller, especially if"
echo "only a subset of functionality is used."
