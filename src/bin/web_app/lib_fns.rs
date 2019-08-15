// Copyright © 2019 Jeff Austin, Kamakshi Nagar
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

/*
    citations:
    Mark Morrissey --CS333 Operating Systems--Portland State University practice exams:
    https://web.cecs.pdx.edu/~markem/CS333/exams/Final%202019-01.pdf

    Bart Massey
    http://web.cecs.pdx.edu/~bart/

    Rust textbook:
    Blandy, J., & Orendorff, J. (2018). Programming Rust: Fast, safe systems development. Sebastopol: OReilly Media.

    Rocket Crate examples and project:
    https://github.com/SergioBenitez/Rocket
    https://github.com/adi105/WebHelperRocket
*/

// import necessary crates
use std::process::exit;
extern crate rand;
use rand::Rng;
use std::fmt::Write as OtherWrite;
use std::io;
use std::io::Write; // need flush() method.

// series of mathematical functions and tools used in this software system

/// # use stats::*;
/// # Examples:
///
/// ```
/// assert_eq!(23295, bn_to_b10("0x5AFF", 16);
/// ```

// base `n` to base 10
// potential for n.pow(i as u32) * x to overflow IF num has too many digits --
// this especially becomes a problem as n gets bigger.
// for example, if n = 10, the maximum amount of digits this can translate is 10.
// with hex, this number is smaller
pub fn _bn_to_b10(num: &str, n: u32) -> Option<u32> {
    let mut i: u32 = num.len() as u32;
    let mut sum: u32 = 0;
    for itr in num.chars() {
        // this iterator starts at the left hand side of the string.
        // if we have this i = i -1 at the bottom of the for loop we have overflow (with i - 1 eventually equaling -1).
        i -= 1; // so we gotta perform this base n to base 10 math from left to right --unconventional but doable with len().
        match itr.to_digit(n) {
            Some(x) => {
                sum += n.pow(i) * x;
            }
            None => {
                return None;
            }
        }
    }
    Some(sum)
}

// returns true if all the characters in the given string turn out to be numeric.
pub fn _are_all_numeric(str: &str, radix: u32) -> bool {
    for itr in str.chars() {
        // skip over hex characters
        if itr == 'x' || itr == 'X' || itr == '\n' {
            continue;
        }
        if !itr.is_digit(radix) {
            return false;
        }
    }
    true
}

// prints a number of hyphens to cover a the number of bits required by the `num` argument
pub fn print_hyphens(num: u32) -> String {
    let h = num_bits_reqd(num);
    let mut to_print = String::new();
    for _i in 0..h {
        write!(&mut to_print, "-").unwrap();
    }
    writeln!(&mut to_print).unwrap();
    io::stdout().flush().unwrap(); // ensure our output is flushed entirely, as we are not using the _println_ macro here
    to_print
}

// work in progress, works only for the first upper four binary digits as of right now.
// print a binary number such that every four characters are separated by a single space
pub fn print_readable_binary(address: u32, size_of_space: u32) -> String {
    let mut to_print = String::new();
    let multiple_of_4: u32 = size_of_space / 4; // we want to group the binary into 4 groups
    if size_of_space % 4 != 0 {
        // print the binary digits at the end of the string which dont fit in a group of 4
        to_print =
            to_print + &print_leading_zeros(address >> (4 * multiple_of_4), size_of_space % 4);
        write!(&mut to_print, "{:b} ", address >> (4 * multiple_of_4)).unwrap();
    }
    let mut array_of_binaries: Vec<u32> = Vec::new(); // array of binaries to print
    for i in 1..=multiple_of_4 {
        // these bounds are tricky to determine/prove the correctness of.

        // mask the ith and (i -1)th group of bits
        /*
            An example of what this code does:
            let i = 2
            bit mask = 1111 1111 - 1111 = 1111 0000
                              isolated this ^^ group
        */
        let bit_mask: u32 = (2u32.pow(i * 4) - 1) - (2u32.pow((i - 1) * 4) - 1);

        // now when we & and shift, we get just the grouping that we wanted to document in our vector
        let b: u32 = (address & bit_mask) >> ((i - 1) * 4);
        array_of_binaries.push(b);
    }
    // print the binary groups
    let mut i: usize = array_of_binaries.len();
    while i > 0 {
        to_print = to_print + &print_leading_zeros(array_of_binaries[i - 1], 4); // we dont want to print more than 4 leading zeros
        write!(&mut to_print, "{:b} ", array_of_binaries[i - 1]).unwrap(); // --we are already printing at least one character immediately after this
        i -= 1;
    }
    io::stdout().flush().unwrap(); // ensure our output is flushed entirely, as we are not using the _println_ macro above
    to_print
}

