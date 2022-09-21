use crate::db::connecion::Conn;
use crate::db::hero;
use crate::db::hero::Hero;
use crate::db::models::Schedule;
use crate::db::query;
use diesel;
use diesel::result::Error;
use rocket::http::Status;
use rocket_contrib::json::Json;
use rocket_contrib::json::JsonValue;
use serde::{Deserialize, Serialize};
#[get("/hello")]
pub fn read() -> Json<JsonValue> {
    Json(json!(["hero 1", "hero 2"]))
}
#[post("/hello", data = "<hero>")]
pub fn create(hero: Json<Hero>) -> Json<Hero> {
    hero
}
fn error_status(error: Error) -> Status {
    match error {
        Error::NotFound => Status::Ok, // 챗봇은 무조건 200
        _ => Status::InternalServerError,
    }
}
