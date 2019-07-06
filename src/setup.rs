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

// this function prints the main menu for the problem at hand and services the menu
pub fn generate_segmented_memory_layout() {
    // Some Definitions:
    // ** vas = size of virtual address space * 1024 bytes (i.e. K)
    // ** va = virtual address
    // ** pm = size of physical memory * 1024 bytes (i.e. K)
    // ** size of the vas = 2^power_of_2

    // calculate vas
    let vas: u32 = lib_fns::rand_power_of_2(lib_fns::rand_even(14, 65), lib_fns::rand_even(65, 256 + 1));

    // calculate the number of bits in the vas
    let power_of2: u32 = lib_fns::num_bits_reqd(vas * 1024);
    
    let pm: u32 = vas * 2;  // the pm should be at least double the vas

    let mut rng = rand::thread_rng(); // seed the rng

    // initialize segment structs. the stack grows down.
    let code_segment = Segment { name: SegName::Code, base: 0, size: 0.0, grows_negative: 0 };
    let heap_segment = Segment { name: SegName::Heap, base:  0, size: 0.0, grows_negative: 0 };
    let stack_segment = Segment { name: SegName::Stack, base: 0, size: 0.0, grows_negative: 1 };

    // store these newly created segment types in a vector (so we can add more of them later _if_ we want)
    let mut segments: Vec<Segment> = Vec::new();
    segments.push(code_segment);
    segments.push(heap_segment);
    segments.push(stack_segment);

    // loop while the stack grows below 0 or while the segments grow into eachother
    let mut conflicting = true;
    let mut out_of_bounds = true;
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
 
    // take these values and pass them to the menu function
   menu(vas, pm, power_of2, segments);
}
// generates a user interface --performs majority of the input and output for the system.
pub fn menu(vas: u32, pm: u32, power_of2: u32, segments: Vec<Segment>) {
    // print the basic information
    println!();
    println!("assume a {}kb virtual address space and a {}kb physical memory. virtual addresses are {} bits and segmentation is being used. the segment information is:", vas, pm, power_of2);

    // continue printing the necessary info.
    println!("\t\tsegment number\tbase\tsize\tgrowsnegative");
    println!("\t\t{}\t00\t{}k\t{}k\t{}", SEG_NAMES[segments[0].name as usize], segments[0].base, segments[0].size, segments[0].grows_negative);
    println!("\t\t{}\t01\t{}k\t{}k\t{}", SEG_NAMES[segments[1].name as usize], segments[1].base, segments[1].size, segments[1].grows_negative);
    println!("\t\t{}\t11\t{}k\t{}k\t{}", SEG_NAMES[segments[2].name as usize], segments[2].base, segments[2].size, segments[2].grows_negative);

    // fetch random u32 in between 100 and the vas (as a power of 2) as the virtual address to be calculated.
    let va: u32 = get_rand_va(power_of2, segments.clone());
    println!("virtual address {:#x} refers to what physical address (in base 10)?", va);

    // more definitions:
    // ** ss = segment selector (or segment number)
    // ** mss = maximum segment size
    let ss: u32 = va >> (power_of2 - 2);
    let mss: u32 = 2u32.pow(power_of2 - 2);  // mss = 2^(number of bits in the offset)
    let mut pa: u32 = 0;
    let mut bit_mask: u32 = 0;
    for i in 0..power_of2 - 2 {  // we only want to mask the bits up to the ss
        bit_mask += 2u32.pow(i); // turning on bits in the mask value
    }
    let offset: u32 = va & bit_mask; // the expression on the left = va but with the 2 highest order bits set to 0 which is the same as the offset

    if ss == 0 { // code ss
        pa = calculate_answer(segments[0], mss, offset);
        show_solution_va_to_pa_hex(segments[0], ss, mss, bit_mask, offset, va, pa, power_of2);
    }
    else if ss == 1 { // heap ss
        pa = calculate_answer(segments[1], mss, offset);
        show_solution_va_to_pa_hex(segments[1], ss, mss, bit_mask, offset, va, pa, power_of2);
    }
    else if ss == 3 { // stack ss
        pa = calculate_answer(segments[2], mss, offset);
        show_solution_va_to_pa_hex(segments[2], ss, mss, bit_mask, offset, va, pa, power_of2);
    }
    else if ss != 0 && ss != 1 && ss != 3 {
        // if so then print error message and exit. --bug
        println!("error. segment selector doesnt represent any of the implemented segments. it equals {}", ss);
        println!("exiting program.");
        exit(-1);
    }
   
    
    println!("type your answer in hexadecimal and press ctrl + d or command + d. `0x` will be automatically parsed out of your answer string.");

    println!("{:?}",pa);
    /*let user_input = io::stdin();
    let mut answer: u32 = 0;
    for a_line in user_input.lock().lines() {
        answer = a_line.unwrap().parse::<u32>().unwrap();
    }
    if answer == pa {
        println!("congratulations!");
    }*/
}

