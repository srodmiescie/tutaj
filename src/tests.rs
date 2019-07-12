use super::{setup, TemplateContext};

use rocket::local::Client;
use rocket::http::Status;
use rocket::http::ContentType;
use rocket_contrib::templates::Template;

#[test]
fn index() {
    let client = Client::new(setup()).unwrap();
    let mut response = client.get("/").dispatch();
    let context = TemplateContext {
        title: "Tutaj",
        parent: "layout",
    };
    let expected = Template::show(client.rocket(), "index", &context).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.body_string(), Some(expected));
}

#[test]
fn not_found() {
    let client = Client::new(setup()).unwrap();
    let mut response = client.get("/lol").dispatch();
    let mut map = std::collections::HashMap::new();
    map.insert("path", "/lol");
    let expected = Template::show(client.rocket(), "error/404", &map).unwrap();
    assert_eq!(response.status(), Status::NotFound);
    assert_eq!(response.body_string(), Some(expected));
}

#[test]
fn form() {
    let client = Client::new(setup()).unwrap();
    let mut response = client.get("/form").dispatch();
    let mut context = std::collections::HashMap::new();
    context.insert("title", "Form");
    let expected = Template::show(client.rocket(), "form/form", &context).unwrap();
    assert_eq!(response.body_string(), Some(expected));
}

#[test]
fn submit() {
    let mut context = std::collections::HashMap::new();
    context.insert("name", "Tutaj");
    context.insert("start_date", "2019-07-12");
    context.insert("end_date", "2019-07-12");

    let client = Client::new(setup()).unwrap();
    let mut response = client.post("/form")
        .body("name=Tutaj&start_date=2019-07-12&end_date=2019-07-12")
        .header(ContentType::Form)
        .dispatch();

    let expected = Template::show(client.rocket(), "form/submit", &context).unwrap();

    assert_eq!(response.body_string(), Some(expected));
}