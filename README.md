## Address translation

This program produces a random address translation problem and either

* --congratulates the user for providing the correct solution
* --explains to the user how to solve the problem showing steps

First, the code generates a segmented memory layout. It randomly generates values for the size of the physical memory, virtual
address space, and segment sizes. It also generates non-conflicting segment bases. 

Next, the code prompts the user with a problem requesting an address translation from a virtual address to a physical address 
_or_ requests the user to calculate the virtual address of X percentage through the stack. The generated questions for the 
user are based on the segmented memory layout generated --mentioned above. In addition, the arbitrary numeric component of 
each question is generated randomly within the range of values that the memory layout provides.

Regardless of if the user's solution is correct or incorrect, the program provides an option to display the steps to solving the 
problem. The user can select from a variety of question formats after choosing a learning path upon boot of the software.
The question format varies on the question in question as well as the number system used --the current version of this software
supports questions prompting for answers in binary, hex, or decimal [base 2, base 16, or base 10]. 
The user can exit or continue practicing questions at their own leisure.

## Project Inspiration

This project was inspired by Mark Morrissey and his CS333 class at Portland State Univeristy. The hope is that this software will
assist end users performing address translation with a calculation tool which verifies their address translation computation
_process_ (math/steps). This software is intended to assist all users performing address translation regardless of their 
purposes --educational, industrial, personal --manual or automated.

## Roadmap (tenative):

Week 1)     Develop repository along with README.md, LICENSE files. 
            Introduce project to stakeholders (project members and professor).

Week 2)     Begin development. Finish prototype.

Week 3)     Demonstrate prototype. Continue development.

Week 4)     Continue development.

Week 5)     Finish source development.

Week 6)     Demonstrate the difference between the prototype and the current version of the product.
            (MVP --minimum viable product); begin turning project into a web app.

Week 7)     Continue working towards turning the project into a web app.

Week 8)     Finish web app development.

Post-class)     Maintain and update software periodically. 
                Share project with prospective CS333 students so they can test it and use it to help
                them with taking their OS course (CS333 at PSU).

## Build Prerequisites

In order to build this repository, you must install rust and rust nightly. `rustup` is a great way to do this: https://rustup.rs/

Once rust has been downloaded and installed, you can install nightly with the following command

    rustup override set nightly-2019-07-09

## Checkout, Build, and Run

First, clone this repository by using `git clone repository_link` on your local machine to clone the repository.
Click the green `Clone or download` button to copy the repository link to your clipboard.

Once rust has been installed, you can build this program using the command: `cargo build` and run it with
`cargo run --bin binary_filename`.

    cargo build
    cargo run --bin web_app
    OR
    cargo run --bin cli

If you chose to run the web app you will need to: press ctrl + click on the following link that appears after cargo successfully running: 

    http://localhost:8000

If you want to run an optimized version then run cargo with: `cargo run --bin binary_filename --release`. `cargo build --release` builds an
optimized version of the code.

## Code Operation

OPTION       PROBLEM TYPE

0)          Translate Random Virtual Address to a corresponding Physical Address
1)          Translate Random Virtual Address Returned by Malloc() to a corresponding Physical Address
2)          Calculate Specified Portion through the Stack as a Virtual Address
9)          Generate Random Problem
10)         Exit
1

Choose format of desired question

OPTION      PROBLEM TYPE

0)          --va in hex to pa in hex
1)          --va in hex to pa in binary
2)          --va in hex to pa in decimal
3)          --va in binary to pa in hex
4)          --va in binary to pa in binary
5)          --va in binary to pa in decimal
6)          --va in decimal to pa in hex
7)          --va in decimal to pa in binary
8)          --va in decimal to pa in decimal
9)          Random option
3

Assume a 128KB virtual address space and a 256KB physical memory. Virtual addresses are 18 bits and segmentation is being used. The segment information is:
                Segment Number  Base    Size    GrowsNegative
                Code      00     8K     12.8K   0
                Heap      01     90K    2.1K    0
                Stack     11    245K    55.2K   1
A call to malloc returns a virtual address of 10110100000000000. What is the physical address (in base 16) of this virtual address?
Type your answer in hexadecimal format with or without the `0x` then press enter and ctrl+d

0xa4f

INCORRECT.

your answer: 0xA4F bytes

actual: 0x1D000 bytes

OPTION      PROBLEM TYPE

0)          Show steps
1)          Return to the previous menu
2)          Exit
0

