#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;

#[cfg(test)] mod tests;

use rocket::Request;
use rocket::request::Form;
use rocket_contrib::templates::Template;

#[derive(Serialize)]
struct TemplateContext {
    title: &'static str,
    parent: &'static str,
}

#[derive(FromForm)]
struct MyForm {
    name: String,
    start_date: String,
    end_date: String,
}

#[get("/")]
fn index() -> Template {
    Template::render("index", &TemplateContext {
        title: "Tutaj",
        parent: "layout",
    })
}

#[get("/form")]
fn form() -> Template {
    let mut context = std::collections::HashMap::new();
    context.insert("title", "Form");
    Template::render("form/form", &context)
}

#[post("/form", data = "<form>")]
fn submit(form: Form<MyForm>) -> Template {
    let fields = form.into_inner();
    let mut context = std::collections::HashMap::new();
    context.insert("name", fields.name);
    context.insert("start_date", fields.start_date);
    context.insert("end_date", fields.end_date);
    Template::render("form/submit", &context)
}

#[catch(404)]
fn not_found(req: &Request<'_>) -> Template {
    let mut map = std::collections::HashMap::new();
    map.insert("path", req.uri().path());
    Template::render("error/404", &map)
}

fn setup() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![index, form, submit])
        .register(catchers![not_found])
        .attach(Template::fairing())
}

fn main() {
    setup().launch();
}
