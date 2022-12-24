mod resources;
use resources::routes::rs;

#[macro_use]
extern crate rocket;

#[get("/teste")]
fn index() -> &'static str {
    return "Hello, world!";
}

#[launch]
fn rocket() -> _ {
    rocket::build()
    .mount("/", routes![index])
    .mount("/", routes![rs::ola])
}
