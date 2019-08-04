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
*/

/*  Definitions:
vas = size of virtual address space * 1024 bytes (i.e. K)
va = virtual address
pm = size of physical memory * 1024 bytes (i.e. K)
pa = physical address (answer)
size of the vas = 2^power_of_2
ss = segment selector (or segment number)
mss = maximum segment size  */

/*
General flow:
1) Generate memory layout
2) Boot menu
3) Display question options to user
   0) VA to PA problem (without a guaranteed call to malloc)
   1) VA to PA problem with malloc call
   2) Stack portion problem
   9) Random problem
4) force user to attempt to solve the problem
    if correct --congratulate!
5) User communicates:
    if they want to see steps
    if they want to return to the main menu (go back)
6) If 0 --show steps and re-print the same menu.
   If 1 --return to the menu
*/

// import necessary libraries!
extern crate rand;
use crate::calculations;
use crate::lib_fns;
use lazy_static::*;
use rand::Rng;
use std::fmt::Write;
use std::io;
use std::path::{Path, PathBuf};
use std::process::exit;
use std::sync::Mutex;
// use std::sync::atomic::{AtomicUsize};

// rocket imports
use rocket::http::RawStr;
use rocket::request::Form;
use rocket::response::NamedFile;
use rocket_contrib::templates::Template;
// use rocket::response::Redirect;

// global constant representing the question choice from the first HTML form
lazy_static! {
    static ref Q_CHOICE: Mutex<QuestionChoice> = Mutex::new(Init);  // setting the choice equal to some initial form (not 0, 1, 2 yet).
}

// enumeration type for the question choice global
#[derive(Debug, Clone, Copy)]
enum QuestionChoice {
    Init, // empty choice (choice has not been made yet).
    Zero, // va_to_pa no malloc question
    One,  // va_to_pa malloc question
    Two,  // stack portion question
}
use QuestionChoice::*;

// struct for inserting templates into HTML files (and hbs files)
#[derive(Debug, Serialize, Deserialize, FromForm)]
struct QuestionSolutionInfo {
    question_prompt: String,
    question_solution: String,
}

// templates to be inserted into HTML/HBS files
#[derive(Serialize)]
pub struct TemplateContext {
    query: String,
    items: QuestionSolutionInfo,
    parent: &'static str,
}

// request from server
#[derive(FromForm, Debug)]
pub struct Request {
    term: String,
}

// data form (user entries)
#[derive(FromForm, Debug)]
pub struct Request2 {
    solution: String,  // needs to be named solution
}

