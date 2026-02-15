use charsplit_fst::Splitter;
use std::process::Command;

fn main() {
    let splitter = Splitter::new().expect("Failed to create splitter");
    
    // Force memory to be used
    let _result = splitter.split_compound("Autobahnraststätte");
    
    // Get memory usage using ps
    let output = Command::new("ps")
        .args(["-o", "rss=", "-p", &std::process::id().to_string()])
        .output()
        .expect("Failed to execute ps");
    
    let rss_kb = String::from_utf8_lossy(&output.stdout).trim().parse::<u64>().unwrap_or(0);
    let rss_mb = rss_kb as f64 / 1024.0;
    
    println!("RSS memory: {:.2} MB", rss_mb);
}
