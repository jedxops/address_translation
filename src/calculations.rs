// Copyright © 2019 Jeff Austin, Kamakshi Nagar
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

// import necessary crates!
// extern crate rand;
use crate::lib_fns;
use rand::Rng;
use std::fmt::Write as OtherWrite;
use std::io;
use std::io::Write;
use std::process::exit; // need flush() method.

#[derive(Copy, Clone)]
pub struct Segment {
    // segment information packaged.
    pub name: SegName, // segment name
    pub base: u32,
    pub size: f32,
    pub grows_negative: u32, // default is false the stack is the only one
                             // that grows negative so this way of structuring things is practical.
}

// used for segment name identification
#[derive(Copy, Clone)]
pub enum SegName {
    Code,  // as of right now, we are only concerned with these three segments.
    Heap,  // but this code is structured so that more segments should be able to
    Stack, // be added without too much difficulty
}

// array of segment names for printing and comparison purposes.
pub static SEG_NAMES: [&str; 3] = ["Code", "Heap", "Stack"];

// shows the `student` how to solve the stack to va problem
pub fn show_solution_stack_va(
    seg: Segment,
    offset: u32,
    va: u32,
    power_of2: u32,
    percent: f32,
    aformate: i8,
) -> String {
    let mut to_print = String::new();
    writeln!(&mut to_print,"\n--Note that the stack base and size are parameters in Physical Memory --not Virtual Memory.").unwrap();
    writeln!(&mut to_print,"The problem prompts a translation from PA to VA --the address corresponding to the specified portion through the stack as a virtual address").unwrap();
    writeln!(&mut to_print,"Therefore, calculating the percent of the stack size and subtracting it from the stack base yields an incorrect translation.\n").unwrap();
    writeln!(
        &mut to_print,
        "Drawing a picture helps to understand this non-intuitive math."
    )
    .unwrap();

    writeln!(
        &mut to_print,
        "\nStep 1: Calculate {}% into the stack _size_ in K (as a multiple of 1024)",
        percent
    )
    .unwrap();
    writeln!(
        &mut to_print,
        "{}% * {}K = ({} * {})K = {}K\n",
        percent,
        seg.size,
        percent / 100.0,
        seg.size,
        seg.size * (percent / 100.0)
    )
    .unwrap();

    writeln!(
        &mut to_print,
        "Step 2: Calculate the maximum segment size in relation to the stack's base"
    )
    .unwrap();
    writeln!(
        &mut to_print,
        "Remember --the stack grows downwards from its base.\n"
    )
    .unwrap();
    writeln!(
        &mut to_print,
        "MSS = 2^(number of bits in the offset) = 2^{} = {} bytes",
        power_of2 - 2,
        2u32.pow(power_of2 - 2)
    )
    .unwrap();
    writeln!(&mut to_print,"=> If the stack segment were to use up its MSS, it would grow downwards to {}K - {}K = {}K\n",
            seg.base, 2u32.pow(power_of2 -2) / 1024, seg.base - 2u32.pow(power_of2 -2) / 1024).unwrap();

    writeln!(&mut to_print, "Step 3: Calculate the offset").unwrap();
    writeln!(&mut to_print,"Subtract the address of the maximum segment size from the stack base minus the calculated portion of the stack size in K:").unwrap();
    writeln!(
        &mut to_print,
        "({}K - {}K) - {}K = {}K - {}K = {}K",
        seg.base,
        seg.size * (percent / 100.0),
        seg.base - 2u32.pow(power_of2 - 2) / 1024,
        seg.base as f32 - seg.size * (percent / 100.0),
        seg.base - 2u32.pow(power_of2 - 2) / 1024,
        offset
    )
    .unwrap();
    writeln!(&mut to_print,"This is the _offset_ portion of the binary number to be constructed for the virtual address.\n").unwrap();

    writeln!(
        &mut to_print,
        "Step 4: Append the appropriate segment selector bits to the offset"
    )
    .unwrap();
    writeln!(&mut to_print,"The SS = the top two bits of the virtual address. If virtual addresses use up 17 bits, then the SS goes in the place").unwrap();
    writeln!(
        &mut to_print,
        "of the 15th and 16th bits, counting from zero.\n"
    )
    .unwrap();
    writeln!(&mut to_print,"Since this translation is to a stack VA, this means the segment selector bits for that address must be 11 already.").unwrap();
    writeln!(&mut to_print, "=> SS = Stack = 11").unwrap();
    writeln!(
        &mut to_print,
        "Append these two SS bits on to the highest portion of the offset binary:"
    )
    .unwrap();

    writeln!(&mut to_print, "  {:b}\t--SS bits", 3 << (power_of2 - 2)).unwrap();
    write!(&mut to_print, "+ ").unwrap();
    writeln!(
        &mut to_print,
        "{}",
        lib_fns::print_leading_zeros(offset * 1024, power_of2)
    )
    .unwrap();
    writeln!(&mut to_print, "{:b}\t--OFFSET", offset * 1024).unwrap();

    io::stdout().flush().unwrap(); // ensure our output is flushed entirely. print! doesnt print a line.

    write!(&mut to_print, "--").unwrap();
    write!(
        &mut to_print,
        "{}",
        lib_fns::print_hyphens(3 << (power_of2 - 2))
    )
    .unwrap();
    io::stdout().flush().unwrap();
    writeln!(
        &mut to_print,
        "  {:b}",
        offset * 1024 + (3 << (power_of2 - 2))
    )
    .unwrap();

    writeln!(
        &mut to_print,
        "{:horiz$}VA in bytes",
        " ",
        horiz = (lib_fns::num_bits_reqd(3 << (power_of2 - 2)) / 3) as usize
    )
    .unwrap();

    match aformate {
        16 => {
            writeln!(&mut to_print, "=> VA = {:#X} bytes", va).unwrap();
        }
        2 => {}
        10 => {
            writeln!(&mut to_print, "=> VA = {} bytes", va).unwrap();
        }
        _ => {
            error();
        }
    }
    writeln!(&mut to_print,"Check out youtube for shortcuts on converting to and from binary, decimal, and hexadecimal by hand.\n").unwrap();
    to_print
}

