#!/bin/bash
# SPDX-License-Identifier: MIT OR Apache-2.0
# Script to compress FST files with Brotli for web deployment

set -e

echo "Compressing FST files with Brotli..."

# Create data directory if it doesn't exist
mkdir -p web-demo/data

# Compress the FST files
cd data
brotli -k -c suffix.fst > ../web-demo/data/suffix.fst.br
brotli -k -c prefix.fst > ../web-demo/data/prefix.fst.br
brotli -k -c infix.fst > ../web-demo/data/infix.fst.br
cd ..

echo "FST files compressed with Brotli:"
ls -la web-demo/data/

echo ""
echo "For web deployment, you can load the .br files and decompress them in the browser."
echo "Or copy the original .fst files to the data directory for direct use."