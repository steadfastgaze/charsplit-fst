# German Compound Splitter - Rust Reimplementation

**Project:** TDD-based Rust reimplementation using FST (Finite State Transducer) data structures

**Status:** Complete - All tests passing, Python bindings available

## Quick Overview

Rust reimplementation of a German compound word splitter that previously used Python and JSON data. The reimplementation achieves:

- 51% memory reduction: 39 MB JSON to 19 MB FST
- UTF-8/Unicode safe: Proper character-based indexing (not byte-based)
- Comprehensive tests: Python characterization tests + Rust tests
- Python bindings: Drop-in replacement for original Python API
- CLI tool: Single-word and stdin modes for batch processing

## Table of Contents

1. [Architecture](#architecture)
2. [Critical Implementation Details](#critical-implementation-details)
3. [File Structure](#file-structure)
4. [Build & Run](#build--run)
5. [Python Usage](#python-usage)
6. [Web/WASM Usage](#webwasm-usage)
7. [Testing](#testing)
8. [Performance](#performance)
9. [Known Issues & Solutions](#known-issues--solutions)

---

## Architecture

### Data Flow

```
Input Word → Lowercase → Hyphen Check? → Yes: Split at last '-'
                                        → No:  Try all split positions (3 to len-2)
                                                → Calculate score for each
                                                → Sort by score descending
                                                → Return top results
```

### Score Formula

For each potential split position:
```
score = start_prob - in_prob + pre_prob
```

Where:
- `start_prob` = Max prefix probability of second part (after Fugen-S removal)
- `in_prob` = Min infix probability crossing the split boundary
- `pre_prob` = Max suffix probability of first part (after Fugen-S removal)

### Fugen-S Detection

Fugen-S is a linking "s" in German compounds. These endings are detected and removed:

**Patterns:** `ts`, `gs`, `ks`, `hls`, `ns`

**Examples:**
- `beits` → `beit` (removes `ts`)
- `tags` → `tag` (removes `gs`)
- `mehls` → `mehl` (removes `hls`)

**Constraint:** Only remove if result would be > 2 characters

---

## Critical Implementation Details

### UTF-8/Character Indexing

**The most important technical decision:** Rust strings are UTF-8 encoded, and using byte indices (e.g., `&str[n..]`) will panic if `n` falls in the middle of a multi-byte character like `ä` (2 bytes), `ö` (2 bytes), or `ß` (2 bytes).

**Solution:** Always work with character indices, then convert to byte positions for slicing:

```rust
// CORRECT - Character-based
let chars: Vec<char> = word.chars().collect();
let char_count = chars.len();

for split_pos in 3..char_count.saturating_sub(2) {
    // Convert character position to byte position
    let byte_end: usize = chars[0..split_pos]
        .iter()
        .map(|c| c.len_utf8())
        .sum();

    let part1 = &word[..byte_end];  // Safe!
    let part2 = &word[byte_end..];  // Safe!
}

// WRONG - Will panic on Unicode
for split_pos in 3..word.len().saturating_sub(2) {
    let part1 = &word[..split_pos];  // PANIC if split_pos is mid-character!
}
```

### FST Key Ordering

**FST requires keys to be inserted in sorted order.** If you insert keys out of order, building the FST will fail.

```rust
// CORRECT
let mut builder = MapBuilder::new(Vec::new()).unwrap();
builder.insert("a".as_bytes(), value).unwrap();
builder.insert("b".as_bytes(), value).unwrap();

// WRONG - Will panic!
builder.insert("b".as_bytes(), value).unwrap();
builder.insert("a".as_bytes(), value).unwrap();
```

### f64 to u64 Conversion

FST stores values as `u64`, but probabilities are `f64`. Use bitwise conversion:

```rust
fn f64_to_u64(v: f64) -> u64 {
    v.to_bits()  // Lossless conversion
}

fn u64_to_f64(v: u64) -> f64 {
    f64::from_bits(v)
}
```

---

## File Structure

### Source Files (`src/`)

| File | Purpose |
|------|---------|
| `lib.rs` | Public API exports, Python module definition |
| `splitter.rs` | Main `Splitter` struct, `split_compound()` algorithm |
| `score.rs` | `ScoreCalculator`, score formula implementation |
| `ngram.rs` | `NgramLookup`, FST wrapper for probability lookups |
| `fugen_s.rs` | Fugen-S detection and removal |
| `error.rs` | Error types: `Error` enum (Io, Fst, InvalidData, WordTooShort), `Result<T>` alias |
| `python_bindings.rs` | PyO3 wrapper for Python API |
| `cli.rs` | Command-line interface binary |
| `web.rs` | WASM bindings for browser usage |

### Test Files

| File | Purpose |
|------|---------|
| `tests/integration_tests.rs` | End-to-end tests, Python parity checks |
| `tests/test_python_bindings.py` | Python binding API tests |
| `tests/characterization/*.py` | Python characterization tests (document current behavior) |
| Unit tests (inline in src/*.rs) | Module-level unit tests |

### Tools

| File | Purpose |
|------|---------|
| `tools/convert_json_to_fst.rs` | Convert `split_words/ngram_probs.json` to FST format |
| `tools/verify_conversion.py` | Verify JSON data before conversion |

### Generated Data (`data/`)

Run `cargo run --bin convert_json_to_fst` to generate:

```
data/suffix.fst  - 435,910 entries, 6.2 MB
data/prefix.fst  - 424,816 entries, 5.1 MB
data/infix.fst   - 1,010,100 entries, 7.0 MB
```

---

## Build & Run

### Prerequisites

- Rust 1.70+ (tested with 1.93.1)
- Python 3.8+ (for characterization tests and Python bindings)
- maturin (for Python bindings): `cargo install maturin`
- Existing `split_words` Python package (for data conversion)

### Build

```bash
cargo build --release
```

**Build features:**
- Default (Rust library only): `cargo build --release`
- With Python bindings: `cargo build --release --features python`
- With WASM support: `cargo build --release --features web`

### Generate FST Files

```bash
# Verify JSON data first
python tools/verify_conversion.py

# Convert to FST
cargo run --bin convert_json_to_fst
```

### Usage

```rust
use charsplit_fst::Splitter;

let splitter = Splitter::new().unwrap();
let results = splitter.split_compound("Autobahnraststätte");

for result in results.iter().take(3) {
    println!("{:.3} | {} | {}",
        result.score, result.part1, result.part2);
}
```

Output:
```
0.795 | Autobahn | Raststätte
0.650 | Auto | Bahnraststätte
0.542 | Autobahnrast | Stätte
```

---

## Python Usage

The Rust implementation can be used from Python via PyO3 bindings. It provides a drop-in replacement for the original `split_words.Splitter` class.

### Installation

```bash
maturin develop
```

### Python API

```python
from charsplit_fst import Splitter

splitter = Splitter()
results = splitter.split_compound("Autobahnraststätte")

# Results are list of (score, part1, part2) tuples
for score, part1, part2 in results[:3]:
    print(f"{score:.3f} | {part1} | {part2}")
```

### Command-Line Interface

The package includes a CLI tool `charsplit-fst`:

```bash
# Single word mode
charsplit-fst Autobahnraststätte

# Stdin mode (one word per line)
echo "Autobahnraststätte" | charsplit-fst
```

**Output format:**
- Single word mode: `{score}\t{part1}\t{part2}` (tab-separated, 3 decimal places, top 10 results)
- Stdin mode: `{part1}\t{part2}\t{original_word}` (tab-separated, best result only)

---

## Web/WASM Usage

The Rust implementation can be compiled to WebAssembly for browser-based usage.

### Build

```bash
cargo build --release --features web
```

### JavaScript API

```javascript
import { WebSplitter } from './charsplit_fst.js';

// Load FST data and create splitter
const splitter = new WebSplitter(suffixData, prefixData, infixData);
const results = splitter.split_compound("Autobahnraststätte");
// Returns: [{score: 0.795, part1: "Autobahn", part2: "Raststätte"}, ...]
```

**Convenience function** (for one-off splits without creating a splitter instance):
```javascript
import { split_word_once } from './charsplit_fst.js';

const results = split_word_once(suffixData, prefixData, infixData, "Autobahnraststätte");
```

### Raw Data Loading

For custom data loading (useful for embedded systems or testing):

```rust
use charsplit_fst::Splitter;

let suffix_bytes = std::fs::read("data/suffix.fst")?;
let prefix_bytes = std::fs::read("data/prefix.fst")?;
let infix_bytes = std::fs::read("data/infix.fst")?;

let splitter = Splitter::from_raw_data(&suffix_bytes, &prefix_bytes, &infix_bytes)?;
```

---

## Testing

### Run All Tests

```bash
# Rust tests
cargo test

# Python binding tests
python -m pytest tests/test_python_bindings.py -v

# Python characterization tests
python -m pytest tests/characterization/ -v
```

### Test Results

All tests passing (run `cargo test` to verify).

### Python Binding Tests

Tests verify API compatibility with original Python implementation:

- Basic compound splitting
- Return type matching (list of tuples)
- Empty/short word handling
- Unicode character handling
- Hyphenated word handling
- Multiple result ordering
- Case-insensitive input

### Python Characterization Tests

These tests **document current Python behavior** of the original Python implementation:

Key test files:
- `test_basic_splits.py` - Common compounds
- `test_fugen_s.py` - Fugen-S edge cases
- `test_hyphen_handling.py` - Hyphenated words
- `test_edge_cases.py` - Short words, Unicode, case variations
- `test_score_ranking.py` - Scoring logic verification

---

## Performance

### Memory Usage

| Implementation | Memory | Reduction |
|----------------|--------|-----------|
| Python + JSON | 180 MB | baseline |
| JSON file only | 39 MB | - |
| **Rust + FST** | **19 MB** | **51% vs JSON** |
| **Total savings** | - | **89% vs Python** |

### Benchmarks

Run benchmarks with:
```bash
cargo bench
```

Results in `target/criterion/`:

**Current performance (MacBook M1):**
- Latency: ~230μs per word
- Throughput: ~4,400 words/second
- Memory: < 40 MB total

---

## Known Issues & Solutions

### Issue 1: Byte Indexing Panics on Unicode

**Problem:**
```rust
let word = "autobahnraststätte";
let part = &word[..12];  // PANIC! byte 12 is in the middle of 'ä'
```

**Solution:** Always use character-based indexing

### Issue 2: FST Build Order Panics

**Problem:**
```rust
builder.insert("z", value)?;
builder.insert("a", value)?;  // PANIC! out of order
```

**Solution:** Sort keys before inserting:
```rust
let mut pairs: Vec<(&str, u64)> = data.iter().collect();
pairs.sort_by_key(|(k, _)| *k);

for (key, value) in pairs {
    builder.insert(key.as_bytes(), value)?;
}
```

### Issue 3: Empty FST Maps in Tests

**Problem:** `Map::new(Vec::new())` fails because empty vectors aren't valid FSTs.

**Solution:** Insert a dummy value:
```rust
let mut builder = MapBuilder::new(Vec::new()).unwrap();
builder.insert(b"\x00", 0).unwrap();
let dummy_map = Map::new(builder.into_inner().unwrap()).unwrap();
```

### Issue 4: `to_title_case` Only Capitalizes First Character

**Problem:** Input "bundes-autobahn" becomes "Bundes-autobahn" (not "Bundes-Autobahn").

**Reason:** The function uppercases only the first character. For hyphenated words, each part is processed as one string, so "bundes-autobahn" becomes "Bundes-autobahn" (second part remains lowercase).

---

## Algorithm Details

### Main Algorithm (in `split_compound()`)

1. Convert input to lowercase
2. **Hyphen check:** If hyphen found, split at last hyphen and return
3. **Iterate split positions:** From character position 3 to `len - 2`
4. **For each position:**
   - Apply Fugen-S to first part for suffix probability lookup
   - Apply Fugen-S to second part for prefix probability lookup
   - Calculate `pre_slice_prob` = suffix probability of first part after Fugen-S (exact match, defaults to -1.0)
   - Calculate `start_slice_prob` = prefix probability of second part after Fugen-S (exact match, defaults to -1.0)
   - Calculate `in_slice_prob` = min infix probability (starts from length 3, defaults to 1.0)
   - Score = `start_prob - in_prob + pre_prob`
5. Sort results by score descending
6. Return sorted list (or `[(0.0, word, word)]` if no splits found)

**Note:** Probability lookups use exact matches only, not cascading to shorter ngrams.

**Score guarantees:** Every valid split position gets a score. Missing probabilities default to: `pre_slice_prob` = -1.0, `start_slice_prob` = -1.0, `in_slice_prob` = 1.0.

### Fugen-S Removal

Applied to both parts of the split:

```rust
fn remove_fugen_s(s: &str) -> Option<&str> {
    let patterns = ["ts", "gs", "ks", "hls", "ns"];

    for pattern in &patterns {
        if s.ends_with(pattern) && s.len() > 3 {
            return Some(&s[..s.len() - 1]);
        }
    }
    None
}
```

**Note:** The length check (`s.len() > 3`) ensures the result is > 2 characters.

### Probability Lookup Strategy

**Important:** The implementation uses exact-match lookups only, not cascading to shorter ngrams.

**Suffix probability** (for first part):
```rust
// Only look up the full slice (no cascade)
self.lookup.get_suffix_prob(slice).or(Some(-1.0))
```

**Prefix probability** (for second part):
```rust
// Apply Fugen-S, then look up the full slice (no cascade)
let slice = remove_fugen_s(slice).unwrap_or(slice);
self.lookup.get_prefix_prob(slice).or(Some(-1.0))
```

**Infix probability:**
```rust
let mut min_prob = None;
// Start from length 3, not 1
for len in 3..=chars_from_split.len() {
    let ngram = &word[split_byte_pos..byte_end];
    if let Some(p) = self.lookup.get_infix_prob(ngram) {
        min_prob = Some(match min_prob {
            Some(current_min) => current_min.min(p),
            None => p,
        });
    }
}
// Default to 1.0 if no ngrams found
min_prob.or(Some(1.0))
```

---

## Development Notes

### Adding New Tests

**Unit tests:** Add inline in `src/*.rs` files:
```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_my_feature() {
        assert_eq!(my_function(), expected);
    }
}
```

**Integration tests:** Add to `tests/integration_tests.rs`:
```rust
#[test]
fn test_my_integration() {
    let splitter = Splitter::new().unwrap();
    let result = splitter.split_compound("testword");
    assert!(!result.is_empty());
}
```

**Python characterization tests:** Add to `tests/characterization/*.py`:
```python
def test_new_word(self, splitter):
    """Test new word behavior."""
    result = splitter.split_compound("NewWord")
    assert len(result) > 0
    # Document actual behavior, not desired behavior
```

### Code Style

- **Rustfmt:** Run `cargo fmt` before committing
- **Clippy:** Run `cargo clippy` to catch common issues
- **Documentation:** All public items must have doc comments
- **Tests:** Aim for > 90% code coverage

### Dependencies

- `fst = "0.4"` - Finite State Transducer library
- `thiserror = "1.0"` - Error handling
- `serde_json = "1.0"` - JSON parsing (for conversion tool)
- `pyo3 = "0.22"` - Python bindings (optional, via "python" feature)
- `wasm-bindgen = "0.2"` - WebAssembly bindings (optional, via "web" feature)
- `web-sys = "0.3"` - Web API bindings (optional, via "web" feature)
- `criterion = "0.5"` - Benchmarking (dev-dependency)

---

## Troubleshooting

### "No such file or directory" when running tests

**Problem:** `Splitter::new()` can't find FST files.

**Solution:** Run `cargo run --bin convert_json_to_fst` first to generate `data/*.fst` files.

### Tests fail with "byte index is not a char boundary"

**Problem:** Code is using byte indices on UTF-8 strings.

**Solution:** Convert to character-based indexing (see "Critical Implementation Details" above).

### FST build fails with "out-of-order key"

**Problem:** Keys are being inserted in unsorted order.

**Solution:** Sort keys before inserting into FST builder.

### Python tests cannot import `split_words`

**Problem:** `split_words` package not in Python path.

**Solution:** Run tests from project root: `python -m pytest tests/characterization/`

---

**Rust version:** 1.93.1
