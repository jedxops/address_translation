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

/*
#[allow(clippy::type_complexity)]   // I want to preserve the trippy C-like syntax seen here in the
// array of functions definition
// array of functions for address translation questions = atqs
let mut atqs: Vec<fn(u32, u32, Vec<calculations::Segment>) -> (u32, u32, Vec<calculations::Segment>),> = Vec::new();
atqs.push(va_to_pa);
atqs.push(va_to_pa_malloc);
atqs.push(stack_va);
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
4) if a) --exit program. Else we print the memory layout and the problem
5) force user to attempt to solve the problem
    if correct --congratulate!
6) User communicates:
    0) if they want to see steps
    1) if they want to return to the main menu
    2) if they want to exit
7) If 0 --show steps and re-print the same menu.
   If 1 --return to previous menu
   If 2 --exit program
*/

// import necessary libraries!
extern crate rand;
use crate::calculations;
use crate::lib_fns;
use rand::Rng;
use std::fmt::Write;
use std::io;
use std::path::{Path, PathBuf};
use std::process::exit;
use lazy_static::*;
use std::sync::Mutex;
// use std::sync::atomic::{AtomicUsize};

// rocket imports
use rocket::request::Form;
use rocket_contrib::templates::Template;
use rocket::response::NamedFile;
use rocket::http::RawStr;
// use rocket::response::Redirect;

// global constant representing the question choice from the first HTML form
lazy_static! {
    static ref Q_CHOICE: Mutex<QuestionChoice> = Mutex::new(ChoiceInit);  // setting the choice equal to some initial form (not 0, 1, 2 yet).
}

// enumeration type for the question choice global
#[derive(Debug, Clone, Copy)]
enum QuestionChoice {
    ChoiceInit,     // empty choice (choice has not been made yet).
    Choice0,        // va_to_pa no malloc question
    Choice1,        // va_to_pa malloc question
    Choice2,        // stack portion question
}
use QuestionChoice::*;

#[derive(Debug, Serialize, Deserialize)]
struct QuestionSolutionInfo {
    question_prompt: String,
    question_solution: String,

}

#[derive(Serialize)]
pub struct TemplateContext {
    query: String,
    items: QuestionSolutionInfo,
    parent: &'static str,
}

#[derive(FromForm)]
pub struct Request {
    term: String,
}

