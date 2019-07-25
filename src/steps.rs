
use rocket::request::{ Form };

use rocket_contrib::templates::Template;
use crate::main1;
use crate::calculations;

#[derive(Serialize)]
pub struct TemplateContext {
    items: String,
    parent: &'static str,
}

#[derive(FromForm)]
pub struct Request {
    term: String,
}

#[post("/search", data = "<data>")]
pub fn compute(data: Form<Request>) -> Template {

    let qry = &data.term;
    let res_tuple = main1::generate_segmented_memory_layout();
    let func_result = main1::print_layout(res_tuple.0,(res_tuple.0)*2,res_tuple.1,res_tuple.2);
    return Template::render("steps", &TemplateContext {
            items: func_result,
            parent: "result",
        })
/*Template::render("result",&TemplateContext {
    query: ""
    parent: ""
}) */

/*Template::render("result", &TemplateContext {
        query: "invalid".to_string(),
        items: vec!["Please reference available commands.".to_string()],
        parent: "index",
    }) */
}
