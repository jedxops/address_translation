// Copyright © 2019 Jeff Austin, Kamakshi Nagar
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

// extern crate rand;
use std::process::exit;
use rand::Rng;
use std::io;
use std::io::prelude::*;
use std::io::Write;     // need flush() method.
pub use crate::lib_fns;

#[derive(Copy, Clone)]
pub struct Segment {
    name: SegName,
    base: u32,
    size: f32,
    grows_negative: u32,  // default is false the stack is the only one
                          // that grows negative so this way of structuring things is practical.
}

// used for segment name identification
#[derive(Copy, Clone)]
pub enum SegName {
    Code,           // as of right now, we are only concerned with these three segments.
    Heap,           // but this code is structured so that more segments should be able to
    Stack,          // be added without much more difficulty
}

// array of segment names for printing and comparison purposes.
pub static SEG_NAMES: [&'static str;3] = ["Code", "Heap", "Stack"];

// this function calculates the bounds of the address space and generates a segmented memory
// model for the code heap and stack sections.
pub fn generate_segmented_memory_layout() -> (u32, u32, u32, Vec<Segment>) {
    
/*  Definitions:
    vas = size of virtual address space * 1024 bytes (i.e. K)
    va = virtual address
    pm = size of physical memory * 1024 bytes (i.e. K)
    pa = physical address (answer)
    2^power_of_2 = size of the vas
    ss = segment selector (or segment number)
    mss = maximum segment size  */

    // calculate vas
    let vas: u32 = lib_fns::rand_power_of_2(lib_fns::rand_even(14, 65), lib_fns::rand_even(65, 256 + 1));

    // calculate the number of bits in the vas
    let power_of2: u32 = lib_fns::num_bits_reqd(vas * 1024);
    
    let pm: u32 = vas * 2;  // the pm should be at least double the vas

    // initialize segment structs. the stack grows down.
    let code_segment = Segment { name: SegName::Code, base: 0, size: 0.0, grows_negative: 0 };
    let heap_segment = Segment { name: SegName::Heap, base:  0, size: 0.0, grows_negative: 0 };
    let stack_segment = Segment { name: SegName::Stack, base: 0, size: 0.0, grows_negative: 1 };

    // store these newly created segment types in a vector (so we can add more of them later _if_ we want)
    let mut segments: Vec<Segment> = Vec::new();
    segments.push(code_segment);
    segments.push(heap_segment);
    segments.push(stack_segment);
    segments = are_conflicting(power_of2, segments.clone());
    (vas, pm, power_of2, segments)
}

// prints the generated segmented memory model
pub fn print_layout(vas: u32, pm: u32, power_of2: u32, segments: Vec<Segment>) {
    println!();
    println!("Assume a {}KB virtual address space and a {}KB physical memory. Virtual addresses are {} bits and segmentation is being used. The segment information is:", vas, pm, power_of2);
    // print ecessary info.
    println!("\t\tSegment Number\tBase\tSize\tGrowsNegative");
    println!("\t\t{}\t00\t{}K\t{}K\t{}", SEG_NAMES[segments[0].name as usize], segments[0].base, segments[0].size, segments[0].grows_negative);
    println!("\t\t{}\t01\t{}K\t{}K\t{}", SEG_NAMES[segments[1].name as usize], segments[1].base, segments[1].size, segments[1].grows_negative);
    println!("\t\t{}\t11\t{}K\t{}K\t{}", SEG_NAMES[segments[2].name as usize], segments[2].base, segments[2].size, segments[2].grows_negative);
}

// takes a format flag passed from the client and prints the question returning a tuple of format specifiers
pub fn print_question_va_to_pa(va: u32, format_flag: u8, malloc: bool) -> (u8, u8) {
    let qformat = match format_flag {
        0 | 1 | 2 => 0,
        3 | 4 | 5 => 1,
        6 | 7 | 8 => 2,
        _ => -1,
    };
    let aformat = match format_flag {
            0 | 3 | 6 => 16,
            1 | 4 | 7 => 2,
            2 | 5 | 8 => 10,
            _ => -1,
    };
    if aformat as i32 == -1 || qformat as i32 == -1 {
        error();
    }
    match malloc {
        false => match qformat {
                     0 => println!("Virtual Address {:#X} refers to what physical address (in base {})?", va, aformat),
                     1 => println!("Virtual Address {:b} refers to what physical address (in base {})?", va, aformat),
                     2 => println!("Virtual Address {} (base 10) refers to what physical address (in base {})?", va, aformat),
                     _ => {
                         println!("Unexpected error. Exiting");
                         error();
                     }
                 }
        true => match qformat {
                     0 => println!("A call to malloc returns a virtual address of {:#X}. What is the physical address (in base {}) of this virtual address?"
                            , va, aformat),
                     1 => println!("A call to malloc returns a virtual address of {:b}. What is the physical address (in base {}) of this virtual address?"
                            , va, aformat),
                     2 => println!("A call to malloc returns a virtual address of {} (base 10). What is the physical address (in base {}) of this virtual address?"
                            , va, aformat),
                     _ => {
                         println!("Unexpected error. Exiting");
                         error(); 
                     }
                 }
    }
    (qformat as u8, aformat as u8)
}

// given a hexadecimal va, provide the equivalent pa
pub fn va_to_pa(vas: u32, pa: u32, power_of2: u32, segments: Vec<Segment>, format_flag: u8) -> (u32, u32, u32, Vec<Segment>) {
/*
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
        pa = calculate_answer(segments[2], mss, offset);
        show_solution_va_to_pa_hex(segments[2], ss, mss, bit_mask, offset, va, pa, power_of2);
    }
    else if ss == 0 || ss == 1 {
        pa = calculate_answer(segments[ss as usize], mss, offset);
        show_solution_va_to_pa_hex(segments[ss as usize], ss, mss, bit_mask, offset, va, pa, power_of2);
    }
    else {   // if so then print error message and exit. --BUG
        println!("Error. Segment selector doesnt represent any of the implemented segments. It equals {}", ss);
        println!("Exiting program.");
        exit(-1);
    }*/
    (vas, pa, power_of2, segments)
}


// shows the `student` the steps to solving the 
pub fn show_solution_va_to_pa_hex(seg: Segment, ss: u32, mss: u32, mask: u32, offset: u32, va: u32, pa: u32, power_of2: u32) {

    println!("\nStep 1: Convert 0x{:X} to binary", va);
    print!("{:#X} = ", va);
    lib_fns::print_leading_zeros(va, power_of2);
    println!("{:b}", va);
    io::stdout().flush().unwrap();  // ensure our output is flushed entirely, as we are not using the _println_ macro here
    println!("There are great youtube guides on shortcuts for converting to binary by hand.\n");

    println!("Step 2: Note the Virtual Address Space size (in bits) and separate the Segment Selector from the Offset portion of the binary.");
    if ss != 3 {
        println!("0{} {:b}", ss, offset);
    }
    else {
        println!("{:b} {:b}", ss, offset);
    }
    print!("-- ");
    lib_fns::print_hyphens(offset);
    println!("SS {:width$}OFFSET\n", "", width = (((lib_fns::num_bits_reqd(offset))/2) - (((lib_fns::num_bits_reqd(offset))/2) / 2)) as usize);
    println!("Discard the segment selector bits from your calculation of offset.\n");
    
    println!("Step 3: Note the value of the Segment Selector.");
    println!("00 ===> Code");
    println!("01 ===> Heap");
    println!("11 ===> Stack");

    println!("Step 4: Remember the equation we are using: PA = (-1)*(GN)*(MSS) + base + offset");
    println!("GN = `grows negative`. If the SS tells you that the section");
    println!("you are looking in (i.e., if your SS = 11 => Stack) then this the value for GN would be 1. Otherwise, it is 0");
    println!("MSS  `maximum segment size`. MSS = 2^(number of bits in the offset)");
    println!("base is just the base of the segment, measured in bytes.");
    println!("offset is the value of the offset binary we have already parsed.");

    println!("Our offset is {:b} which is {} bytes in base 10 (decimal)", offset, offset);
    println!("There are {} bits in the offset, so our MSS is 2^{} = {}", power_of2 - 2, power_of2 - 2, 2u32.pow(power_of2 - 2));
    if ss == 0 {
        println!("Our SS is 00, so we are in the code segment and GN is 0");
        println!("=> PA = (-1)(0)({}) + ({} * 1024) + {}", 2u32.pow(power_of2), seg.base, offset);

    }
    else if ss == 1 {
        println!("Our SS is 01, so we are in the heap segment and GN is 0");
    }
    else if ss == 3 {
        println!("Our SS is 11, so we are in the stack segment and GN is 1");
    }
    else {
        error();
    }
}

// this function calculates the answer to the given problema and returns the result
// based on the following equation: PA = (-1)*(GN)(MSS) + base + offset
// GN = grows negative. offset = the value of the virtual address shifted left 2 bits.
// (the value of the va neglecting the final two bits)
pub fn calculate_answer(seg: Segment, mss: u32, offset: u32) -> u32 {
    ((-1) * (seg.grows_negative as i32) * (mss as i32) + (seg.base as i32 * 1024) + offset as i32) as u32
}

// generates a problem prompting for a VA to PA translation
// returns the generated result
pub fn get_rand_va(va_pow_2: u32, segments: Vec<Segment>) -> u32 {
    let mut rng = rand::thread_rng();
    let mut r: u32 = rng.gen_range(100, 2u32.pow(va_pow_2) + 1);
    let mut fresh_ss: u32 = 19;  // the ss cant be 19 in this implementation
    // ensure that the SS mimics the the code, stack or heap segment numbers.
    while (fresh_ss != 0 && fresh_ss != 1 && fresh_ss != 3) || !valid_va(r, fresh_ss, segments.clone()) {
        r = rng.gen_range(100, 2u32.pow(va_pow_2)); // exclusive on the end so this works.
        fresh_ss = r >> (va_pow_2 - 2);
    }
    r
}

// returns true if the address selected is within the scope/range of some segment.
pub fn valid_va(num: u32, fresh_ss: u32, segments: Vec<Segment>) -> bool {
    let mut enum_ss: SegName = SegName::Code;
    match fresh_ss {
        0 => enum_ss = SegName::Code,
        1 => enum_ss = SegName::Heap,
        3 => enum_ss = SegName::Stack,
        _ => {error();},  // should never reach this case
    }
    for sg in segments {
        if sg.grows_negative == 1 {
            // if num is in the stack segment's range.
            if num <= sg.base * 1024 && num as f32 >= (sg.base as f32 - sg.size) * 1024.0 && sg.name as u32 == enum_ss as u32 {
                return true;
            }
        }
        else {
            if (num as f32 <= (sg.base as f32 + sg.size) * 1024.0) && num >= sg.base * 1024 && sg.name as u32 == enum_ss as u32 {
                return true;
            }
        }
    }
    false
}

pub fn error() {
    println!("Encountered fatal error. Exiting");
    exit(-1);
}

// infinitely loops unless segments generated are not conflicting. returns a valid segment array
pub fn are_conflicting(power_of2: u32, mut segments: Vec<Segment>) -> Vec<Segment> {
    let mut rng = rand::thread_rng(); // seed the rng
    let mut conflicting = true;
    let mut out_of_bounds = true;
    // loop while the stack grows below 0 or while the segments grow into eachother
    while conflicting || out_of_bounds {
        conflicting = false;
        out_of_bounds = false;  // assume we are OK unless one of the conditions fails.

        for seg in 0..segments.len() {
            // we want the bases to be a number as a multiple of K (1024) so we divide by 1024
            // note: gen_range is exclusive with the upper bound.
            if SEG_NAMES[segments[seg].name as usize] == "Code" {
                segments[seg].base = rng.gen_range(0, (2u32.pow(power_of2 - 3)) / 1024);
                segments[seg].size = ((rng.gen_range(1.0,
                    ((2u32.pow(power_of2 - 2) / 1024) - ((2u32.pow(power_of2 - 3)) / 1024)) as f32)*10.0) as u32) as f32 /10.0;
            }
            else if SEG_NAMES[segments[seg].name as usize] == "Heap" {
                segments[seg].base = rng.gen_range(2u32.pow(power_of2 - 2) / 1024, 2u32.pow(power_of2 -1)/1024);
                segments[seg].size = ((rng.gen_range(1.0, 
                    ((2u32.pow(power_of2 - 1) + 2u32.pow(power_of2 - 2))/1024 - 2u32.pow(power_of2 -1)/1024) as f32)*10.0) as u32) as f32 / 10.0;
            }
            else if SEG_NAMES[segments[seg].name as usize] == "Stack" {
                segments[seg].base = rng.gen_range((2u32.pow(power_of2 - 1) + 2u32.pow(power_of2 - 2))/1024, 2u32.pow(power_of2)/1024);
                segments[seg].size = ((rng.gen_range(1.0, 
                    (((2u32.pow(power_of2 - 1) + 2u32.pow(power_of2 - 2))/1024) - (2u32.pow(power_of2 -1)/1024)) as f32)*10.0) as u32) as f32 / 10.0;
            }
        }

        // the following checks may or may not be necessary based on how we are seeding the base and size generations above.
        // idea to use _two_ for loops here and a reference type inspired by Bart Massey:
        for i in 0..segments.len() {
            for j in 0..segments.len() { // rustic-safe! 
                if i == j {
                    continue;
                }

                let seg1 = &segments[i];
                let seg2 = &segments[j];

                if seg1.grows_negative == 1 {
                    if seg1.base as f32 - seg1.size < 0.0 { // if this if statement is false then we have a useable stack base.
                        out_of_bounds = true;
                    }
                    // what this basically says:
                    // if the base of a non-stack segment lies between the stack base (bottom of the stack)
                    // and the stack base minus the stack size OR 
                    // if the stretch of the non-stack segment lies between this range, then these two segments are 
                    // conflicting with one another and therefore we must `roll` again. We have a non-valid set of segments.
                    else if (seg2.base <= seg1.base && seg2.base as f32 >= seg1.base as f32 - seg1.size) ||
                            (seg2.base as f32 + seg2.size >= seg1.base as f32 - seg1.size && seg2.base <= seg1.base) ||
                            (seg2.base as f32 + seg2.size <= seg1.base as f32 && seg2.base as f32 + seg2.size >= seg1.base as f32 - seg1.size)
                    {
                        conflicting = true;
                    }
                }
                // if either of the two non-stack segments grow into eachother or are based in one of each other's range they are conflicting.
                else if seg1.grows_negative == 0 && seg2.grows_negative == 0 {
                    if (seg1.base >= seg2.base && seg1.base as f32 <= seg2.base as f32 + seg2.size) || (seg1.base <= seg2.base &&
                        seg2.base as f32 <= seg1.base as f32 + seg1.size) {
                        conflicting = true;
                    }
                }
            }
        }
    }
    segments
}
