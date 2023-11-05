#[macro_use] extern crate rocket;

mod session;
mod message;
mod api;

use rocket::response::content::RawHtml;
use rocket_dyn_templates::Template;
use persistence::data_persistence::DataPersistence;

#[get("/")]
fn index() -> RawHtml<&'static str> {
    RawHtml(r#"<a href="message">Set a Message</a> or <a href="session">Use Sessions</a>."#)
}

#[launch]
fn rocket() -> _ {
    let mut db = DataPersistence::new();
    rocket::build()
        .attach(Template::fairing())
        .mount("/", routes![index])
        .mount("/create_admin", api::create_admin::routes(&mut db))
        .mount("/message", message::routes())
        .mount("/session", session::routes())
}