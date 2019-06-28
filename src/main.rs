// Copyright © 2019 Jeff Austin, Kamakshi Nagar
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

use std::process::exit;
extern crate rand;
use rand::Rng;

///!
///!

// return a random power of 2 (u32)
fn get_pw2() -> u32 {
    let mut found = false;
    let mut done = false;
    let mut dummy: u32 = 2;
    let mut num: u32 = 0;
    
    let mut rng = rand::thread_rng();
    while !found {
        num = rng.gen_range(2, 302);
        /*if num == 8 || num == 256 || num == 64 || num == 2 || num == 32 {
            println!("Num: {}", num);
        }*/
        done = false;
        while !done {
            while dummy < num {
                dummy *= 2;
            }
            if dummy == num {
                found = true;
                done = true;  // its a power of 2 --success!
            }
            else if dummy > num {
                done = true;  // its not a power of 2 and we need to generate again
            }
        }
        if done == true && found == true {
            return num;
        }
        dummy = 2;
        done = false;
        found = false;
        rng = rand::thread_rng();
    }
    0
}

// main function
fn main() {
    // ** vas = size of virtual address space * 1024 bytes (i.e. K)
    // ** PM = size of physical memory * 1024 bytes (i.e. K)
    // ** size of the vas = 2^power_of_2

    // calculate vas
    let mut VAS: u32 = get_pw2();
    // calculate the number of bits in the VAS
    let mut power_of2: u32 = 1;
    let mut dummy: u32 = 1;  // assume at least 1 bits
    for i in 1..20 {  // i will start at 1 and go through 19 --we would need 18 bits to represent a 256K VAS
        if dummy == (VAS*1024) {  // not going to be true for first iteration
            power_of2 = i - 1;
            break;
        }
        dummy *= 2;
    }
    let PM: u32 = VAS * 2;


    // print the basic information
    println!();
    println!("Assume a {}KB virtual address space and a {}KB physical memory. Virtual addresses are {} bits and segmentation is being used. The segment information is:\n", VAS, PM, power_of2);

    println!("\t\tSegment Number\tBase\tSize\tGrowsNegative");
    println!("\t\tCode\t00\t16K\t3.5K\t0");
    println!("\t\tHeap\t01\t8K\t5.0K\t0");
    println!("\t\tStack\t11\t48K\t4.0K\t1");
}
