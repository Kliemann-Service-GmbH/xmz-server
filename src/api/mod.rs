use rocket;
use rocket::http::RawStr;

#[get("/")]
fn index() -> &'static str {
    "RA-GAS GmbH (c) 1978 - 2018"
}

#[get("/<name>")]
fn hello(name: &RawStr) -> String {
    format!("Hello, {}", name.as_str())
}

pub fn launch() {
    rocket::ignite()
        .mount("/", routes![index, hello])
        .mount("/v1", routes![index, hello])
        .launch();
}
