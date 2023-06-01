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
    rocket::build().mount("/", routes![index, delay])
}
