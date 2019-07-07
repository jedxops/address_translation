// Copyright © 2019 Jeff Austin, Kamakshi Nagar
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

extern crate rand;
use std::process::exit;
use std::io;
use std::io::prelude::*;
use rand::Rng;
pub mod setup;
pub mod lib_fns;

/*  Definitions:
    vas = size of virtual address space * 1024 bytes (i.e. K)
    va = virtual address
    pm = size of physical memory * 1024 bytes (i.e. K)
    size of the vas = 2^power_of_2
    ss = segment selector (or segment number)
    mss = maximum segment size

    we need a way of telling each of the functions 
        1) the format of the va that the question will specify
        2) the format that the answer should be provided in

    FLAG DEFINITIONS for va_to_pa and va_to_pa_malloc
    0 --va in hex, answer in hex
    1 --va in hex, answer in binary
    2 --va in hex, answer in decimal
    3 --va in binary, answer in hex
    4 --va in binary, answer in binary
    5 --va in binary, answer in decimal
    6 --va in decimal, answer in hex
    7 --va in decimal, answer in binary
    8 --va in decimal, answer in decimal

    FLAG DEFINITIONS for stack_va
    0 --answer in hex
    1 --answer in binary
    2 --answer in decimal    */

// main function --the menu is managed here
fn main() {

    /*
    General flow:
    1) Generate memory layout
    2) Boot menu
    3) User communicates:
       a) that he/she wants to exit --terminate program
       b) what problem they want to solve (random, specific)
    4) if a) --exit program. Else we print the memory layout and the problem
    5) force user to attempt to solve the problem
        if correct --congratulate!
    6) User communicates:
        a) if they want to see steps
        b) if they want to try another problem
        c) if they want to generate a new memory layout
        d) if they want to exit
    7) If a --show steps and re-print the same menu.
       If b --return to previous menu
       If c --have control flow return back to beginning of main
       If d --exit program 
    */

    // array of functions for address translation questions = atqs
    let mut atqs: Vec<fn(u32, u32, u32, Vec<setup::Segment>) -> (u32, u32, u32, Vec<setup::Segment>)> = Vec::new();

    atqs.push(va_to_pa);
    atqs.push(va_to_pa_malloc);
    atqs.push(stack_va);
    let x: (u32, u32, u32, Vec<setup::Segment>) = setup::generate_segmented_memory_layout();
    clear_screen();
    loop {
        // clear_screen();
        let mut input_string = String::new();
        println!("OPTION\t\tPROBLEM TYPE\n");
        println!("0\u{29}\t\tTranslate Random Virtual Address to a corresponding Physical Address");
        println!("1\u{29}\t\tTranslate Random Virtual Address Returned by Malloc() to a corresponding Physical Address");
        println!("2\u{29}\t\tCalculate Specified Portion through the Stack as a Virtual Address");
        println!("9\u{29}\t\tGenerate Random Problem");
        println!("10\u{29}\t\tExit");

        match io::stdin().read_line(&mut input_string) {
            Ok(_) => {}
            Err(_) => {continue;}
        }
        let y: i8 = match input_string.trim().parse::<i8>() {
            Ok(k) => k,
            Err(_) => {
                println!("Error. Invalid input --not an integer. Please try again.");
                -1
            },
        };
        if y == -1 {
            continue;
        }
        else {
            match y {
                0 => {atqs[0](x.0, x.1, x.2, x.3.clone());}
                1 => {atqs[1](x.0, x.1, x.2, x.3.clone());}
                2 => {atqs[2](x.0, x.1, x.2, x.3.clone());}
                9 => {let mut rng = rand::thread_rng(); // seed the rng
                    let rando: usize = rng.gen_range(0, atqs.len());
                    atqs[rando](x.0, x.1, x.2, x.3.clone());}
                10 => {return;}
                _ => {println!("Unexpected error parsing integer input. Exiting.");
                    exit(-1);}
            }
        }
    }
}

fn va_to_pa(vas: u32, pa: u32, power_of2: u32, segments: Vec<setup::Segment>) -> (u32, u32, u32, Vec<setup::Segment>) {
    let choice: u8 = choose_format_va_to_pa();
    clear_screen();
    setup::print_layout(vas, vas * 2, power_of2, segments.clone());

    // fetch random u32 in between 100 and the VAS (as a power of 2) as the virtual address to be calculated.
    let va: u32 = setup::get_rand_va(power_of2, segments.clone());
    let format_specifiers = setup::print_question_va_to_pa(va, choice, false);

    // calculate offset:
    let ss: u32 = va >> (power_of2 - 2);
    let mss: u32 = 2u32.pow(power_of2 - 2);  // MSS = 2^(number of bits in the offset)
    let mut pa: u32 = 0;
    let mut bit_mask: u32 = 0;
    for i in 0..power_of2 - 2 {  // we only want to mask the bits up to the ss
        bit_mask += 2u32.pow(i); // turning on bits in the mask value
    }
    let offset: u32 = va & bit_mask; // the expression on the left = va but with the 2 highest order bits set to 0 which is the same as the offset 

    if ss == 3 { // stack ss
        pa = setup::calculate_answer(segments[2], mss, offset);
        compare_answer(format_specifiers.1, pa);
        //setup::show_solution_va_to_pa_hex(segments[2], ss, mss, bit_mask, offset, va, pa, power_of2);
    }
    else if ss == 0 || ss == 1 {
        pa = setup::calculate_answer(segments[ss as usize], mss, offset);
        compare_answer(format_specifiers.1, pa);
        //setup::show_solution_va_to_pa_hex(segments[ss as usize], ss, mss, bit_mask, offset, va, pa, power_of2);
    }
    else {   // if so then print error message and exit. --BUG
        println!("Error. Segment selector doesnt represent any of the implemented segments. It equals {}", ss);
        println!("Exiting program.");
        exit(-1);
    }
    (vas, pa, power_of2, segments)
}