// prints leading zeros of a binary number up to max_bits
pub fn print_leading_zeros(num: u32, max_bits: u32) -> String {
    let mut to_print = String::new();
    let zeros_to_print = max_bits - num_bits_reqd(num);
    if zeros_to_print > 0 {
        // the leading_zeros function for usizes is not helpful here
        let mut i: u32 = 0;
        while i < zeros_to_print {
            write!(&mut to_print, "0").unwrap();
            i += 1;
        }
    }
    to_print
}

// Tricky function. Function takes a u32 and returns the number of bits required to represent that number
pub fn num_bits_reqd(num: u32) -> u32 {
    if num < 2 {
        return 1;
    }
    let mut dummy: u32 = 1; // assume at least 1 bits required
    for i in 0..=num {
        // again, value difficult to determine. user must know what argument to pass, exactly.
        // else he/she will experience incorrect results.
        // need to loop long enough to reach `num` through multiplying by 2 repeatedly.
        if dummy > num {
            return i;
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

    loop {
        // looping forever
        let num: u32 = rng.gen_range(lower_bound, upper_bound);
        if num % 2 == 0 {
            // => the number is even
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
        let num = rng.gen_range(lower - 1, upper + 1); // ensure that the lower and upper bounds are included in the search domain.
        let mut done = false; // not done multiplying
        while !done {
            while dummy < num {
                dummy *= 2;
            }
            if dummy == num {
                found = true;
                done = true; // its a power of 2 --success!
            } else if dummy > num {
                done = true; // its not a power of 2 and we need to generate again
            }
        } // done multiplying
        if done && found {
            return num;
        }
        dummy = 2;
        found = false;
    }
    0
}

//                  Unit Tests
/*--------------------------------------------------*/
//unit test for the bn_to_b10 function version 1 (hex)
#[test]
pub fn test_16_to_10_v1() {
    assert_eq!(Some(0), _bn_to_b10("0", 16));
}

//unit test for the bn_to_b10 function version 2 (hex)
#[test]
pub fn test_16_to_10_v2() {
    assert_eq!(Some(23295), _bn_to_b10("5AFF", 16));
}

//unit test for the bn_to_b10 function version 3 (hex)
#[test]
pub fn test_16_to_10_v3() {
    assert_eq!(Some(23295), _bn_to_b10("5aFf", 16));
}

//unit test for the bn_to_b10 function version 4 (hex)
#[test]
pub fn test_16_to_10_v4() {
    assert_eq!(Some(9477896), _bn_to_b10("909F08", 16));
}

//unit test for the bn_to_b10 function version 5 (binary)
#[test]
pub fn test_16_to_10_v5() {
    assert_eq!(Some(0), _bn_to_b10("0", 2));
}

//unit test for the bn_to_b10 function version 6 (binary)
#[test]
pub fn test_16_to_10_v6() {
    assert_eq!(Some(21841), _bn_to_b10("101010101010001", 2));
}

//unit test for the bn_to_b10 function version 7 (binary)
#[test]
pub fn test_16_to_10_v7() {
    assert_eq!(Some(268), _bn_to_b10("00000100001100", 2));
}

//unit test for the bn_to_b10 function version 8 (binary)
#[test]
pub fn test_16_to_10_v8() {
    assert_eq!(Some(1), _bn_to_b10("00000000000000001", 2));
}

//unit test for the bn_to_b10 function version 9 (decimal)
#[test]
pub fn test_16_to_10_v9() {
    assert_eq!(Some(0), _bn_to_b10("0", 10));
}

//unit test for the bn_to_b10 function version 10 (decimal)
#[test]
pub fn test_16_to_10_v10() {
    assert_eq!(Some(2184100), _bn_to_b10("2184100", 10));
}

//unit test for the bn_to_b10 function version 11 (decimal)
#[test]
pub fn test_16_to_10_v11() {
    assert_eq!(Some(100001100), _bn_to_b10("0100001100", 10));
}

//unit test for the bn_to_b10 function version 12 (decimal)
#[test]
pub fn test_16_to_10_v12() {
    assert_eq!(Some(1), _bn_to_b10("0000000001", 10));
}

//unit test for the bn_to_b10 function version 13 (hex)
#[test]
pub fn test_16_to_10_v13() {
    assert_ne!(Some(2184100), _bn_to_b10("2184100", 16));
}

//unit test for the bn_to_b10 function version 14 (binary)
#[test]
pub fn test_16_to_10_v14() {
    assert_ne!(Some(100001100), _bn_to_b10("0100001100", 2));
}

//unit test for the bn_to_b10 function version 15 (decimal)
#[test]
pub fn test_16_to_10_v15() {
    assert_ne!(Some(1110), _bn_to_b10("1111", 10));
}

/*---------------------------------------------------------*/

//unit test for the are_all_numeric function version 1 (hex)
#[test]
pub fn test_allnum_v1() {
    assert_eq!(true, _are_all_numeric("aBcDeF", 16));
}

//unit test for the are_all_numeric function version 2 (binary)
#[test]
pub fn test_allnum_v2() {
    assert_eq!(true, _are_all_numeric("1010101", 2));
}

//unit test for the are_all_numeric function version 3 (decimal)
#[test]
pub fn test_allnum_v3() {
    assert_eq!(true, _are_all_numeric("123456666777789", 10));
}

//unit test for the are_all_numeric function version 4 (hex)
#[test]
pub fn test_allnum_v4() {
    assert_eq!(false, _are_all_numeric("aBcDeF1h2", 16));
}

//unit test for the are_all_numeric function version 5 (binary)
#[test]
pub fn test_allnum_v5() {
    assert_eq!(false, _are_all_numeric("1010301", 2));
}

//unit test for the are_all_numeric function version 6 (decimal)
#[test]
pub fn test_allnum_v6() {
    assert_eq!(false, _are_all_numeric("12345666d777789", 10));
}

/*------------------------------------------------------------*/

// unit test for the num_bits_reqd function
#[test]
pub fn test_bits_reqd_v1() {
    assert_eq!(10, num_bits_reqd(1023));
}

// unit test for the num_bits_reqd function
#[test]
pub fn test_bits_reqd_v2() {
    assert_eq!(1, num_bits_reqd(0));
}

// unit test for the num_bits_reqd function
#[test]
pub fn test_bits_reqd_v3() {
    assert_eq!(1, num_bits_reqd(1));
}

// unit test for the num_bits_reqd function
#[test]
pub fn test_bits_reqd_v4() {
    assert_eq!(14, num_bits_reqd(16383));
}

// unit test for the num_bits_reqd function
#[test]
pub fn test_bits_reqd_v5() {
    assert_eq!(15, num_bits_reqd(16384));
}

// unit test for the num_bits_reqd function
#[test]
pub fn test_bits_reqd_v6() {
    assert_ne!(10, num_bits_reqd(1024));
}

/*------------------------------------------------------------*/
