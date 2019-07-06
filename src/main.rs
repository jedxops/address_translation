// Copyright © 2019 Jeff Austin, Kamakshi Nagar
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

extern crate rand;
pub mod setup;
pub mod lib_fns;

// main function
fn main() 
{
    println!("Welcome ");
    println!("Do you want your random problem or a specific one?");
     use std::io::{stdin,stdout,Write};
    let mut s=String::new();
    let _=stdout().flush();
    stdin().read_line(&mut s).expect("Did not enter a correct string");
    println!("your answer is {}",s);
    //let arg = u64::from_str(&arg).expect("bad argument");

    //let v :vec =vec![foo(),bar()];

    setup::generate_segmented_memory_layout();
}