Step 1: Convert virtual address 10110100000000000 to binary
10110100000000000 = 01 0110 1000 0000 0000 

Step 2: Note the Virtual Address Space size (abbrv. VAS --measured in bits) and separate the Segment Selector (SS) from the Offset portion of the binary.
Remember --if the amount of bits in the Virtual Address Space differs from the amount of bits in the binary calculated, we must either

a) Pad the calculated binary number with zeros until the length of the binary equals the amount of bits in the VAS.

b) Trim the top bits of the calculated binary until the length of the binary equals the amount of bits in the VAS.

In this case, the Virtual Address Space size in bits is 18.
So, only the first 18 bits of the calculated binary are considered.

The SS is always either 00, 01, or 11 => SS ϵ {0, 1, 3}

Discard the segment selector bits from the offset calculation.

01 110100000000000
-- ---------------
SS    OFFSET

Step 3: Note the value of the Segment Selector and Offset bits:
00 ===> Code
01 ===> Heap
11 ===> Stack

Offset = 110100000000000 = 26624 bytes

Step 4: Note: PA = (-1)*(GN)*(MSS) + base + offset

GN = `grows negative`. If SS = 11 => SS = Stack => GN = 1. Otherwise, GN = 0.
MSS = `maximum segment size`. MSS = 2^(number of bits in the offset)
Base = the base of the segment, measured in bytes. Value provided in table.
The offset has already been calculated: offset = 26624 bytes (base 10)
There are 16 bits in the offset, so the MSS is 2^16 = 65536 bytes.
The SS = 01 (base 2) = 1 (base 10) => SS = Heap Segment => GN = 0

PA (in bytes) = (-1)*(GN)*(MSS) + base + offset

=> PA = (-1)(0)(2^16) + (90K) + 26624 bytes

=> PA = (-1)(0)(65536) + (90 * 1024) + 26624 bytes

=> PA = 0 + (92160) + 26624 bytes

=> PA = 118784 bytes

=> PA = 0x1D000 bytes

Check out youtube for shortcuts on converting to and from binary, decimal, and hexadecimal by hand.

OPTION      PROBLEM TYPE

0)          Show steps
1)          Return to the previous menu
2)          Exit
1

OPTION      PROBLEM TYPE

0)          Translate Random Virtual Address to a corresponding Physical Address
1)          Translate Random Virtual Address Returned by Malloc() to a corresponding Physical Address
2)          Calculate Specified Portion through the Stack as a Virtual Address
8)          Generate fresh segmented memory model
9)          Generate Random Problem
10)         Exit

10

## Authors

Copyright (c) 2019
Jeff Austin <jja6@pdx.edu> (github user jedxops),
Kamakshi Nagar <kamakshi@pdx.edu> (nagarkamakshi),
and Bart Massey <bart@cs.pdx.edu> (BartMassey) (development team advisor).

## License

This program is licensed under the "MIT License". Please
see the file `LICENSE` in the source distribution of this
software for license terms (highest level directory of this
repository).

## Awknowledgements

Variety of changes suggested by Bart Massey. He also participated in the project by providing
support to its developers when they had questions. Thanks and shout out to Prof. Massey!

Mark Morrissey's CS333 class was the inspiration of this entire project. With his permission,
similar practice questions and problem layouts from his course have been designed and used throughout the
course of this software's development. Thanks to Mark for being a great teacher and creating 
and teaching the course which inspired this project!

One or more of the developers were completely new to the rust programming language during
this project, but still actively participated and heavily assisted in building a decently
sized project. A big thank you to this developer and to Jim Blandy and his textbook:
Blandy, J., & Orendorff, J. (2018). Programming Rust: Fast, safe systems development. Sebastopol: OReilly Media.
This text was used to assist with learning many of the programming techniques in rust.
Shout out and thanks to Jim as well.

## Citations

Mark Morrissey --CS333 Operating Systems--Portland State University practice exams:
https://web.cecs.pdx.edu/~markem/CS333/exams/Final%202019-01.pdf

Bart Massey
http://web.cecs.pdx.edu/~bart/

Rust textbook:
Blandy, J., & Orendorff, J. (2018). Programming Rust: Fast, safe systems development. Sebastopol: OReilly Media.

Link to guide referenced for making _this_ README.md (markdown file extension) file:
https://gist.github.com/PurpleBooth/109311bb0361f32d87a2

Link to guide referenced for learning to create _this_ open source project:
https://opensource.guide/starting-a-project/
