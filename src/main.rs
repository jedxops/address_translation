// Copyright © 2019 Jeff Austin, Kamakshi Nagar
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

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
            routes::setup,
            routes::solution
        ],
    )
}

// main driver program
fn main() {
    rocket().launch();
}