// arbitrary path match
#[get("/static/<file..>")]
pub fn file(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

// index page
#[get("/", rank = 1)]
pub fn index() -> io::Result<NamedFile> {
    NamedFile::open("static/index.html")
}

// user's first entry = 0
#[get("/first?question_format=0", rank = 2)]
pub fn q_format_0() -> io::Result<NamedFile> {
    let mut question_choice = Q_CHOICE.lock().unwrap();
    *question_choice = Zero;
    NamedFile::open("static/va_to_pa_format.html")
}

// user's first entry = 1
#[get("/first?question_format=1", rank = 3)]
pub fn q_format_1() -> io::Result<NamedFile> {
    let mut question_choice = Q_CHOICE.lock().unwrap();
    *question_choice = One;
    NamedFile::open("static/va_to_pa_format.html")
}

// user's first entry = 2
#[get("/first?question_format=2", rank = 4)]
pub fn q_format_2() -> io::Result<NamedFile> {
    let mut question_choice = Q_CHOICE.lock().unwrap();
    *question_choice = Two;
    NamedFile::open("static/stack_format.html")
}

/*#[get("/search/<term>")]
pub fn response(term: &RawStr) -> String {
    format!("You typed in {}.", term)
}*/

// computes data to be printed and forwards it to oncoming pages.
#[post("/search", data = "<data>", rank = 1)]
pub fn setup(data: Form<Request>) -> Template {
    // (vas, power_of2, segments)

    // generate the segmented memory model for the environment
    // we only need to do this once here.
    let res_tuple = generate_segmented_memory_layout();
    let vas = res_tuple.0;
    let power_of2 = res_tuple.1;
    let segments = res_tuple.2; // name the tuple elements some relevant names.
    let mut to_print = print_layout(vas, vas * 2, power_of2, segments.clone());
    let to_print2;

    // format of the question (question and answer format specifier).
    let format_choice: i8 = match (&data.term).trim().parse::<i8>() {
        Ok(k) => k,
        Err(_) => {
            println!("Error. Invalid input. Terminating program.\n");
            -1
        }
    };
    if format_choice == -1 {
        calculations::error();
    }

    let question_choice = Q_CHOICE.lock().unwrap();
    println!("question choice is  {:?}", *question_choice);
    println!("format choice (page 2 form) is  {:?}", format_choice);
    let format_specifiers = fetch_format_specifiers(format_choice);

    // va to pa !malloc problem
    match *question_choice {
        Zero => {
            // fetch a random va
            let va: u32 = calculations::get_rand_va(power_of2, segments.clone(), false);
            // calculate offset:
            let ss: u32 = va >> (power_of2 - 2);
            // MSS = 2^number of bits in the offset = number of total bits - 2 (because SS = 2 bits)
            let mss: u32 = 2u32.pow(power_of2 - 2); // MSS = 2^(number of bits in the offset)
                                                    // calculate offset
            let mut bit_mask: u32 = 0;
            for i in 0..power_of2 - 2 {
                // we only want to mask the bits up to the ss
                bit_mask += 2u32.pow(i); // turning on bits in the mask value
            }
            let offset: u32 = va & bit_mask; // the expression on the left = va but with the 2 highest order bits set to 0 which is the same as the offset

            // get the strings to print to the web page.
            let func_result = print_question_va_to_pa(va, format_choice, false); // returns a tuple of form (i8, i8, String)
            to_print = to_print + &func_result.2;
            to_print = to_print + &calculations::print_answer_instructions(func_result.1);
            match ss {
                3 => {
                    // stack ss
                    // perform the math necessary to solve the given question
                    // the physical address (answer to the question).
                    let pa_ans = calculations::calculate_answer(segments[2], mss, offset);
                    to_print2 = calculations::show_solution_va_to_pa(
                        segments[2],
                        ss,
                        offset,
                        va,
                        pa_ans,
                        power_of2,
                        format_specifiers,
                    );
                }
                0 | 1 => {
                    // code, heap
                    // perform the math necessary to solve the given question
                    // the physical address (answer to the question).
                    let pa_ans = calculations::calculate_answer(segments[ss as usize], mss, offset);
                    to_print2 = calculations::show_solution_va_to_pa(
                        segments[ss as usize],
                        ss,
                        offset,
                        va,
                        pa_ans,
                        power_of2,
                        format_specifiers,
                    );
                }
                _ => {
                    // if so then print error message and exit. --BUG
                    println!("Error. Segment selector doesnt represent any of the implemented segments. It equals {}", ss);
                    println!("Exiting program.");
                    exit(-1);
                }
            }
            println!("sol: {}", to_print2);
            Template::render(
                "result",
                &TemplateContext {
                    query: "0".to_string(),
                    items: QuestionSolutionInfo {
                        question_prompt: to_print,
                        question_solution: to_print2,
                    },
                    parent: "index",
                },
            )
        }
        One => {
            // malloc problem
            // fetch a random va
            let va: u32 = calculations::get_rand_va(power_of2, segments.clone(), true);
            // calculate offset:
            let ss: u32 = va >> (power_of2 - 2);
            // MSS = 2^number of bits in the offset = number of total bits - 2 (because SS = 2 bits)
            let mss: u32 = 2u32.pow(power_of2 - 2); // MSS = 2^(number of bits in the offset)
                                                    // calculate offset
            let mut bit_mask: u32 = 0;
            for i in 0..power_of2 - 2 {
                // we only want to mask the bits up to the ss
                bit_mask += 2u32.pow(i); // turning on bits in the mask value
            }
            let offset: u32 = va & bit_mask; // the expression on the left = va but with the 2 highest order bits set to 0 which is the same as the offset

            // get the strings to print to the web page.
            let func_result = print_question_va_to_pa(va, format_choice, true); // returns a tuple of form (i8, i8, String)
            to_print = to_print + &func_result.2;
            to_print = to_print + &calculations::print_answer_instructions(func_result.1);
            match ss {
                3 => {
                    // stack ss
                    // perform the math necessary to solve the given question
                    // the physical address (answer to the question).
                    let pa_ans = calculations::calculate_answer(segments[2], mss, offset);
                    to_print2 = calculations::show_solution_va_to_pa(
                        segments[2],
                        ss,
                        offset,
                        va,
                        pa_ans,
                        power_of2,
                        format_specifiers,
                    );
                }
                0 | 1 => {
                    // code, heap
                    // perform the math necessary to solve the given question
                    // the physical address (answer to the question).
                    let pa_ans = calculations::calculate_answer(segments[ss as usize], mss, offset);
                    to_print2 = calculations::show_solution_va_to_pa(
                        segments[ss as usize],
                        ss,
                        offset,
                        va,
                        pa_ans,
                        power_of2,
                        format_specifiers,
                    );
                }
                _ => {
                    // if so then print error message and exit. --BUG
                    println!("Error. Segment selector doesnt represent any of the implemented segments. It equals {}", ss);
                    println!("Exiting program.");
                    exit(-1);
                }
            }

            Template::render(
                "result",
                &TemplateContext {
                    query: "0".to_string(),
                    items: QuestionSolutionInfo {
                        question_prompt: to_print,
                        question_solution: to_print2,
                    },
                    parent: "index",
                },
            )
        }
        Two => {
            // stack problem
            // generate percentage;
            let mut rng = rand::thread_rng(); // seed the rng
            let quarters = [100.0, 25.0, 75.0, 50.0, 0.0];
            let rando = rng.gen_range(0, quarters.len());
            let percent: f32 = quarters[rando];

            // get the strings to print to the web page.
            let func_result = print_question_stack_percentage(percent as u32, format_choice);
            to_print = to_print + &func_result.1;
            to_print = to_print + &calculations::print_answer_instructions(func_result.0);

            // MSS = 2^number of bits in the offset = number of total bits - 2 (because SS = 2 bits)
            let mss: u32 = 2u32.pow(power_of2 - 2); // MSS = 2^(number of bits in the offset)
                                                    // returns a tuple, the 0th position has the virtual address answer
            let tuple = calculations::calculate_answer_stack_percentage(
                segments[2],
                percent,
                mss,
                power_of2,
            );
            let va_ans = tuple.0;
            let offset = tuple.1;
            to_print2 = calculations::show_solution_stack_va(
                segments[2],
                offset,
                va_ans,
                power_of2,
                percent,
                format_specifiers.1,
            );
            Template::render(
                "result",
                &TemplateContext {
                    query: "0".to_string(),
                    items: QuestionSolutionInfo {
                        question_prompt: to_print,
                        question_solution: to_print2,
                    },
                    parent: "index",
                },
            )
        }

        _ => {
            exit(-1);
        }
    }
}

// shows the solution
#[post("/showsteps", data = "<data>")]
pub fn solution(data: Form<Request2>) -> Template {
    println!("{:?}", data.solution);
    let return_value = QuestionSolutionInfo {
        question_prompt: "null".to_string(),
        question_solution: (&data.solution).to_string(),
    };
    Template::render(
        "solution",
        &TemplateContext {
            query: "0".to_string(),
            items: return_value,
            parent: "index",
        },
    )
}

// function for fetching the format specifiers given a question/answer format user choice
pub fn fetch_format_specifiers(format_choice: i8) -> (i8, i8) {
    let qformat = match format_choice {
        0 | 1 | 2 => 16,
        3 | 4 | 5 => 2,
        6 | 7 | 8 => 10,
        _ => -1,
    };
    let aformat = match format_choice {
        0 | 3 | 6 => 16,
        1 | 4 | 7 => 2,
        2 | 5 | 8 => 10,
        _ => -1,
    };
    (qformat, aformat)
}

// this function calculates the bounds of the address space and generates a segmented memory
// model for the code heap and stack sections.
pub fn generate_segmented_memory_layout() -> (u32, u32, Vec<calculations::Segment>) {
    // calculate vas
    let vas: u32 =
        lib_fns::rand_power_of_2(lib_fns::rand_even(14, 65), lib_fns::rand_even(65, 256 + 1));

    // calculate the number of bits in the vas
    let power_of2: u32 = lib_fns::num_bits_reqd(vas * 1024);

    // initialize segment structs. the stack grows down.
    let code_segment = calculations::Segment {
        name: calculations::SegName::Code,
        base: 0,
        size: 0.0,
        grows_negative: 0,
    };
    let heap_segment = calculations::Segment {
        name: calculations::SegName::Heap,
        base: 0,
        size: 0.0,
        grows_negative: 0,
    };
    let stack_segment = calculations::Segment {
        name: calculations::SegName::Stack,
        base: 0,
        size: 0.0,
        grows_negative: 1,
    };

    // store these newly created segment types in a vector (so we can add more of them later _if_ we want)
    let mut segments: Vec<calculations::Segment> = Vec::new();
    segments.push(code_segment);
    segments.push(heap_segment);
    segments.push(stack_segment);
    segments = calculations::are_conflicting(power_of2, segments.clone());
    (vas, power_of2, segments)
}

// prints the generated segmented memory model
// text taken with permission from Mark Morissey's slides
pub fn print_layout(
    vas: u32,
    pm: u32,
    power_of2: u32,
    segments: Vec<calculations::Segment>,
) -> String {
    let mut to_print = String::new();
    writeln!(&mut to_print).unwrap();
    writeln!(&mut to_print,"Assume a {}KB virtual address space and a {}KB physical memory. Virtual addresses are {} bits and segmentation is being used.
    The segment information is:", vas, pm, power_of2).unwrap();
    // print ecessary info.
    writeln!(
        &mut to_print,
        "\t\tSegment Number\tBase\tSize\tGrowsNegative"
    )
    .unwrap();
    writeln!(
        &mut to_print,
        "\t\t{}\t00\t{}K\t{}K\t{}",
        calculations::SEG_NAMES[segments[0].name as usize],
        segments[0].base,
        segments[0].size,
        segments[0].grows_negative
    )
    .unwrap();
    writeln!(
        &mut to_print,
        "\t\t{}\t01\t{}K\t{}K\t{}",
        calculations::SEG_NAMES[segments[1].name as usize],
        segments[1].base,
        segments[1].size,
        segments[1].grows_negative
    )
    .unwrap();
    writeln!(
        &mut to_print,
        "\t\t{}\t11\t{}K\t{}K\t{}",
        calculations::SEG_NAMES[segments[2].name as usize],
        segments[2].base,
        segments[2].size,
        segments[2].grows_negative
    )
    .unwrap();
    to_print
}

// takes a format flag passed from the client and prints the question returning a format specifier (u32 flag).
// question text taken with permission from Mark Morissey's slides
pub fn print_question_stack_percentage(percent: u32, question_format: i8) -> (i8, String) {
    let mut to_print = String::new();
    let aformat = match question_format {
        0 => 16,
        1 => 2,
        2 => 10,
        _ => -1,
    };
    if aformat == -1 {
        calculations::error();
    }
    match aformat {
        16 => {
            writeln!(
                &mut to_print,
                "What virtual address, in hexadecimal, is {}% into the stack??",
                percent
            )
            .unwrap();
        }
        2 => {
            writeln!(
                &mut to_print,
                "What virtual address, in binary, is {}% into the stack??",
                percent
            )
            .unwrap();
        }
        10 => {
            writeln!(
                &mut to_print,
                "What virtual address, in decimal, is {}% into the stack??",
                percent
            )
            .unwrap();
        }
        _ => {
            writeln!(&mut to_print, "Unexpected error. Exiting").unwrap();
            calculations::error();
        }
    }
    (aformat, to_print)
}

pub fn print_question_va_to_pa(va: u32, format_flag: i8, malloc: bool) -> (i8, i8, String) {
    let mut to_print = String::new();
    let qformat = match format_flag {
        0 | 1 | 2 => 16,
        3 | 4 | 5 => 2,
        6 | 7 | 8 => 10,
        _ => -1,
    };
    let aformat = match format_flag {
        0 | 3 | 6 => 16,
        1 | 4 | 7 => 2,
        2 | 5 | 8 => 10,
        _ => -1,
    };
    if aformat == -1 || qformat == -1 {
        calculations::error();
    }
    if !malloc {
        match qformat {
            16 => {
                writeln!(
                    &mut to_print,
                    "Virtual Address {:#X} refers to what physical address (in base {})?",
                    va, aformat
                )
                .unwrap();
            }

            2 => {
                writeln!(
                    &mut to_print,
                    "Virtual Address {:b} refers to what physical address (in base {})?",
                    va, aformat
                )
                .unwrap();
            }
            10 => {
                writeln!(
                    &mut to_print,
                    "Virtual Address {} (base 10) refers to what physical address (in base {})?",
                    va, aformat
                )
                .unwrap();
            }
            _ => {
                writeln!(&mut to_print, "Unexpected error. Exiting").unwrap();
                calculations::error();
            }
        }
    } else {
        match qformat {
            16 => {
                writeln!(&mut to_print,"A call to malloc returns a virtual address of {:#X}. What is the physical address (in base {}) of this virtual address?"
                   , va, aformat).unwrap();
            }
            2 => {
                writeln!(&mut to_print,"A call to malloc returns a virtual address of {:b}. What is the physical address (in base {}) of this virtual address?"
                   , va, aformat).unwrap();
            }
            10 => {
                writeln!(&mut to_print,"A call to malloc returns a virtual address of {} (base 10). What is the physical address (in base {}) of this virtual address?"
                   , va, aformat).unwrap();
            }
            _ => {
                writeln!(&mut to_print, "Unexpected error. Exiting").unwrap();
                calculations::error();
            }
        }
    }
    (qformat, aformat, to_print)
}

/*
We need a way of telling each of the functions
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

// function for determining the format of the question --helps program flow determine the q/a format
fn _choose_format(question_flag: u8) -> (i8, String) {
    let mut to_print = String::new();
    let mut choice: i8;
    //let to_print:String ;
    loop {
        // clear_screen();
        let mut input_string = String::new();
        match question_flag {
            0 => {
                writeln!(&mut to_print, "\nChoose format of desired question\n").unwrap();
                writeln!(&mut to_print, "OPTION\t\tPROBLEM TYPE\n").unwrap();
                writeln!(&mut to_print, "0\u{29}\t\t--va in hex to pa in hex").unwrap();
                writeln!(&mut to_print, "1\u{29}\t\t--va in hex to pa in binary").unwrap();
                writeln!(&mut to_print, "2\u{29}\t\t--va in hex to pa in decimal").unwrap();
                writeln!(&mut to_print, "3\u{29}\t\t--va in binary to pa in hex").unwrap();
                writeln!(&mut to_print, "4\u{29}\t\t--va in binary to pa in binary").unwrap();
                writeln!(&mut to_print, "5\u{29}\t\t--va in binary to pa in decimal").unwrap();
                writeln!(&mut to_print, "6\u{29}\t\t--va in decimal to pa in hex").unwrap();
                writeln!(&mut to_print, "7\u{29}\t\t--va in decimal to pa in binary").unwrap();
                writeln!(&mut to_print, "8\u{29}\t\t--va in decimal to pa in decimal").unwrap();
                writeln!(&mut to_print, "9\u{29}\t\tRandom option").unwrap();
            }
            1 => {
                writeln!(&mut to_print, "Choose format of desired question\n").unwrap();
                writeln!(&mut to_print, "OPTION\t\tPROBLEM TYPE").unwrap();
                writeln!(&mut to_print, "0\u{29}\t\t--answer in hex").unwrap();
                writeln!(&mut to_print, "1\u{29}\t\t--answer in binary").unwrap();
                writeln!(&mut to_print, "2\u{29}\t\t--answer in decimal").unwrap();
                writeln!(&mut to_print, "9\u{29}\t\t--random question").unwrap();
            }
            _ => {
                writeln!(
                    &mut to_print,
                    "Unexpected Fatal error in question format function. Exiting."
                )
                .unwrap();
                exit(-1);
            }
        }
        match io::stdin().read_line(&mut input_string) {
            Ok(_) => {}
            Err(_) => {
                continue;
            }
        }
        choice = match input_string.trim().parse::<i8>() {
            Ok(k) => k,
            Err(_) => {
                writeln!(
                    &mut to_print,
                    "Error. Invalid input --not an integer. Please try again."
                )
                .unwrap();
                -1
            }
        };
        writeln!(&mut to_print).unwrap();
        if choice == -1 {
            continue;
        } else {
            match choice {
                0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 => {
                    break;
                }
                9 => {
                    if question_flag == 0 {
                        let mut rng = rand::thread_rng(); // seed the rng
                        choice = rng.gen_range(0, 9);
                    } else if question_flag == 1 {
                        let mut rng = rand::thread_rng(); // seed the rng
                        choice = rng.gen_range(0, 3);
                    }
                    break;
                }
                _ => {
                    writeln!(
                        &mut to_print,
                        "Please enter one of the digits corresponding to an option on screen"
                    )
                    .unwrap();
                    continue;
                }
            }
        }
    }
    (choice, to_print)
}

// function useful for clearing the output buffer
fn _clear_screen() -> String {
    let mut to_print = String::new();
    for _i in 0..50 {
        writeln!(&mut to_print).unwrap();
    }
    to_print
}
