
use rocket::request::{ Form };
use rocket::response::Redirect;

use rocket_contrib::templates::Template;

#[derive(Serialize)]
pub struct TemplateContext {
    query: String,
    items: Vec<String>,
    parent: &'static str,
}

#[derive(FromForm)]
pub struct Request {
    term: String,
}
#[post("/search", data = "<data>")]
pub fn compute(data: Form<Request>) -> Template {

Template::render("result", &TemplateContext {
        query: "invalid".to_string(),
        items: vec!["Please reference available commands.".to_string()],
        parent: "index",
    })
}
