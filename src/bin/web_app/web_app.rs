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

// rocket necessities
#![feature(proc_macro_hygiene, decl_macro)]

// rocket necessities
#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
use rocket_contrib::templates::Template;
#[macro_use]
extern crate serde_derive;

// modules for other files (functions we need in those files)
mod calculations;
mod lib_fns;
mod routes;

// launcher
fn rocket() -> rocket::Rocket {
    rocket::ignite().attach(Template::fairing()).mount(
        "/",
        routes![
            routes::file,
            routes::index,
            routes::q_format_0,
            routes::q_format_1,
            routes::q_format_2,
            routes::q_format_3,
            routes::setup,
            routes::solution,
        ],
    )
}

// main driver program
fn main() {
    rocket().launch();
}
