// SPDX-License-Identifier: MIT OR Apache-2.0

use charsplit_fst::Splitter;
use std::thread;
use std::time::Duration;

fn main() {
    println!("READY");
    let _splitter = Splitter::new().unwrap();
    println!("LOADED");

    // Keep alive for measurement
    loop {
        thread::sleep(Duration::from_secs(1));
    }
}