// shows the `student` the steps to solving the VA to PA problem
pub fn show_solution_va_to_pa(
    seg: Segment,
    ss: u32,
    offset: u32,
    va: u32,
    pa: u32,
    power_of2: u32,
    qaformate: (i8, i8),
) -> String {
    let mut to_print = String::new();
    match qaformate.0 {
        16 => {
            writeln!(
                &mut to_print,
                "\nStep 1: Convert virtual address {:#X} to binary",
                va
            )
            .unwrap();
            write!(&mut to_print, "{:#X} = ", va).unwrap();
        }
        2 => {
            writeln!(
                &mut to_print,
                "\nStep 1: Convert virtual address {:b} to binary",
                va
            )
            .unwrap();
            write!(&mut to_print, "{:b} = ", va).unwrap();
        }
        10 => {
            writeln!(
                &mut to_print,
                "\nStep 1: Convert virtual address {} to binary",
                va
            )
            .unwrap();
            write!(&mut to_print, "{} = ", va).unwrap();
        }
        _ => {
            error();
        }
    }
    // lib_fns::print_leading_zeros(va, power_of2);
    // print!("{:b} = ", va);
    writeln!(
        &mut to_print,
        "{}",
        lib_fns::print_readable_binary(va, power_of2)
    )
    .unwrap(); // I think this function is cool, visit the file to check it out.
    writeln!(&mut to_print).unwrap();
    io::stdout().flush().unwrap(); // ensure our output is flushed entirely, as we are not using the _println_ macro here

    writeln!(&mut to_print,"\nStep 2: Note the Virtual Address Space size (abbrv. VAS --measured in bits) and separate the Segment Selector (SS) from the Offset portion of the binary.").unwrap();
    writeln!(&mut to_print,"Remember --if the amount of bits in the Virtual Address Space differs from the amount of bits in the binary calculated, we must either").unwrap();
    writeln!(&mut to_print,"\n\ta\u{29} Pad the calculated binary number with zeros until the length of the binary equals the amount of bits in the VAS.").unwrap();
    writeln!(&mut to_print,"\n\tb\u{29} Trim the top bits of the calculated binary until the length of the binary equals the amount of bits in the VAS.\n").unwrap();
    writeln!(
        &mut to_print,
        "In this case, the Virtual Address Space size in bits is {}.",
        power_of2
    )
    .unwrap();
    writeln!(
        &mut to_print,
        "So, only the first {} bits of the calculated binary are considered.\n",
        power_of2
    )
    .unwrap();
    writeln!(
        &mut to_print,
        "The SS is always either 00, 01, or 11 => SS \u{3F5} \u{7B}{0, 1, 3\u{7D}}"
    )
    .unwrap();
    writeln!(
        &mut to_print,
        "\nDiscard the segment selector bits from the offset calculation.\n"
    )
    .unwrap();
    if ss != 3 {
        writeln!(&mut to_print, "0{} {:b}", ss, offset).unwrap();
    } else {
        writeln!(&mut to_print, "{:b} {:b}", ss, offset).unwrap();
    }
    write!(&mut to_print, "-- ").unwrap();
    //lib_fns::print_hyphens(offset);
    write!(&mut to_print, "{}", lib_fns::print_hyphens(offset)).unwrap();
    writeln!(
        &mut to_print,
        "SS {:x_axis$}OFFSET",
        " ",
        x_axis = ((lib_fns::num_bits_reqd(offset) / 2) / 2) as usize
    )
    .unwrap();

    writeln!(
        &mut to_print,
        "\nStep 3: Note the value of the Segment Selector and Offset bits:"
    )
    .unwrap();
    writeln!(&mut to_print, "00 ===> Code").unwrap();
    writeln!(&mut to_print, "01 ===> Heap").unwrap();
    writeln!(&mut to_print, "11 ===> Stack\n").unwrap();
    writeln!(&mut to_print, "Offset = {:b} = {} bytes\n", offset, offset).unwrap();

    writeln!(
        &mut to_print,
        "Step 4: Note: PA = (-1)*(GN)*(MSS) + base + offset\n"
    )
    .unwrap();
    writeln!(
        &mut to_print,
        "GN = `grows negative`. If SS = 11 => SS = Stack => GN = 1. Otherwise, GN = 0."
    )
    .unwrap();
    writeln!(
        &mut to_print,
        "MSS = `maximum segment size`. MSS = 2^(number of bits in the offset)"
    )
    .unwrap();
    writeln!(
        &mut to_print,
        "Base = the base of the segment, measured in bytes. Value provided in table."
    )
    .unwrap();
    writeln!(
        &mut to_print,
        "The offset has already been calculated: offset = {} bytes (base 10)",
        offset
    )
    .unwrap();

    writeln!(
        &mut to_print,
        "There are {} bits in the offset, so the MSS is 2^{} = {} bytes.",
        power_of2 - 2,
        power_of2 - 2,
        2u32.pow(power_of2 - 2)
    )
    .unwrap();
    if ss == 0 {
        writeln!(
            &mut to_print,
            "The SS = 00 (base 2) = 0 (base 10) => SS = Code Segment => GN = 0"
        )
        .unwrap();
        writeln!(
            &mut to_print,
            "PA (in bytes) = (-1)*(GN)*(MSS) + base + offset\n"
        )
        .unwrap();
        writeln!(
            &mut to_print,
            "=> PA = (-1)(0)(2^{}) + ({}K) + {} bytes",
            power_of2 - 2,
            seg.base,
            offset
        )
        .unwrap();
        writeln!(
            &mut to_print,
            "=> PA = (-1)(0)({}) + ({} * 1024) + {} bytes",
            2u32.pow(power_of2 - 2),
            seg.base,
            offset
        )
        .unwrap();
        writeln!(
            &mut to_print,
            "=> PA = 0 + ({}) + {} bytes",
            seg.base * 1024,
            offset
        )
        .unwrap();
        writeln!(
            &mut to_print,
            "=> PA = {} bytes",
            (seg.base * 1024) + offset
        )
        .unwrap();
        if pa != (seg.base * 1024) + offset {
            writeln!(&mut to_print, "Error. Conflicting calculations of pa").unwrap();
            exit(-1);
        }
    } else if ss == 1 {
        writeln!(
            &mut to_print,
            "The SS = 01 (base 2) = 1 (base 10) => SS = Heap Segment => GN = 0"
        )
        .unwrap();
        writeln!(
            &mut to_print,
            "PA (in bytes) = (-1)*(GN)*(MSS) + base + offset\n"
        )
        .unwrap();
        writeln!(
            &mut to_print,
            "=> PA = (-1)(0)(2^{}) + ({}K) + {} bytes",
            power_of2 - 2,
            seg.base,
            offset
        )
        .unwrap();
        writeln!(
            &mut to_print,
            "=> PA = (-1)(0)({}) + ({} * 1024) + {} bytes",
            2u32.pow(power_of2 - 2),
            seg.base,
            offset
        )
        .unwrap();
        writeln!(
            &mut to_print,
            "=> PA = 0 + ({}) + {} bytes",
            seg.base * 1024,
            offset
        )
        .unwrap();
        writeln!(
            &mut to_print,
            "=> PA = {} bytes",
            (seg.base * 1024) + offset
        )
        .unwrap();
        if pa != (seg.base * 1024) + offset {
            writeln!(&mut to_print, "Error. Conflicting calculations of pa").unwrap();
            exit(-1);
        }
    } else if ss == 3 {
        writeln!(
            &mut to_print,
            "The SS = 11 (base 2) = 3 (base 10) => SS = Stack Segment => GN = 1"
        )
        .unwrap();
        writeln!(
            &mut to_print,
            "PA (in bytes) = (-1)*(GN)*(MSS) + base + offset\n"
        )
        .unwrap();
        writeln!(
            &mut to_print,
            "=> PA = (-1)(1)(2^{}) + ({}K) + {} bytes",
            power_of2 - 2,
            seg.base,
            offset
        )
        .unwrap();
        writeln!(
            &mut to_print,
            "=> PA = (-1)(1)({}) + ({} * 1024) + {} bytes",
            2u32.pow(power_of2 - 2),
            seg.base,
            offset
        )
        .unwrap();
        writeln!(
            &mut to_print,
            "=> PA = (-{}) + ({}) + {} bytes",
            2u32.pow(power_of2 - 2),
            seg.base * 1024,
            offset
        )
        .unwrap();
        writeln!(
            &mut to_print,
            "=> PA = {} bytes",
            ((seg.base * 1024) + offset) - 2u32.pow(power_of2 - 2)
        )
        .unwrap();
        if pa != (seg.base * 1024) + offset - 2u32.pow(power_of2 - 2) {
            writeln!(&mut to_print, "Error. Conflicting calculations of pa").unwrap();
            exit(-1);
        }
    } else {
        error();
    }

    match qaformate.1 {
        16 => {
            writeln!(&mut to_print, "=> PA = {:#X} bytes", pa).unwrap();
        }
        2 => {
            writeln!(&mut to_print, "=> PA = {:b} bytes", pa).unwrap();
        }
        10 => {}
        _ => {
            error();
        }
    }
    writeln!(&mut to_print,"Check out youtube for shortcuts on converting to and from binary, decimal, and hexadecimal by hand.\n").unwrap();
    to_print
}

