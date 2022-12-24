mod resources;
use resources::routes::rs;

#[macro_use]
extern crate rocket;

#[get("/teste")]
fn index() -> &'static str {
    return "Hello, Wworld!";
}

#[launch]
fn rocket() -> _ {
    rocket::build()
    .mount("/", routes![index, rs::ola])

    //  se mudar a base ai deve se usar um novo "mount"
    // .mount("/teste", routes![rs::ola])

    // funciona incluir varias rotas na mesma lista assim:
    //.mount("/", routes![index, rs::ola, ...etc])
}
