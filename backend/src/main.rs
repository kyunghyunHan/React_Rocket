#![feature(proc_macro_hygiene, decl_macro)]
use rocket_cors::{AllowedHeaders, AllowedOrigins, Cors, CorsOptions, Error};
#[macro_use]
extern crate rocket;
extern crate rocket_cors;
use rocket::http::Method;
fn make_cors() -> Cors {
    let allowed_origins =
        AllowedOrigins::some_exact(&["http://localhost:8000", "http://localhost:3000"]);
    CorsOptions {
        // 5.
        allowed_origins,
        allowed_methods: vec![Method::Get].into_iter().map(From::from).collect(), // 1.
        allowed_headers: AllowedHeaders::some(&[
            "Authorization",
            "Accept",
            "Access-Control-Allow-Origin", // 6.
        ]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("error while building CORS")
}
#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}
#[get("/myrocket")]
fn myrocket() -> String {
    "My 🚀 server".to_string()
}
#[get("/login")]
fn login() -> String {
    "login".to_string()
}
fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount(
            "/",
            routes![
                //PASS IN THE NAME OF YOUR ROUTES HERE
                index, myrocket, login
            ],
        )
        .attach(make_cors())
}
fn main() {
    rocket().launch();
}