// this function calculates the answer to the stack percentage problem.
pub fn calculate_answer_stack_percentage(
    seg: Segment,
    percent: f32,
    mss: u32,
    power_of2: u32,
) -> (u32, u32) {
    if seg.size == 0.0 {
        return (3 << (power_of2 - 2), 0);
    }
    let midpoint = ((percent / 100.0) * seg.size) as f32; // in K
    let stack_max_seg_addr = seg.base as f32 - mss as f32 / 1024.0; // in K
    let offset = (seg.base as f32 - midpoint) - stack_max_seg_addr; // in K

    (
        (offset * 1024.0 + (3 << (power_of2 - 2)) as f32) as u32,
        offset as u32,
    ) // in bytes
}

#[test]
pub fn test_stack_percentage_calculation_v1() {
    let stak_seg = Segment {
        name: SegName::Stack,
        base: 64,
        size: 2.0,
        grows_negative: 1,
    };
    assert_eq!(
        114688,
        calculate_answer_stack_percentage(stak_seg, 0.0, 16384, 17).0
    );
}

#[test]
pub fn test_stack_percentage_calculation_v2() {
    let stak_seg = Segment {
        name: SegName::Stack,
        base: 64,
        size: 0.0,
        grows_negative: 1,
    };
    assert_eq!(
        196608,
        calculate_answer_stack_percentage(stak_seg, 0.0, 16384, 18).0
    );
}

