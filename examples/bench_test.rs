use charsplit_fst::Splitter;
use std::time::Instant;

fn main() {
    let splitter = Splitter::new().expect("Failed to create splitter");
    
    let test_words = [
        "Autobahnraststätte", "Behördenangaben", "Donaudampfschifffahrt",
        "Bundesregierung", "Arbeitsamt", "Hilfskraft", "Wirtschaftsschule",
    ];
    
    let iterations = 10000;
    let start = Instant::now();
    
    for _ in 0..iterations {
        for word in &test_words {
            std::hint::black_box(splitter.split_compound(std::hint::black_box(word)));
        }
    }
    
    let duration = start.elapsed();
    let total_ops = iterations * test_words.len();
    let ops_per_sec = total_ops as f64 / duration.as_secs_f64();
    
    println!("Total operations: {}", total_ops);
    println!("Duration: {:?}", duration);
    println!("Operations per second: {:.0}", ops_per_sec);
    println!("Average latency: {:.2} μs", duration.as_micros() as f64 / total_ops as f64);
}
