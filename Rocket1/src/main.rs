// import the stuff we need from rocket
use rocket::*;

// defines how to handle a get-request at the route "/"
#[get("/")]
fn index() -> &'static str {
    "hello world"
}

#[get("/delay/<seconds>")]
async fn delay(seconds: u64) -> String {
    tokio::time::sleep(tokio::time::Duration::from_secs(seconds)).await;
    format!("slept for {seconds} seconds")
}

// starts the server
#[launch]
fn rocket() -> _ {
    // mounting was hard to get, but it seems like this gets the "decorator" route and appends it to the
    // root at "/", so you could say that "mount x [fny, fnz, fnw]" prepeds X to the path for all the functions
    // in a sense? maybe maybe. this works tho ^~^
    rocket::build().mount("/", routes![index, delay])
}
