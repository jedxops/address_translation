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

    // using a trick for getting an even such as performing integer division and then returning
    // `num` multiplied by 2 does not work here because in the case of the lower bound being 1,
    // 1 / 2 would be zero, and we could end up with a value of 0 as a base/size. So we use modulo.

    loop { // looping forever
        let num: u32 = rng.gen_range(lower_bound, upper_bound);
        if num % 2 == 0 { // => the number is even
            return num;
        }
    }
}

        //rng = rand::thread_rng();
// return a random power of 2 (u32)
fn rand_power_of_2(lower: u32, upper: u32) -> u32 {
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
        if done == true && found == true {
            return num;
        }
        dummy = 2;
        found = false;
    }
    0
}

// generates a problem prompting for a VA to PA translation
fn va_to_pa(va_pow_2: u32) -> () { 
    let mut rng = rand::thread_rng();
    let hex = rng.gen_range(100, 2u32.pow(va_pow_2));
    println!("Virtual Address {:X} refers to what physical address (in base 10)?", hex);
}

//#[derive(Debug)]
struct Segment {
    name: SegName,
    base: u32,
    size: f32,
    grows_negative: bool,  // default is false the stack is the only one
                          // that grows negative so this way of structuring things is practical.
}

// used for segment name identification
enum SegName {
    Code,
    Heap,
    Stack,   // as of right now, we are only concerned with these three segments.
                    // but this code is structured so that more segments should be able to
                    // be added without much more difficulty
}

static segment_names: [&'static str;3] = ["Code", "Heap", "Stack"];

// this function prints the main menu for the problem at hand and services the menu
fn problem_setup() -> () {
    // ** vas = size of virtual address space * 1024 bytes (i.e. K)
    // ** pm = size of physical memory * 1024 bytes (i.e. K)
    // ** size of the vas = 2^power_of_2

    // calculate vas
    let vas: u32 = rand_power_of_2(rand_even(16, 65), rand_even(65, 256));
    // calculate the number of bits in the vas
    let mut power_of2: u32 = 1;
    let mut dummy: u32 = 1;  // assume at least 1 bits
    for i in 1..20 {  // i will start at 1 and go through 19 --we would need 18 bits to represent a 256K vas
        if dummy == (vas*1024) {  // not going to be true for first iteration
            power_of2 = i - 1;
            break;
        }
        dummy *= 2;
    }
    let pm: u32 = vas * 2;  // the pm needs to be at least double the vas

    let mut rng = rand::thread_rng();

    let mut code_segment = Segment { name: SegName::Code, base: 0, size: 0.0, grows_negative: false };

    let mut heap_segment = Segment { name: SegName::Heap, base:  0, size: 0.0, grows_negative: false };
    let mut stack_segment = Segment { name: SegName::Stack, base: 0, size: 0.0, grows_negative: true };
    // store these newly created segment types in a vector (so we can add more of them later _if_ we want)

    let mut segments: Vec<Segment> = Vec::new();

    segments.push(code_segment);
    segments.push(heap_segment);
    segments.push(stack_segment);

    let mut sum_segment_size: f32 = 0.0;
    //let mut index: u32 = 0;

    while sum_segment_size != (vas as f32/4.0) {

        for index in 0..segments.len() {  // use this for iterating over these segments
            segments[index].size = rng.gen_range(1.0, (vas as f32));
            sum_segment_size += segments[index].size;
            // println!("{}", seg);
            println!("name: {}", segment_names[((segments[index].name)) as usize]);
            //heap_size = (((rng.gen_range(1.0, (vas as f32/4.0) as f32) * 10.0) as u32 - 1) as f32) / 10.0;
            //stack_size = (((rng.gen_range(1.0, (vas as f32/4.0) as f32) * 10.0) as u32 - 1) as f32) / 10.0; 
        }
               // println!("size search");
    }
    println!("Sum: {}", sum_segment_size);
    // println!("code: {} heap {} stack {}", code_size, heap_size, stack_size);
/*
    let mut code_base: u32 = 0;
    let mut heap_base: u32 = 0;
    let mut bottom_of_stack: u32 = 0;
    let mut conflicting = true;
    let mut out_of_bounds = true;
    while conflicting || out_of_bounds {
        conflicting = true;
        out_of_bounds = true;

        code_base = rand_even(1, vas/2);
        heap_base = rand_even(1, vas/2);
        bottom_of_stack = rand_even(1, vas/2);
        // int vs int
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
        // println!("searching");
    }

    // print the basic information
    println!();
    println!("Assume a {}KB virtual address space and a {}KB physical memory. Virtual addresses are {} bits and segmentation is being used. The segment information is:\n", vas, pm, power_of2);

    println!("\t\tSegment Number\tBase\tSize\tGrowsNegative");
    println!("\t\tCode\t00\t{}K\t{}K\t0", code_base, code_size);
    println!("\t\tHeap\t01\t{}K\t{}K\t0", heap_base, heap_size);
    println!("\t\tStack\t11\t{}K\t{}K\t1", bottom_of_stack, stack_size);

    va_to_pa(power_of2);*/
}

// main function
fn main() {
    problem_setup();
}
