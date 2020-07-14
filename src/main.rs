#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello world!"
}

#[get("/bye")]
fn bye() -> &'static str {
    "Goodbye world"
}

fn main() {
    rocket::ignite().mount("/", routes![index, bye]).launch();
}