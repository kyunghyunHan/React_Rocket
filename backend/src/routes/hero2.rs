use crate::db::connecion::Conn;

use crate::db::hero::Hero;

use crate::db::schema::heroes;

use diesel::prelude::*;

use diesel;

use diesel::result::Error;
use rocket::http::Status;
use rocket_contrib::json::Json;
use rocket_contrib::json::JsonValue;

#[post("/read")]
pub fn read() -> Json<JsonValue> {
    Json(json!(["hero 1", "hero 2"]))
}
#[post("/hello", format = "json", data = "<hero>")]
pub fn create(hero: Json<Hero>) -> Json<Hero> {
    hero
}

#[delete("/hello/<id>")]
pub fn delete(id: i32) -> Json<JsonValue> {
    Json(json!({"status": "ok"}))
}
fn error_status(error: Error) -> Status {
    match error {
        Error::NotFound => Status::Ok,
        _ => Status::InternalServerError,
    }
}

use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct Notice {
    id: u64,
    title: String,
    date: String,
    link: String,
    writer: String,
}

#[get("/hello")]
pub fn hello() -> Json<Notice> {
    let notice = Notice {
        id: 12345,
        title: "공지1".to_string(),
        date: "2021-07-09".to_string(),
        link: "https://".to_string(),
        writer: "CSW".to_string(),
    };
    Json(notice)
}
#[post("/page_view", data = "<page_view>")]

pub fn create_page_view(conn: Conn, page_view: Json<Hero>) -> Result<String, String> {
    let inserted_rows = diesel::insert_into(heroes::table)
        .values(&page_view.0)
        .execute(&*conn.0)
        .map_err(|err| -> String {
            println!("Error inserting row: {:?}", err);

            "Error inserting row into database".into()
        })?;

    Ok(format!("Inserted {} row(s).", inserted_rows))
}
