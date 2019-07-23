
use rocket::request::{ Form };

use rocket_contrib::templates::Template;
use crate::main1;

#[derive(Serialize)]
pub struct TemplateContext {
    query: String,
    v1:u32,
    v2: u32,
    v3:u32,
    //base:u32,
    //size:f32,
    //items: (u32,f32),
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
    main1::print_layout(res_tuple.0,(res_tuple.0)*2,res_tuple.1,res_tuple.2);
    //let func_result = main1::str(res_tuple.2);
    return Template::render("result", &TemplateContext {
            query: qry.to_string(),
            v1: res_tuple.0,
            v2:(res_tuple.0)*2,
            v3:res_tuple.1,
        //    items: func_result,
            parent: "index",
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
