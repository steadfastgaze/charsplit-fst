// SPDX-License-Identifier: MIT OR Apache-2.0

//! Benchmark suite for the German splitter.
//!
//! Run with: `cargo bench`

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use charsplit_fst::Splitter;

fn bench_split_compound(c: &mut Criterion) {
    let splitter = Splitter::new().expect("Failed to create splitter");

    let test_words = [
        "Autobahnraststätte",
        "Behördenangaben",
        "Donaudampfschifffahrt",
        "Bundesregierung",
        "Arbeitsamt",
        "Hilfskraft",
        "Wirtschaftsschule",
    ];

    let mut group = c.benchmark_group("split_compound");

    for word in test_words {
        group.bench_with_input(BenchmarkId::from_parameter(word), &word, |b, word| {
            b.iter(|| {
                splitter.split_compound(black_box(word))
            })
        });
    }

    group.finish();
}

fn bench_split_compound_long_words(c: &mut Criterion) {
    let splitter = Splitter::new().expect("Failed to create splitter");

    // Very long compound words
    let long_words = [
        "Donaudampfschifffahrt",
        "Rheinlandpfalz",
        "Niedersachsen",
    ];

    let mut group = c.benchmark_group("split_long_words");

    for word in long_words {
        group.bench_with_input(BenchmarkId::from_parameter(word), &word, |b, word| {
            b.iter(|| {
                splitter.split_compound(black_box(word))
            })
        });
    }

    group.finish();
}

fn bench_split_compound_short_words(c: &mut Criterion) {
    let splitter = Splitter::new().expect("Failed to create splitter");

    // Short words
    let short_words = ["Haus", "Bad", "Baum", "Tor"];

    let mut group = c.benchmark_group("split_short_words");

    for word in short_words {
        group.bench_with_input(BenchmarkId::from_parameter(word), &word, |b, word| {
            b.iter(|| {
                splitter.split_compound(black_box(word))
            })
        });
    }

    group.finish();
}

fn bench_split_compound_with_hyphen(c: &mut Criterion) {
    let splitter = Splitter::new().expect("Failed to create splitter");

    // Hyphenated words (should be very fast - early return)
    let hyphen_words = [
        "Bundes-Autobahn",
        "Arbeitsamt-Gebäude",
        "Donaudampfschifffahrt-Kapitän",
    ];

    let mut group = c.benchmark_group("split_hyphenated");

    for word in hyphen_words {
        group.bench_with_input(BenchmarkId::from_parameter(word), &word, |b, word| {
            b.iter(|| {
                splitter.split_compound(black_box(word))
            })
        });
    }

    group.finish();
}

fn bench_fugen_s_detection(c: &mut Criterion) {
    use charsplit_fst::fugen_s::{remove_fugen_s, should_remove_fugen_s};

    let fugen_s_words = ["arbeit", "hilfs", "tag", "werk"];
    let non_fugen_s_words = ["haus", "baum", "tor"];

    let mut group = c.benchmark_group("fugen_s_detection");

    group.bench_function("should_remove_fugen_s_positive", |b| {
        b.iter(|| {
            for word in fugen_s_words {
                black_box(should_remove_fugen_s(black_box(word)));
            }
        })
    });

    group.bench_function("should_remove_fugen_s_negative", |b| {
        b.iter(|| {
            for word in non_fugen_s_words {
                black_box(should_remove_fugen_s(black_box(word)));
            }
        })
    });

    group.bench_function("remove_fugen_s", |b| {
        b.iter(|| {
            for word in fugen_s_words {
                black_box(remove_fugen_s(black_box(word)));
            }
        })
    });

    group.finish();
}

fn bench_title_case(c: &mut Criterion) {
    let mut group = c.benchmark_group("title_case");

    let test_words = [
        "autobahnraststätte",
        "behördenangaben",
        "arbeitsamt",
        "wirtschaftsschule",
    ];

    group.bench_function("to_title_case", |b| {
        b.iter(|| {
            for word in test_words {
                black_box(word.chars().next().unwrap().to_uppercase().collect::<String>() + &word[word.len()..]);
            }
        })
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_split_compound,
    bench_split_compound_long_words,
    bench_split_compound_short_words,
    bench_split_compound_with_hyphen,
    bench_fugen_s_detection,
    bench_title_case
);

criterion_main!(benches);
