
use rocket::request::{ Form };

use rocket_contrib::templates::Template;
use crate::main1;
use crate :: calculations;

#[derive(Serialize)]
pub struct TemplateContext {
    query: String,
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
    //if(qry.contains("1")){
    let res_tuple = main1::generate_segmented_memory_layout();
    //main1::va_to_pa(res_tuple.0,res_tuple.1,res_tuple.2.clone());
    let func_result = main1::print_layout(res_tuple.0,(res_tuple.0)*2,res_tuple.1,res_tuple.2.clone());
    let func_result2 = main1::print_question_va_to_pa(res_tuple.0,0,false);
    let func_result = func_result + &func_result2.2;
    /*return Template::render("result", &TemplateContext {
            query: qry.to_string(),
            items: func_result,
            parent: "index",
        }) */

     let func_result3= calculations::show_solution_va_to_pa_hex
        (res_tuple.2[0].clone(),0,1000,res_tuple.0,(res_tuple.2[0].base)*1024+1000,res_tuple.1,(16,16));

        let func_result4 = func_result + &func_result3;
        return Template::render("result", &TemplateContext {
                query: qry.to_string(),
                items: func_result4,
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
