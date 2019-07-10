use super::{setup, TemplateContext};

use rocket::local::Client;
use rocket::http::Status;
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
