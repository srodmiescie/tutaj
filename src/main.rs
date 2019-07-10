#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;

#[cfg(test)] mod tests;

use rocket::Request;
use rocket_contrib::templates::Template;

#[derive(Serialize)]
struct TemplateContext {
    title: &'static str,
    parent: &'static str,
}

#[get("/")]
fn index() -> Template {
    Template::render("index", &TemplateContext {
        title: "Tutaj",
        parent: "layout",
    })
}

#[catch(404)]
fn not_found(req: &Request<'_>) -> Template {
    let mut map = std::collections::HashMap::new();
    map.insert("path", req.uri().path());
    Template::render("error/404", &map)
}

fn setup() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![index])
        .register(catchers![not_found])
        .attach(Template::fairing())
}

fn main() {
    setup().launch();
}
