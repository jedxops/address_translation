// Copyright © 2019 Jeff Austin, Kamakshi Nagar
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

use std::process::exit;

///!
///!

/// Report proper usage and exit.
fn usage() -> ! {
    eprintln!("stats: usage: stats [--mean|--stddev|--median|--l2]");
    exit(1);
}

/// Do the computation.
fn main() {
    // print the basic information
    // Process the argument.
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        usage();
    }
    let target = &args[1];

    // Read the input.
    use std::io::BufRead;
    let nums: Vec<f64> = std::io::stdin()
        .lock()
        .lines()
        .map(|s| {
            let s = s.unwrap_or_else(|e| {
                eprintln!("error reading input: {}", e);
                exit(-1);
            });
            s.parse::<f64>().unwrap_or_else(|e| {
                eprintln!("error parsing number {}: {}", s, e);
                exit(-1);
            })
        })
        .collect(); 
}