// shows the `student` the steps to solving the 
pub fn show_solution_va_to_pa_hex(seg: Segment, ss: u32, mss: u32, mask: u32, offset: u32, va: u32, pa: u32, power_of2: u32) {
    println!("step 1: convert 0x{:x} to binary", va);

    print!("{:#x} = ", va);
    lib_fns::print_leading_zeros(offset, power_of2);
    println!("{:b}", va);
    io::stdout().flush().unwrap();  // ensure our output is flushed entirely, as we are not using the _println_ macro.

    println!("there are great youtube guides on shortcuts for converting to binary by hand.");
    println!("step 2: note the virtual address space size (in bits) and separate the segment selector from the offset portion of the binary.");
    if ss != 3 {
        println!("0{} {:b}", ss, offset);
    }
    else {
        println!("{} {:b}", ss, va);
    }
    println!("-- -----------------");
    println!("ss      offset\n");
    println!("discard the segment selector bits from your calculation of offset.");
}

// this function calculates the answer to the given problema and returns the result
// based on the following equation: pa = (-1)*(gn)(mss) + base + offset
// gn = grows negative. offset = the value of the virtual address shifted left 2 bits.
// (the value of the va neglecting the final two bits)
pub fn calculate_answer(seg: Segment, mss: u32, offset: u32) -> u32 {
    ((-1) * (seg.grows_negative as i32) * (mss as i32) + (seg.base as i32 * 1024) + offset as i32) as u32
}

// generates a problem prompting for a va to pa translation
// returns the generated result
pub fn get_rand_va(va_pow_2: u32, segments: Vec<Segment>) -> u32 {
    let mut rng = rand::thread_rng();
    let mut r: u32 = rng.gen_range(100, 2u32.pow(va_pow_2) + 1);
    let mut fresh_ss = r >> (va_pow_2 - 2);

    // ensure that the ss mimics the the code, stack or heap segment numbers.
    while !valid_va(r, fresh_ss, segments.clone()) || !(r >> (va_pow_2 - 2) == 0 || r >> (va_pow_2 - 2) == 1 || r >> (va_pow_2 - 2) == 3){

        r = rng.gen_range(100, 2u32.pow(va_pow_2)); // exclusive on the end so this works.

        fresh_ss = r >> (va_pow_2 - 2);

        // println!("{} <- r", r);
        //println!("r: {} fresh_ss {} pow: {} code base: {} code size {} heap base {} heap size {} stack base: {} stack size: {}", r, fresh_ss, va_pow_2, segments[0].base, segments[0].size, segments[1].base, segments[1].size, segments[2].base, segments[2].size);
    }
    r
}
// returns true if the address selected is within the scope/range of some segment.
pub fn valid_va(num: u32, fresh_ss: u32, segments: Vec<Segment>) -> bool {
    let mut enum_ss: SegName = SegName::Code;
    if fresh_ss == 0 {
        enum_ss = SegName::Code;
    }
    else if fresh_ss == 1 {
        enum_ss = SegName::Heap;
    }
    else if fresh_ss == 3 {
        enum_ss = SegName::Stack;
    }
    /*println!("{}", enum_ss as u32);
    println!("{}", SegName::Stack as u32);*/

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

// SAVED COMMENTED OUT CODE: 
    // let mut sum_segment_size: f32 = 0.0;
    //let mut index: u32 = 0;
    //while sum_segment_size != (vas as f32 / 4.0) {  // for the sake of the example/question, we want
    /*sum_segment_size = 0.0;
    for index in 0..segments.len() {
        // Seed the rng with a lower bound of 1 and an upper of 10*vas / 4.0.
        // Cast this value as u32 to remove the trailing decimal places
        // Finally, cast back to a f32 and divide by 10 to remove the effects of multiplying by 10.
        segments[index].size = (((rng.gen_range(1.0, (vas as f32 / 4.0) as f32) * 10.0) as u32) as f32) / 10.0;
        sum_segment_size += segments[index].size;  // track the sum of the _different_ sizes.
    }*/
    //}
    // testing purposes.
    /*println!("Sum: {}", sum_segment_size);
    println!("vas / 4.0: {}", vas as f32 / 4.0);
    for seg in segments {
        println!("{} size: {}", SEG_NAMES[seg.name as usize], seg.size);
    }*/
    //println!("checking, {}", ((2u32.pow(power_of2) - (segments[2].size as u32 * 1024)) / 1024));
    //println!("max, {}", ((2u32.pow(power_of2) - 1) / 1024));

/*println!("Hello {:5}!", "x");
    println!("Hello {:1$}!", "x", 5);
    println!("Hello {1:0$}!", 5, "x");
    println!("Hello {:0>width$}!", "x", width = 5);  // 0 is the fill character and < is the allignment
    println!("{:width$}!", "x", width = 5);*/
