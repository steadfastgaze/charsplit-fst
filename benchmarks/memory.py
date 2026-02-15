#!/usr/bin/env python3
# SPDX-License-Identifier: MIT OR Apache-2.0
"""
Memory benchmark for German compound splitter.
Measures RSS (Resident Set Size) after loading FST data.
"""

import subprocess
import sys
import os
import time

def get_rss_mb(pid):
    """Get RSS in MB for process."""
    result = subprocess.run(
        ['ps', '-o', 'rss=', '-p', str(pid)],
        capture_output=True,
        text=True
    )
    return int(result.stdout.strip()) / 1024

def main():
    # Get project root (script is in benchmarks/, project root is one level up)
    script_dir = os.path.dirname(os.path.abspath(__file__))
    project_root = os.path.dirname(script_dir)

    # Build release binary
    print("Building release binary...")
    subprocess.run(
        ['cargo', 'build', '--release', '--example', 'test_memory'],
        cwd=project_root,
        check=True,
        capture_output=True
    )

    binary_path = os.path.join(project_root, 'target/release/examples/test_memory')

    # Launch the process
    proc = subprocess.Popen(
        [binary_path],
        stdout=subprocess.PIPE,
        cwd=project_root
    )

    # Wait for LOADED signal
    for line in proc.stdout:
        if line.decode().strip() == 'LOADED':
            break

    # Measure memory
    rss_mb = get_rss_mb(proc.pid)
    print(f"Memory usage: {rss_mb:.1f} MB")

    # Cleanup
    proc.terminate()
    proc.wait(timeout=5)

if __name__ == '__main__':
    main()
