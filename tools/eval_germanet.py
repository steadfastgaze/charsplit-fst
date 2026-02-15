#!/usr/bin/env python3
# SPDX-License-Identifier: MIT OR Apache-2.0
"""
Evaluate splitter performance on GermaNet compound test set.

NOTE: This script requires the GermaNet compound test set which must be obtained separately from:
http://www.sfs.uni-tuebingen.de/lsd/compounds.shtml

This data has restrictive licensing and is NOT included in this repository.

Usage:
    python tools/eval_germanet.py path/to/split_compounds_from_GermaNet13.0.txt
"""
import sys
from pathlib import Path

def eval_germanet(germanet_file: str):
    """
    Evaluate splitter on GermaNet compounds.

    Args:
        germanet_file: Path to GermaNet compound test file
    """
    try:
        from charsplit_fst import Splitter
    except ImportError:
        print("Please install charsplit-fst first: maturin develop")
        sys.exit(1)

    splitter = Splitter()
    correct = 0
    total = 0

    with open(germanet_file, "r") as f:
        for line in f.readlines()[2:]:  # Skip header lines
            if not line.strip():
                continue
            parts = line.strip().split('\t')
            if len(parts) < 3:
                continue

            compound = parts[0]
            expected_split = parts[1]

            results = splitter.split_compound(compound)
            if results:
                actual_split = results[0][1]  # Best split's first part
                if actual_split == expected_split:
                    correct += 1
                total += 1

    accuracy = (correct / total * 100) if total > 0 else 0
    print(f"Accuracy: {correct}/{total} ({accuracy:.1f}%)")

if __name__ == "__main__":
    if len(sys.argv) < 2:
        print(__doc__)
        sys.exit(1)
    eval_germanet(sys.argv[1])
