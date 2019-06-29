// Copyright © 2019 Jeff Austin, Kamakshi Nagar
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

// use std::process::exit;
extern crate rand;
use rand::Rng;

///!
///!

// return a random even number <= the passed parameter
fn rand_even(lower_bound: u32, upper_bound: u32) -> u32 {
    let mut rng = rand::thread_rng();
    let mut num: u32 = 0;
    while 1 == 1 { // looping forever
        num = rng.gen_range(2, upper_bound);
        if num % 2 == 0 && num >= lower_bound && num <= upper_bound { // => the number is even
            return num;
        }
    }
    0
}

        //rng = rand::thread_rng();
// return a random power of 2 (u32)
fn rand_powerOf2(lower: u32, upper: u32) -> u32 {
    let mut found = false;
    let mut done = false;
    let mut dummy: u32 = 2;
    let mut num: u32 = 0;

    let mut rng = rand::thread_rng();
    while !found {
        num = rng.gen_range(lower - 1, upper + 1);
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
    }
    0
}

// generates a problem prompting for a VA to PA translation
fn VA_to_PA(VA_pow2: u32) -> () { 
    let mut rng = rand::thread_rng();
    let mut hex = rng.gen_range(100, 2u32.pow(VA_pow2));
    println!("Virtual Address {:X} refers to what physical address (in base 10)?", hex);
}

// this function prints the main menu for the problem at hand and services the menu
fn problem_setup() -> () {
    // ** vas = size of virtual address space * 1024 bytes (i.e. K)
    // ** PM = size of physical memory * 1024 bytes (i.e. K)
    // ** size of the vas = 2^power_of_2

    // calculate vas
    let VAS: u32 = rand_powerOf2(rand_even(16, 64), rand_even(65, 256));
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

    let mut rng = rand::thread_rng();
    let mut code_size: f32 = 0.0;
    let mut heap_size: f32 = 0.0;
    let mut stack_size: f32 = 0.0;
    while code_size + heap_size + stack_size != (VAS as f32/4.0) as f32 {
        code_size = (((rng.gen_range(1.0, (VAS as f32/4.0) as f32) * 10.0) as u32 - 1) as f32) / 10.0;
        heap_size = (((rng.gen_range(1.0, (VAS as f32/4.0) as f32) * 10.0) as u32 - 1) as f32) / 10.0;
        stack_size = (((rng.gen_range(1.0, (VAS as f32/4.0) as f32) * 10.0) as u32 - 1) as f32) / 10.0;
        // println!("size search");
    }
    // println!("code: {} heap {} stack {}", code_size, heap_size, stack_size);

    let mut code_base: u32 = 0;
    let mut heap_base: u32 = 0;
    let mut bottom_of_stack: u32 = 0;
    let mut conflicting = true;
    let mut out_of_bounds = true;
    while conflicting || out_of_bounds {
        conflicting = true;
        out_of_bounds = true;

        code_base = rand_even(1, VAS/2);
        heap_base = rand_even(1, VAS/2);
        bottom_of_stack = rand_even(1, VAS/2);

        if ( (code_base as f32) < (heap_base as f32) || (code_base as f32) > ((heap_base as f32) + heap_size) ) && 

           ( ((code_base as f32) + code_size < (heap_base as f32) || ((code_base as f32) + code_size > (heap_base as f32) + heap_size)) ||

               (((code_base as f32) > ((heap_base as f32) + heap_size)) || ((heap_base as f32) + heap_size > (code_base as f32) + code_size)) ) &&

            ( ((code_base as f32) + code_size) < ((bottom_of_stack as f32) - stack_size) || (code_base as f32) > (bottom_of_stack as f32) ) &&

            ( (heap_base as f32) < (code_base as f32) || (heap_base as f32) > ((code_base as f32) + code_size) ) && 

            ( ((heap_base as f32) + heap_size) < ((bottom_of_stack as f32) - stack_size) || (heap_base as f32) > (bottom_of_stack as f32) ) {

            conflicting = false;  // non-conflicting base and sizes in the physical address space
        }
        if (bottom_of_stack as f32) - stack_size >= 0.0 {
            out_of_bounds = false;
        }
        println!("searching");
    }

    // print the basic information
    println!();
    println!("Assume a {}KB virtual address space and a {}KB physical memory. Virtual addresses are {} bits and segmentation is being used. The segment information is:\n", VAS, PM, power_of2);

    println!("\t\tSegment Number\tBase\tSize\tGrowsNegative");
    println!("\t\tCode\t00\t{}K\t{}K\t0", code_base, code_size);
    println!("\t\tHeap\t01\t{}K\t{}K\t0", heap_base, heap_size);
    println!("\t\tStack\t11\t{}K\t{}K\t1", bottom_of_stack, stack_size);

    VA_to_PA(power_of2);
}

// main function
fn main() {
    problem_setup();
}
