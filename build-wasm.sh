#!/bin/bash
# SPDX-License-Identifier: MIT OR Apache-2.0
# Build script for WASM version

set -e

echo "Building WASM version of charsplit-fst..."

# Check if wasm-pack is installed
if ! command -v wasm-pack &> /dev/null; then
    echo "wasm-pack is not installed. Installing..."
    cargo install wasm-pack
fi

# Build the WASM package
wasm-pack build --target web --features web --out-dir pkg-web

echo "WASM build completed successfully!"
echo "Output is in pkg-web/"