// this function calculates the answer to the given problem and returns the result
// based on the following equation: PA = (-1)*(GN)(MSS) + base + offset
// GN = grows negative. offset = the value of the virtual address shifted left 2 bits.
// (the value of the va neglecting the final two bits)
#[allow(clippy::neg_multiply)] // --Massey suggestion to get rid of clippy warning.
                               // using (1 * (!1)) does not fix the issue, it results in -2 instead of -1
pub fn calculate_answer(seg: Segment, mss: u32, offset: u32) -> u32 {
    ((-1) * (seg.grows_negative as i32) * (mss as i32) + (seg.base as i32 * 1024) + offset as i32)
        as u32 // in bytes
}

#[test]
pub fn test_va_pa_calculation_v1() {
    let heap_seg = Segment {
        name: SegName::Stack,
        base: 128,
        size: 12.0,
        grows_negative: 0,
    };
    assert_eq!(132096, calculate_answer(heap_seg, 32768, 1024));
}

pub fn print_answer_instructions(aformat: i8) -> String {
    let mut to_print = String::new();
    match aformat {
        16 => {
            writeln!(
                &mut to_print,
                "Type your answer in hexadecimal format with or without the '0x'"
            )
            .unwrap();
        }
        2 => {
            writeln!(&mut to_print,"Type your answer in binary format with or without leading zeros then press enter and ctrl+d").unwrap();
        }
        10 => {
            writeln!(&mut to_print,"Type your answer in decimal format (base 10, no decimal points) then press enter and ctrl+d").unwrap();
        }
        _ => {
            writeln!(
                &mut to_print,
                "Error. Unexpected format specifier. Fatal error. Terminating program"
            )
            .unwrap();
            // for terminal error output
            println!("Error. Unexpected format specifier. Fatal error. Terminating program");
            exit(-1)
        }
    }
    to_print
}

