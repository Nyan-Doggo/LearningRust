// i have no clue what this line menas
#[macro_use] extern crate rocket;

// defines how to handle a get-request at the route "/"
#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

// starts the server
#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}