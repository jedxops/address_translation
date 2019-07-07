// Copyright © 2019 Jeff Austin, Kamakshi Nagar
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.


use std::process::exit;
extern crate rand;
use rand::Rng;
use std::io;
use std::io::Write;     // need flush() method.

// series of mathematical functions and tools used in this software system

// base `n` to base 10
pub fn bn_to_b10(num: &String, n: u32) -> Option<u32> {
    let mut i: u32 = (num.len()) as u32;
    let mut sum: u32 = 0;
    for itr in num.chars() {  // this iterator starts at the left hand side of the string.
        // if we have this i = i -1 at the bottom of the for loop we have overflow (with i - 1 eventually equaling -1).
        i -= 1;  // so we gotta perform this base n to base 10 math from left to right --unconventional but doable with len().
        match itr.to_digit(n) {
            Some(x) => {sum = sum + (n.pow(i) * x)},
            None => {return None;},
        }
    }
    Some(sum)
}

// returns true if all the characters in the given string turn out to be numeric.
pub fn are_all_numeric(str: &String, radix: u32) -> bool {
    for itr in str.chars() {
        if itr == 'x' || itr == 'X' || itr == '\n'{
            continue;
        }
        if itr.is_digit(radix) == false {
            println!("FALSE!: {}", itr);
            return false;
        }
    }
    true
}

// prints a number of hyphens to cover a the number of bits required by the `num` argument
pub fn print_hyphens(num: u32) {
    let h = num_bits_reqd(num);
    for i in 0..h {
        print!("-");
    }
    print!("\n");
    io::stdout().flush().unwrap();  // ensure our output is flushed entirely, as we are not using the _println_ macro here
}

pub fn print_leading_zeros(num: u32, max_bits: u32) {
    let zeros_to_print = max_bits - num_bits_reqd(num);
    if zeros_to_print > 0 {
        // the leading_zeros function for usizes is not helpful here
        let mut i: u32 = 0;
        while i < zeros_to_print {
            print!("0");
            i += 1;
        }
    }
}

// function takes a u32 and returns the number of bits required to represent that number
pub fn num_bits_reqd(num: u32) -> u32 {
    if num < 2 {
        return 1;
    }
    let mut dummy: u32 = 1;  // assume at least 1 bits required
    for i in 1..num {  // need to loop long enough to reach `num` through multiplying by 2 repeatedly.
        if dummy >= num {
            return i - 1;
        }
        dummy *= 2;
    }
    println!("Unexpected --could not calculate power of 2. Exiting.");
    exit(-1);
}
// return a random even number <= the passed parameter
pub fn rand_even(lower_bound: u32, upper_bound: u32) -> u32 {
    let mut rng = rand::thread_rng();

    // using a trick for getting an even such as performing integer division and then returning
    // `num` multiplied by 2 does not work here because in the case of the lower bound being 1,
    // 1 / 2 would be zero, and we could end up with 0 return value. So we use modulo. A non-zero would be preferable.

    loop { // looping forever
        let num: u32 = rng.gen_range(lower_bound, upper_bound);
        if num % 2 == 0 { // => the number is even
            return num;
        }
    }
}

// return a random power of 2 (u32)
pub fn rand_power_of_2(lower: u32, upper: u32) -> u32 {
    let mut found = false;
    let mut dummy: u32 = 2;

    let mut rng = rand::thread_rng();
    while !found {
        let num = rng.gen_range(lower - 1, upper + 1);  // ensure that the lower and upper bounds are included in the search domain.
        let mut done = false;  // not done multiplying
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
        }  // done multiplying
        if done && found {
            return num;
        }
        dummy = 2;
        found = false;
    }
    0
}