#[get("/static/<file..>")]
pub fn file(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

#[get("/", rank=1)]
pub fn index() -> io::Result<NamedFile> {
    NamedFile::open("static/index.html")
}

// user's first entry = 0
#[get("/first?question_format=0", rank=2)]
pub fn q_format_0() -> io::Result<NamedFile> {
    // QUESTION_CHOICE = AtomicUsize::new(0);
    /*{
        GLOBAL_Q_CHOICE = 0;
    }*/
    let mut question_choice = Q_CHOICE.lock().unwrap();
    *question_choice = Choice0;
    NamedFile::open("static/va_to_pa_format.html")
}

// user's first entry = 1
#[get("/first?question_format=1", rank=3)]
pub fn q_format_1() -> io::Result<NamedFile> {
    // QUESTION_CHOICE = AtomicUsize::new(1);
    /*{
        GLOBAL_Q_CHOICE = 1;
    }*/
    let mut question_choice = Q_CHOICE.lock().unwrap();
    *question_choice = Choice1;
    NamedFile::open("static/va_to_pa_format.html")
}

// user's first entry = 2
#[get("/first?question_format=2", rank=4)]
pub fn q_format_2() -> io::Result<NamedFile> {
    // QUESTION_CHOICE = AtomicUsize::new(2);
    /*{
        GLOBAL_Q_CHOICE = 2;
    }*/
    let mut question_choice = Q_CHOICE.lock().unwrap();
    *question_choice = Choice2;
    NamedFile::open("static/stack_format.html")
}

#[get("/search/<term>")]
pub fn response(term: &RawStr) -> String {
    format!("You typed in {}.", term)
}

#[post("/search", data = "<data>", rank=1)]
pub fn setup(data: Form<Request>) -> Template {
    // (vas, power_of2, segments)

    // generate the segmented memory model for the environment
    // we only need to do this once here.
    let res_tuple = generate_segmented_memory_layout();
    let vas = res_tuple.0;
    let power_of2 = res_tuple.1;
    let segments = res_tuple.2;   // name the tuple elements some relevant names.
    let mut to_print = print_layout(vas, vas * 2, power_of2, segments.clone());
    let mut to_print2 = "Default".to_string();

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
        Choice0 => {
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
            let func_result = print_question_va_to_pa(va, format_choice, false);  // returns a tuple of form (i8, i8, String)
            to_print = to_print + &func_result.2;
            to_print = to_print + &calculations::print_answer_instructions(func_result.1);
            match ss {
                3 => {
                    // stack ss
                    // perform the math necessary to solve the given question
                    // the physical address (answer to the question).
                    let pa_ans = calculations::calculate_answer(segments[2], mss, offset);
                    to_print2 = calculations::show_solution_va_to_pa(segments[2], ss, offset, va, pa_ans, power_of2, format_specifiers);
                }
                0 | 1 => {
                    // code, heap
                    // perform the math necessary to solve the given question
                    // the physical address (answer to the question).
                    let pa_ans = calculations::calculate_answer(segments[ss as usize], mss, offset);
                    to_print2 = calculations::show_solution_va_to_pa(segments[ss as usize], ss, offset, va, pa_ans, power_of2, format_specifiers);
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
        Choice1 => {    // malloc problem
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
            let func_result = print_question_va_to_pa(va, format_choice, true);  // returns a tuple of form (i8, i8, String)
            to_print = to_print + &func_result.2;
            to_print = to_print + &calculations::print_answer_instructions(func_result.1);
            match ss {
                3 => {
                    // stack ss
                    // perform the math necessary to solve the given question
                    // the physical address (answer to the question).
                    let pa_ans = calculations::calculate_answer(segments[2], mss, offset);
                    to_print2 = calculations::show_solution_va_to_pa(segments[2], ss, offset, va, pa_ans, power_of2, format_specifiers);
                }
                0 | 1 => {
                    // code, heap
                    // perform the math necessary to solve the given question
                    // the physical address (answer to the question).
                    let pa_ans = calculations::calculate_answer(segments[ss as usize], mss, offset);
                    to_print2 = calculations::show_solution_va_to_pa(segments[ss as usize], ss, offset, va, pa_ans, power_of2, format_specifiers);
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
        Choice2 => {  // stack problem
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
            let tuple = calculations::calculate_answer_stack_percentage(segments[2], percent, mss, power_of2);
            let va_ans = tuple.0;
            let offset = tuple.1;
            to_print2 = calculations::show_solution_stack_va(segments[2], offset, va_ans, power_of2, percent, format_specifiers.1);
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


        _ => {exit(-1);}
    }
}

#[get("/search", rank=2)]
pub fn solution() -> io::Result<NamedFile>{
    NamedFile::open("static/solution.html")
}


/*#[get("/search/<term2>", rank=2)]
pub fn compare_user_answer_to_actual(term2: &RawStr) -> io::Result<NamedFile> {
    format!("You typed in {}", term2);
    NamedFile::open("static/va_to_pa_format.html")
}*/

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

    // compiler says "unreachable expression".
    /*Template::render(
        "result",
        &TemplateContext {
            query: "invalid".to_string(),
            items: "Please reference available commands.".to_string(),
            parent: "index",
        },
    )*/

 /*if(qry2.contains("3")){
     return Template::render("result",&TemplateContext {
    query: qry2.to_string(),
    items : "new query".to_string(),
    parent: "index",
}) ;
} */
    /*if format_choice == 2 {
        return Template::render(
            "result",
            &TemplateContext {
                query: "second option".to_string(),
                items: "new query".to_string(),
                parent: "index",
            },
        );
    } */

/*return Template::render("result", &TemplateContext {
                    query: qry.to_string(),
                    items: func_result,
                    parent: "index",
                }) */
                /*let func_result3 = calculations::show_solution_va_to_pa_hex(
                    res_tuple.2[0],
                    0,
                    1000,
                    res_tuple.0,
                    (res_tuple.2[0].base) * 1024 + 1000,
                    res_tuple.1,
                    (16, 16),
                );

                let func_result4 = func_result + &func_result3;*/

/*  let mut reset = true;
    while reset {
        let x: (u32, u32, Vec<calculations::Segment>) = generate_segmented_memory_layout();
        clear_screen();
        loop {
            // clear_screen();
            let mut input_string = String::new();
            println!("OPTION\t\tPROBLEM TYPE\n");
            println!(
                "0\u{29}\t\tTranslate Random Virtual Address to a corresponding Physical Address"
            );
            println!("1\u{29}\t\tTranslate Random Virtual Address Returned by Malloc() to a corresponding Physical Address");
            println!(
                "2\u{29}\t\tCalculate Specified Portion through the Stack as a Virtual Address"
            );
            println!("8\u{29}\t\tGenerate fresh segmented memory model");
            println!("9\u{29}\t\tGenerate Random Problem");
            println!("10\u{29}\t\tExit");

            match io::stdin().read_line(&mut input_string) {
                Ok(_) => {} // this function returns a result type
                Err(_) => {
                    continue;
                }
            }
            let y: i8 = match input_string.trim().parse::<i8>() {
                Ok(k) => k,
                Err(_) => {
                    println!("Error. Invalid input [not an integer in the specified range]. Please try again.\n");
                    -1
                }
            };
            if y == -1 {
                continue;
            } else {
                match y {
                    0 => {
                        atqs[0](x.0, x.1, x.2.clone());
                    }
                    1 => {
                        atqs[1](x.0, x.1, x.2.clone());
                    }
                    2 => {
                        atqs[2](x.0, x.1, x.2.clone());
                    }
                    8 => {
                        reset = true;
                        clear_screen();
                        println!("Generated.");
                        break;
                    }
                    9 => {
                        let mut rng = rand::thread_rng(); // seed the rng
                        let rando: usize = rng.gen_range(0, atqs.len());
                        atqs[rando](x.0, x.1, x.2.clone());
                    }
                    10 => {
                        return;
                    }
                    _ => {
                        continue;
                    }
                }
            }
        }
    }
}*/

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
        16 => {writeln!(&mut to_print,
            "What virtual address, in hexadecimal, is {}% into the stack??",
            percent
        ).unwrap();},
        2 => {writeln!(&mut to_print,
            "What virtual address, in binary, is {}% into the stack??",
            percent
        ).unwrap();},
        10 => {writeln!(&mut to_print,
            "What virtual address, in decimal, is {}% into the stack??",
            percent
        ).unwrap();},
        _ => {
            writeln!(&mut to_print, "Unexpected error. Exiting").unwrap();
            calculations::error();
        }
    }
    (aformat, to_print)
}

// takes a format flag passed from the client and prints the question returning a tuple of format specifiers
// question text taken with permission from Mark Morissey's slides
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

fn va_to_pa(
    vas: u32,
    power_of2: u32,
    segments: Vec<calculations::Segment>,
) -> (u32, u32, Vec<calculations::Segment>) {
    /*let retv = choose_format(0);
    clear_screen();
    print_layout(vas, vas * 2, power_of2, segments.clone());

    // fetch random u32 in between 100 and the VAS (as a power of 2) as the virtual address to be calculated.
    let va: u32 = calculations::get_rand_va(power_of2, segments.clone(), false);
    // let format_specifiers = print_question_va_to_pa(va, choice, false);

    // new addition: 7/25/2019
    let mut format_specifiers: (i8, i8) = (0, 0);
    format_specifiers.0 = print_question_va_to_pa(va, choice, false).0;
    format_specifiers.1 = print_question_va_to_pa(va, choice, false).1;

    // calculate offset:
    let ss: u32 = va >> (power_of2 - 2);
    let mss: u32 = 2u32.pow(power_of2 - 2); // MSS = 2^(number of bits in the offset)
    let mut bit_mask: u32 = 0;
    let pa: u32;
    for i in 0..power_of2 - 2 {
        // we only want to mask the bits up to the ss
        bit_mask += 2u32.pow(i); // turning on bits in the mask value
    }
    let offset: u32 = va & bit_mask; // the expression on the left = va but with the 2 highest order bits set to 0 which is the same as the offset

    match ss {
        3 => {
            // stack ss
            pa = calculations::calculate_answer(segments[2], mss, offset);
            calculations::compare_answer(format_specifiers.1, pa);
        }
        0 | 1 => {
            // code, heap
            pa = calculations::calculate_answer(segments[ss as usize], mss, offset);
            calculations::compare_answer(format_specifiers.1, pa);
        }

        _ => {
            // if so then print error message and exit. --BUG
            println!("Error. Segment selector doesnt represent any of the implemented segments. It equals {}", ss);
            println!("Exiting program.");
            exit(-1);
        }
    }
    loop {
        let mut input_string = String::new();
        println!("\nOPTION\t\tPROBLEM TYPE");
        println!("0\u{29}\t\tShow steps");
        println!("1\u{29}\t\tReturn to the previous menu");
        println!("2\u{29}\t\tExit");

        match io::stdin().read_line(&mut input_string) {
            Ok(_) => {}
            Err(_) => {
                continue;
            }
        }
        let y: i8 = match input_string.trim().parse::<i8>() {
            Ok(k) => k,
            Err(_) => {
                println!("Error. Invalid input --not an integer. Please try again.");
                -1
            }
        };
        if y == -1 {
            continue;
        } else {
            match y {
                0 => {
                    match ss {
                        3 => {
                            // stack ss --show solution
                            calculations::show_solution_va_to_pa_hex(
                                segments[2],
                                ss,
                                offset,
                                va,
                                pa,
                                power_of2,
                                format_specifiers,
                            );
                        }
                        0 | 1 => {
                            // code, heap --show solution
                            calculations::show_solution_va_to_pa_hex(
                                segments[ss as usize],
                                ss,
                                offset,
                                va,
                                pa,
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
                }
                1 => {
                    break;
                }
                2 => {
                    exit(0);
                }
                _ => {
                    println!("Unexpected error parsing integer input. Exiting.");
                    exit(-1);
                }
            }
        }
    }*/
    (vas, power_of2, segments)
}

// function almost identical to va_to_pa
// question text taken with permission from Mark Morissey's slides
fn va_to_pa_malloc(
    vas: u32,
    power_of2: u32,
    segments: Vec<calculations::Segment>,
) -> (u32, u32, Vec<calculations::Segment>) {
    /*let choice: i8 = choose_format(0);
    //clear_screen();
    print_layout(vas, vas * 2, power_of2, segments.clone());

    // fetch random u32 in between 100 and the VAS (as a power of 2) as the virtual address to be calculated.
    let va: u32 = calculations::get_rand_va(power_of2, segments.clone(), true);

    //let format_specifiers = print_question_va_to_pa(va, choice, true);

    // new addition: 7/25/2019
    let mut format_specifiers: (i8, i8) = (0, 0);
    format_specifiers.0 = print_question_va_to_pa(va, choice, true).0;
    format_specifiers.1 = print_question_va_to_pa(va, choice, true).1;

    // calculate offset:
    let ss: u32 = va >> (power_of2 - 2);
    let mss: u32 = 2u32.pow(power_of2 - 2); // MSS = 2^(number of bits in the offset)
    let mut bit_mask: u32 = 0;
    let pa: u32;
    for i in 0..power_of2 - 2 {
        // we only want to mask the bits up to the ss
        bit_mask += 2u32.pow(i); // turning on bits in the mask value
    }
    let offset: u32 = va & bit_mask; // the expression on the left = va but with the 2 highest order bits set to 0 which is the same as the offset

    pa = calculations::calculate_answer(segments[ss as usize], mss, offset);
    calculations::compare_answer(format_specifiers.1, pa);
    loop {
        let mut input_string = String::new();
        println!("\nOPTION\t\tPROBLEM TYPE");
        println!("0\u{29}\t\tShow steps");
        println!("1\u{29}\t\tReturn to the previous menu");
        println!("2\u{29}\t\tExit");

        match io::stdin().read_line(&mut input_string) {
            Ok(_) => {}
            Err(_) => {
                continue;
            }
        }
        let y: i8 = match input_string.trim().parse::<i8>() {
            Ok(k) => k,
            Err(_) => {
                println!("Error. Invalid input --not an integer. Please try again.");
                -1
            }
        };
        if y == -1 {
            continue;
        } else {
            match y {
                0 => {
                    // show solution(s)
                    match ss {
                        3 => {
                            // stack ss
                            calculations::show_solution_va_to_pa_hex(
                                segments[2],
                                ss,
                                offset,
                                va,
                                pa,
                                power_of2,
                                format_specifiers,
                            );
                        }
                        0 | 1 => {
                            calculations::show_solution_va_to_pa_hex(
                                segments[ss as usize],
                                ss,
                                offset,
                                va,
                                pa,
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
                }
                1 => {
                    break;
                }
                2 => {
                    exit(0);
                }
                _ => {
                    println!("Unexpected error parsing integer input. Exiting.");
                    exit(-1);
                }
            }
        }
    }*/
    (vas, power_of2, segments)
}

// generates the stack portion -> VA question
fn stack_va(
    vas: u32,
    power_of2: u32,
    segments: Vec<calculations::Segment>,
) -> (u32, u32, Vec<calculations::Segment>) {
    // choose format of the question required answer (hex, dec, binary).
    /*let choice: i8 = choose_format(1);
    clear_screen();
    print_layout(vas, vas * 2, power_of2, segments.clone());

    // generate a percentage as a quarter of some number (1/4, 2/4, 3/4, etc etc).
    let mut rng = rand::thread_rng(); // seed the rng
    let quarters = [100.0, 25.0, 75.0, 50.0, 0.0];
    let rando = rng.gen_range(0, quarters.len());
    let percent: f32 = quarters[rando];
    let format_specifier = print_question_stack_percentage(percent as u32, choice);

    // MSS = 2^number of bits in the offset = number of total bits - 2 (because SS = 2 bits)
    let mss: u32 = 2u32.pow(power_of2 - 2); // MSS = 2^(number of bits in the offset)
    let tuple =
        calculations::calculate_answer_stack_percentage(segments[2], percent, mss, power_of2);
    let va_ans = tuple.0;
    let offset = tuple.1;
    calculations::compare_answer(format_specifier, va_ans);

    // loop until the user breaks the loop and exits or returns to the previous menu.
    loop {
        let mut input_string = String::new();
        println!("\nOPTION\t\tPROBLEM TYPE");
        println!("0\u{29}\t\tShow steps");
        println!("1\u{29}\t\tReturn to the previous menu");
        println!("2\u{29}\t\tExit");

        match io::stdin().read_line(&mut input_string) {
            Ok(_) => {}
            Err(_) => {
                continue;
            }
        }
        let y: i8 = match input_string.trim().parse::<i8>() {
            Ok(k) => k,
            Err(_) => {
                println!("Error. Invalid input --not an integer. Please try again.");
                -1
            }
        };
        if y == -1 {
            continue;
        } else {
            match y {
                0 => {
                    calculations::show_solution_stack_va(
                        segments[2],
                        offset,
                        va_ans,
                        power_of2,
                        percent,
                        format_specifier,
                    );
                }
                1 => {
                    break;
                }
                2 => {
                    exit(0);
                }
                _ => {
                    println!("Unexpected error parsing integer input. Please try again");
                    continue;
                }
            }
        }
    }*/
    (vas, power_of2, segments)
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
fn choose_format(question_flag: u8) -> (i8, String) {
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
                writeln!(&mut to_print, "Unexpected Fatal error in question format function. Exiting.").unwrap();
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
                writeln!(&mut to_print, "Error. Invalid input --not an integer. Please try again.").unwrap();
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
                    writeln!(&mut to_print, "Please enter one of the digits corresponding to an option on screen").unwrap();
                    continue;
                }
            }
        }
    }
    (choice, to_print)
}

// function useful for clearing the output buffer
fn clear_screen() -> String {
    let mut to_print = String::new();
    for _i in 0..50 {
        writeln!(&mut to_print).unwrap();
    }
    to_print
}
