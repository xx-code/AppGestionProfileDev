#[macro_use] extern crate rocket;

use std::path::PathBuf;

#[get("/")]
fn index() -> &'static str {
    "Hello, world! -- 566"
}

#[get("/hello/<name>")]
fn hello(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[get("/page/<path..>")]
fn get_page(path: PathBuf) {

}

#[launch]
fn rocket() -> _ {
    rocket::build()
    .mount("/", routes![index, hello])
}
