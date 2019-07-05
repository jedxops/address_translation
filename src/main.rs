// Copyright © 2019 Jeff Austin, Kamakshi Nagar
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

extern crate rand;
pub mod setup;
pub mod lib_fns;

// main function
fn main() {
    // test
    setup::generate_segmented_memory_layout();
}