// compares actual answer to user answer after printing the question
pub fn compare_answer(input: String, aformat: i8, pa: u32) -> String {
    let mut answer = input;
    let mut to_print = String::new();
    match aformat {
        16 => {
            writeln!(&mut to_print).unwrap();
            answer = answer.replace("x", ""); // replace all the characters that could possibly be taken as hex prefixes (like 0, x) with empty string
            answer = answer.replace("X", "");
            if lib_fns::_are_all_numeric(&answer, 16) {
                match lib_fns::_bn_to_b10(&answer.replace("0x", "").to_string(), 16) {
                    // use my library function to convert answer to base 10 (so we can measure it!)
                    Some(k) => {
                        if k as u32 == pa {
                            writeln!(&mut to_print, "Good.").unwrap();
                        } else {
                            writeln!(&mut to_print, "INCORRECT.\n").unwrap();
                            writeln!(
                                &mut to_print,
                                "your answer: {:#X} bytes\nactual: {:#X} bytes",
                                k, pa
                            )
                            .unwrap();
                        }
                    }
                    None => {
                        writeln!(&mut to_print, "INCORRECT.\n").unwrap();
                        writeln!(
                            &mut to_print,
                            "your answer: {} bytes\nactual: {:#X} bytes",
                            answer, pa
                        )
                        .unwrap();
                    }
                }
            } else {
                writeln!(&mut to_print, "INCORRECT.\n").unwrap();
                writeln!(
                    &mut to_print,
                    "your answer: {} bytes\nactual: {:#X} bytes",
                    answer, pa
                )
                .unwrap();
            }
        }
        2 => {
            writeln!(&mut to_print,"Type your answer in binary format with or without leading zeros then press enter and ctrl+d").unwrap();
            writeln!(&mut to_print).unwrap();
            answer = answer.replace("x", "");
            answer = answer.replace("X", ""); // replace all the characters that could possibly be taken as hex prefixes (like 0, x) with empty string
            if lib_fns::_are_all_numeric(&answer, 2) {
                // the second flag specifies the base which we define as `numeric`
                match lib_fns::_bn_to_b10(&answer.trim().to_string(), 2) {
                    Some(k) => {
                        if k as u32 == pa {
                            writeln!(&mut to_print, "Good.").unwrap();
                        } else {
                            writeln!(&mut to_print, "INCORRECT.\n").unwrap();
                            writeln!(
                                &mut to_print,
                                "your answer: {:b} bytes bytes\nactual: {:b} bytes bytes",
                                k, pa
                            )
                            .unwrap();
                        }
                    }
                    None => {
                        writeln!(&mut to_print, "INCORRECT.\n").unwrap();
                        writeln!(
                            &mut to_print,
                            "your answer: {} bytes\nactual: {:b} bytes",
                            answer, pa
                        )
                        .unwrap();
                    }
                }
            } else {
                writeln!(&mut to_print, "INCORRECT.\n").unwrap();
                writeln!(
                    &mut to_print,
                    "your answer: {} bytes\nactual: {:b} bytes",
                    answer, pa
                )
                .unwrap();
            }
        }
        10 => {
            writeln!(&mut to_print,"Type your answer in decimal format (base 10, no decimal points) then press enter and ctrl+d").unwrap();
            answer = answer.replace("x", "");
            answer = answer.replace("X", "");
            if lib_fns::_are_all_numeric(&answer, 10) {
                match lib_fns::_bn_to_b10(&answer.trim().to_string(), 10) {
                    Some(k) => {
                        if k as u32 == pa {
                            writeln!(&mut to_print, "Good.").unwrap();
                        } else {
                            writeln!(&mut to_print, "INCORRECT.\n").unwrap();
                            writeln!(
                                &mut to_print,
                                "your answer: {} bytes\nactual: {} bytes",
                                k, pa
                            )
                            .unwrap();
                        }
                    }
                    None => {
                        writeln!(&mut to_print, "INCORRECT.\n").unwrap();
                        writeln!(
                            &mut to_print,
                            "your answer: {} bytes\nactual: {} bytes",
                            answer, pa
                        )
                        .unwrap();
                    }
                }
            } else {
                writeln!(&mut to_print, "INCORRECT.\n").unwrap();
                writeln!(
                    &mut to_print,
                    "your answer: {} bytes\nactual: {} bytes",
                    answer, pa
                )
                .unwrap();
            }
        }
        _ => {
            writeln!(
                &mut to_print,
                "Error. Unexpected format specifier. Fatal error. Terminating program"
            )
            .unwrap();
            exit(-1)
        }
    }
    to_print
}

