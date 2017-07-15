#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

pub fn run() {
    rocket::ignite().mount("/", routes![index]).launch();
}


#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}
