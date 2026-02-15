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
wasm-pack build --target web --features web

# Copy WASM package to web-demo
echo "Copying WASM package to web-demo/pkg-web..."
rm -rf web-demo/pkg-web
mkdir -p web-demo/pkg-web
cp -r pkg/* web-demo/pkg-web/

# Compress FST data files with Brotli for web deployment
echo "Compressing FST data files with Brotli..."

# Compress FST files to data/
for fst_file in data/suffix.fst data/prefix.fst data/infix.fst; do
    filename=$(basename "$fst_file")
    echo "  Compressing $filename -> data/${filename}.br"
    brotli --best -f "$fst_file" -o "data/${filename}.br"
done

echo "WASM build completed successfully!"
echo "Output is in pkg-web/"
echo "Compressed FST files are in data/"