// generates a problem prompting for a VA to PA translation
// returns the generated result
pub fn get_rand_va(va_pow_2: u32, segments: Vec<Segment>, malloc: bool) -> u32 {
    let mut rng = rand::thread_rng();
    let mut r: u32 = rng.gen_range(100, 2u32.pow(va_pow_2) + 1); // exclusive with 2nd arg.
    let mut fresh_ss: u32 = 19; // the ss cant be 19 in this implementation
                                // ensure that the SS mimics the the code, stack or heap segment numbers.
    if !malloc {
        // if not the malloc problem
        while (fresh_ss != 0 && fresh_ss != 1 && fresh_ss != 3)
            || !valid_va(r, fresh_ss, segments.clone())
        {
            r = rng.gen_range(100, 2u32.pow(va_pow_2)); // exclusive on the end so this works.
            fresh_ss = r >> (va_pow_2 - 2);
        }
        r
    } else {
        // otherwise, we know we HAVE to return a value with a SS of 01
        while fresh_ss != 1 || !valid_va(r, fresh_ss, segments.clone()) {
            r = rng.gen_range(
                segments[1].base * 1024,
                (segments[1].base as f32 * 1024.0 + segments[1].size * 1024.0) as u32,
            ); // exclusive on the end so this works.
            fresh_ss = r >> (va_pow_2 - 2);
        }
        r
    }
}

