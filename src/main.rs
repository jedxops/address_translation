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
mod files;
mod lib_fns;
mod post;
fn rocket() -> rocket::Rocket {
    rocket::ignite().attach(Template::fairing()).mount(
        "/",
        routes![files::file, files::index, files::index1,files::response, post::compute],
    )
}

fn main() {
    rocket().launch();
}
