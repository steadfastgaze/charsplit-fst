# charsplit-fst

A memory-efficient Rust port of the CharSplit algorithm for German compound splitting, using Finite State Transducers (FST).

## Overview

Charsplit-fst implements the CharSplit algorithm for splitting German compound words into their component parts. It achieves 89% memory reduction compared to the original Python implementation by using Finite State Transducer (FST) data structures.

Based on CharSplit by Don Tuggener: https://github.com/dtuggener/CharSplit

## Features

- 51% smaller data files: 39 MB JSON → 18.2 MB FST
- 89% lower memory usage: 19.6 MB vs 180 MB runtime
- UTF-8 safe: Proper character-based indexing for German Unicode characters
- Python bindings via PyO3
- WebAssembly demo for browser-based usage
- CLI tool for batch processing

## Installation

### Python

```bash
pip install charsplit-fst
```

### Rust

```bash
cargo add charsplit-fst
```

## Quick Start

### Python

```python
from charsplit_fst import Splitter

splitter = Splitter()
results = splitter.split_compound("Autobahnraststätte")
# Returns: [(0.795, 'Autobahn', 'Raststätte'), ...]
```

### Rust

```rust
use charsplit_fst::Splitter;

let splitter = Splitter::new()?;
let results = splitter.split_compound("Autobahnraststätte");
```

### CLI

```bash
cargo run --bin charsplit-fst -- Autobahnraststätte
```

## Algorithm

The algorithm splits German compounds using ngram probability scoring:

**Score formula**: `start_prob - in_prob + pre_prob`

Where:
- `start_prob`: Maximum prefix probability of second part
- `in_prob`: Minimum infix probability crossing split boundary
- `pre_prob`: Maximum suffix probability of first part

## Performance

- Memory: 19.6 MB RSS (vs 180 MB for Python)
- Data size: 18.2 MB on disk (vs 39 MB JSON)

## Web Demo

A browser-based demo using WebAssembly is available in `web-demo/`.

```bash
# Build the WASM version
./build-wasm.sh

# Serve the web-demo directory
cd web-demo
python -m http.server 8000
# Open http://localhost:8000
```

The demo runs entirely in the browser using WebAssembly. No server-side processing is required.
**Browser support:** The demo will try to use Brotli compression via DecompressionStream API where supported, falling back to uncompressed data for browsers that don't support it. Works in all modern browsers.

## Development

```bash
# Build
cargo build --release

# Run tests
cargo test

# Build Python bindings
maturin develop

# Build WASM
./build-wasm.sh
```

## Acknowledgments

This project is a Rust port of CharSplit by Don Tuggener.

- Algorithm: Based on Tuggener (2016), *Incremental Coreference Resolution for German*, University of Zurich.
- Original Implementation: dtuggener/CharSplit (https://github.com/dtuggener/CharSplit) (MIT Licensed).
- Data: The n-gram probabilities are derived from the model provided by the original author.

## License

MIT OR Apache-2.0

See LICENSE-MIT and LICENSE-APACHE-2.0 for details.
