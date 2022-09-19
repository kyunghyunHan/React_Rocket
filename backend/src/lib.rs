#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
extern crate rocket_cors;
use rocket_cors::{AllowedHeaders, AllowedOrigins, Cors, CorsOptions, Error};

use rocket::http::Method;
mod routes;
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

pub fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount(
            "/",
            routes![
                routes::test::index,
                routes::test::myrocket,
                routes::test::login
            ],
        )
        .attach(make_cors())
}
