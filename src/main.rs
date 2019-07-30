// Copyright © 2019 Jeff Austin, Kamakshi Nagar
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
use rocket_contrib::templates::Template;
#[macro_use]
extern crate serde_derive;

mod calculations;
mod routes;
mod lib_fns;

fn rocket() -> rocket::Rocket {
    rocket::ignite().attach(Template::fairing()).mount(
        "/",
        routes![routes::file, routes::index, routes::q_format_0, routes::q_format_1, routes::q_format_2, routes::response, routes::setup, 
            routes::compare_user_answer_to_actual],
    )
}

fn main() {
    rocket().launch();
}
