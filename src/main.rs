#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate rocket_contrib;
use rocket_contrib::templates::Template;
#[macro_use] extern crate serde_derive;

mod files;
mod post;
mod lib_fns;
mod main1;
mod steps;
mod calculations;

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .attach(Template::fairing())
        .mount("/", routes![files::file,files::index,files::response,post::compute],)
}

fn main() {
    rocket().launch();
}