// returns true if the address selected is within the scope/range of some segment.
pub fn valid_va(num: u32, fresh_ss: u32, segments: Vec<Segment>) -> bool {
    let mut enum_ss: SegName = SegName::Code;
    match fresh_ss {
        0 => enum_ss = SegName::Code,
        1 => enum_ss = SegName::Heap,
        3 => enum_ss = SegName::Stack,
        _ => {
            error();
        } // should never reach this case
    }
    for sg in segments {
        if sg.grows_negative == 1 {
            // if num is in the stack segment's range.
            if num <= sg.base * 1024
                && num as f32 >= (sg.base as f32 - sg.size) * 1024.0
                && sg.name as u32 == enum_ss as u32
            {
                return true;
            }
        } else if (num as f32 <= (sg.base as f32 + sg.size) * 1024.0)  // if the number is in some other range.
            && num >= sg.base * 1024
            && sg.name as u32 == enum_ss as u32
        {
            return true;
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
        out_of_bounds = false; // assume we are OK unless one of the conditions fails.

        for seg in 0..segments.len() {
            // we want the bases to be a number as a multiple of K (1024) so we divide by 1024
            // note: gen_range is exclusive with the upper bound.
            if SEG_NAMES[segments[seg].name as usize] == "Code" {
                segments[seg].base = rng.gen_range(0, (2u32.pow(power_of2 - 3)) / 1024);
                segments[seg].size = ((rng.gen_range(
                    1.0,
                    ((2u32.pow(power_of2 - 2) / 1024) - ((2u32.pow(power_of2 - 3)) / 1024)) as f32,
                ) * 10.0) as u32) as f32
                    / 10.0;
            } else if SEG_NAMES[segments[seg].name as usize] == "Heap" {
                segments[seg].base = rng.gen_range(
                    2u32.pow(power_of2 - 2) / 1024,
                    2u32.pow(power_of2 - 1) / 1024,
                );
                segments[seg].size = ((rng.gen_range(
                    1.0,
                    ((2u32.pow(power_of2 - 1) + 2u32.pow(power_of2 - 2)) / 1024
                        - 2u32.pow(power_of2 - 1) / 1024) as f32,
                ) * 10.0) as u32) as f32
                    / 10.0;
            } else if SEG_NAMES[segments[seg].name as usize] == "Stack" {
                segments[seg].base = rng.gen_range(
                    (2u32.pow(power_of2 - 1) + 2u32.pow(power_of2 - 2)) / 1024,
                    2u32.pow(power_of2) / 1024,
                );
                segments[seg].size = ((rng.gen_range(
                    1.0,
                    (((2u32.pow(power_of2 - 1) + 2u32.pow(power_of2 - 2)) / 1024)
                        - (2u32.pow(power_of2 - 1) / 1024)) as f32,
                ) * 10.0) as u32) as f32
                    / 10.0;
            }
        }

        // the following checks may or may not be necessary based on how we are seeding the base and size generations above.
        // idea to use _two_ for loops here and a reference type inspired by Bart Massey:
        for i in 0..segments.len() {
            for j in 0..segments.len() {
                // rustic-safe!
                if i == j {
                    continue;
                }

                let seg1 = &segments[i];
                let seg2 = &segments[j];

                if seg1.grows_negative == 1 {
                    if seg1.base as f32 - seg1.size < 0.0 {
                        // if this if statement is false then we have a useable stack base.
                        out_of_bounds = true;
                    }
                    // what this basically says:
                    // if the base of a non-stack segment lies between the stack base (bottom of the stack)
                    // and the stack base minus the stack size OR
                    // if the stretch of the non-stack segment lies between this range, then these two segments are
                    // conflicting with one another and therefore we must `roll` again. We have a non-valid set of segments.
                    else if (seg2.base <= seg1.base
                        && seg2.base as f32 >= seg1.base as f32 - seg1.size)
                        || (seg2.base as f32 + seg2.size >= seg1.base as f32 - seg1.size
                            && seg2.base <= seg1.base)
                        || (seg2.base as f32 + seg2.size <= seg1.base as f32
                            && seg2.base as f32 + seg2.size >= seg1.base as f32 - seg1.size)
                    {
                        conflicting = true;
                    }
                }
                // if either of the two non-stack segments grow into eachother or are based in one of each other's range they are conflicting.
                else if seg1.grows_negative == 0
                    && seg2.grows_negative == 0
                    && ((seg1.base >= seg2.base
                        && seg1.base as f32 <= seg2.base as f32 + seg2.size)
                        || (seg1.base <= seg2.base
                            && seg2.base as f32 <= seg1.base as f32 + seg1.size))
                {
                    conflicting = true;
                }
            }
        }
    }
    segments
}
