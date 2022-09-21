#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
extern crate rocket_cors;
use rocket_cors::{AllowedHeaders, AllowedOrigins, Cors, CorsOptions, Error};

use rocket::http::Method;

#[macro_use]
extern crate rocket_contrib;

#[macro_use]
extern crate diesel;
// #[macro_use]
// extern crate diesel_codegen;
extern crate r2d2;
extern crate r2d2_diesel;

#[macro_use]
extern crate serde_derive;

mod db;
mod routes;

fn make_cors() -> Cors {
    // let allowed_origins = AllowedOrigins::all();
    let allowed_origins =
        AllowedOrigins::some_exact(&["http://localhost:3000", "http://localhost:8000"]);
    CorsOptions {
        // 5.
        allowed_origins,
        allowed_methods: vec![
            Method::Get,
            Method::Post,
            Method::Delete,
            Method::Options,
            Method::Patch,
        ]
        .into_iter()
        .map(From::from)
        .collect(), // 1.
        allowed_headers: AllowedHeaders::All,
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("error while building CORS")
}
pub fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .manage(db::connecion::init_pool())
        .mount(
            "/",
            routes![
                routes::hero2::read,
                routes::hero2::create,
                routes::hero2::delete
            ],
        )
        .attach(make_cors())
}
