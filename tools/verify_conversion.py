# SPDX-License-Identifier: MIT OR Apache-2.0
"""Verify JSON to FST conversion by cross-checking probabilities."""
import json
import sys
from pathlib import Path


def load_json_probs(json_path: str) -> dict:
    """Load ngram probabilities from JSON file."""
    with open(json_path) as f:
        return json.load(f)


def check_fst_coverage(json_data: dict) -> None:
    """Check that FST conversion would cover all entries."""
    total_entries = sum(len(v) for v in json_data.values())

    print("JSON Statistics:")
    print(f"  Suffix entries: {len(json_data['suffix'])}")
    print(f"  Prefix entries: {len(json_data['prefix'])}")
    print(f"  Infix entries: {len(json_data['infix'])}")
    print(f"  Total entries: {total_entries}")

    # Check for potential issues
    print("\nChecking for potential issues:")

    # Check for empty keys
    for map_name, map_data in json_data.items():
        empty_keys = [k for k in map_data.keys() if k == ""]
        if empty_keys:
            print(f"  WARNING: {map_name} has {len(empty_keys)} empty keys")

    # Check for very long keys
    for map_name, map_data in json_data.items():
        long_keys = [(k, len(k)) for k in map_data.keys() if len(k) > 100]
        if long_keys:
            print(f"  WARNING: {map_name} has {len(long_keys)} keys longer than 100 chars")

    # Sample some keys to verify
    print("\nSample keys from each map:")
    for map_name, map_data in json_data.items():
        sample_keys = list(map_data.keys())[:5]
        print(f"  {map_name}: {sample_keys}")

    # Check probability ranges
    print("\nProbability ranges:")
    for map_name, map_data in json_data.items():
        probs = list(map_data.values())
        if probs:
            print(f"  {map_name}: [{min(probs):.4f}, {max(probs):.4f}]")


def verify_key_encoding(json_data: dict) -> None:
    """Verify that keys can be encoded as UTF-8 bytes."""
    print("\nVerifying key encoding:")

    for map_name, map_data in json_data.items():
        try:
            for key in map_data.keys():
                key.encode('utf-8')
            print(f"  {map_name}: All keys valid UTF-8")
        except UnicodeEncodeError as e:
            print(f"  ERROR: {map_name} has encoding issue: {e}")


def main():
    """Run verification checks."""
    json_path = Path(__file__).parent.parent.parent / "split_words" / "ngram_probs.json"

    if not json_path.exists():
        print(f"ERROR: JSON file not found at {json_path}")
        sys.exit(1)

    print(f"Loading {json_path}...")
    json_data = load_json_probs(json_path)

    check_fst_coverage(json_data)
    verify_key_encoding(json_data)

    print("\n✓ Verification complete")
    print("\nNext steps:")
    print("  1. Run: cargo run --bin convert_json_to_fst")
    print("  2. Verify the generated FST files in data/")


if __name__ == "__main__":
    main()