fn va_to_pa_malloc(vas: u32, pa: u32, power_of2: u32, segments: Vec<setup::Segment>) -> (u32, u32, u32, Vec<setup::Segment>) { 
    let va: u32 = setup::get_rand_va(power_of2, segments.clone());
    //let format_specifiers = setup::print_question_va_to_pa(va, choice, true);
    (vas, pa, power_of2, segments)
}

fn stack_va(vas: u32, pa: u32, power_of2: u32, segments: Vec<setup::Segment>) -> (u32, u32, u32, Vec<setup::Segment>) {
    (vas, pa, power_of2, segments)
}

fn compare_answer(aformat: u8, pa: u32) {
    let mut input = String::new();
    match aformat {
        16 => {
                    println!("Type your answer in hexadecimal format with or without the `0x`");
                    // the read_to_string writes the input data to its argument, not the return value.
                    input = match io::stdin().read_to_string(&mut input) {
                        Ok(usize_bytes) => {input.to_string().trim().to_string()},
                        Err(_) => {"".to_string()},
                    };
                    if lib_fns::are_all_numeric(&input, 16) {
                        match (lib_fns::bn_to_b10(&input.replace("0x", "").to_string(), 16)) {
                            Some(k) => if k as u32 == pa {
                                            println!("Good.");
                                       }
                                       else {
                                            println!("INCORRECT.\n");
                                            println!("your answer in dec: {}\nactual: {}", k, pa);
                                            println!("your answer: {:#X}\nactual: {:#X}", k, pa);
                                       }
                            None => {
                                println!("INCORRECT.\n");
                                println!("your answer in dec: {}\nactual: {}", input, pa);
                                println!("your answer: {}\nactual: {:#X}", input, pa);
                            }
                        }
                    }
                    else {
                        println!("INCORRECT.\n");
                        println!("your answer: {}\nactual: {:#X}", input, pa);
                    }
                    return;
        },
        2  => {
                    println!("Type your answer in binary format with or without leading zeros");
                    input = match io::stdin().read_to_string(&mut input) {
                        Ok(usize_bytes) => {input.to_string().trim().to_string()}, // as far as I know, input and the return value are one and the same thing.
                        Err(_) => {"".to_string()},
                    };
                    if lib_fns::are_all_numeric(&input, 2) {  // the second flag specifies the base which we define as `numeric`
                        match (lib_fns::bn_to_b10(&input.trim().to_string(), 2)) {
                            Some(k) => if k as u32 == pa {
                                            println!("Good.");
                                       }
                                       else {
                                            println!("INCORRECT.\n");
                                            println!("your answer: {:b}\nactual: {:b}", k, pa);
                                       }
                            None => {
                                println!("INCORRECT.\n");
                                println!("your answer: {}\nactual: {:b}", input, pa);
                            }
                        }
                    }
                    else {
                        println!("INCORRECT.\n");
                        println!("your answer: {}\nactual: {:b}", input, pa);
                    }
                    return;
     
              },
        10 => {
                    println!("Type your answer in decimal format (base 10, no decimal points)");
                    // the read_to_string writes the input data to its argument, not the return value.
                    input = match io::stdin().read_to_string(&mut input) {
                        Ok(usize_bytes) => {input.to_string().trim().to_string()},
                        Err(_) => {"".to_string()},
                    };
                    if lib_fns::are_all_numeric(&input, 10) {
                        match (lib_fns::bn_to_b10(&input.trim().to_string(), 10)) {
                            Some(k) => if k as u32 == pa {
                                            println!("Good.");
                                       }
                                       else {
                                            println!("INCORRECT.\n");
                                            println!("your answer: {}\nactual: {}", k, pa);
                                       }
                            None => {
                                println!("INCORRECT.\n");
                                println!("your answer: {}\nactual: {}", input, pa);
                            }
                        }
                    }
                    else {
                        println!("INCORRECT.\n");
                        println!("your answer: {}\nactual: {}", input, pa);
                    }
                    return; 
              },
        _ => {
                    println!("Error. Unexpected format specifier. Fatal error. Terminating program");
                    exit(-1)
             },
    }

}

fn choose_format_va_to_pa() -> u8 {
    let mut choice: i8 = 0;
    loop {
        // clear_screen();
        let mut input_string = String::new();

        println!("OPTION\t\tPROBLEM TYPE\n");
        println!("0\u{29}\t\t--va in hex, answer in hex");
        println!("1\u{29}\t\t--va in hex, answer in binary");
        println!("2\u{29}\t\t--va in hex, answer in decimal");
        println!("3\u{29}\t\t--va in binary, answer in hex");
        println!("4\u{29}\t\t--va in binary, answer in binary");
        println!("5\u{29}\t\t--va in binary, answer in decimal");
        println!("6\u{29}\t\t--va in decimal, answer in hex");
        println!("7\u{29}\t\t--va in decimal, answer in binary");
        println!("8\u{29}\t\t--va in decimal, answer in decimal");

        match io::stdin().read_line(&mut input_string) {
            Ok(_) => {}
            Err(_) => {continue;}
        }
        choice = match input_string.trim().parse::<i8>() {
            Ok(k) => k,
            Err(_) => {
                println!("Error. Invalid input --not an integer. Please try again.");
                -1
            },
        };
        if choice == -1 {
            continue;
        }
        else {
            match choice {
                0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 => {break;}
                _ => {println!("Please enter one of the digits corresponding to an option on screen");
                    continue;}
            }
        }
    }
    choice as u8
}

fn clear_screen() {
    for i in 0..50 {
        println!();
    }
}
