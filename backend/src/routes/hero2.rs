use crate::db::connecion::Conn;
use crate::db::hero;
use crate::db::hero::Hero;

use crate::db::schema::heroes;

use diesel::prelude::*;

use diesel;
use diesel::mysql::MysqlConnection;
use diesel::result::Error;
use rocket::http::Status;
use rocket_contrib::json::Json;
use rocket_contrib::json::JsonValue;
use serde::*;

// impl Hero {
//     pub fn create(hero: Hero, connection: &MysqlConnection) -> Hero {
//         diesel::insert_into(heroes::table)
//             .values(&hero)
//             .execute(connection)
//             .expect("Error creating new hero");

//         heroes::table
//             .order(heroes::id.desc())
//             .first(connection)
//             .unwrap()
//     }

//     pub fn read(connection: &MysqlConnection) -> Vec<Hero> {
//         heroes::table
//             .order(heroes::id.asc())
//             .load::<Hero>(connection)
//             .unwrap()
//     }

//     pub fn update(id: i32, hero: Hero, connection: &MysqlConnection) -> bool {
//         diesel::update(heroes::table.find(id))
//             .set(&hero)
//             .execute(connection)
//             .is_ok()
//     }

//     pub fn delete(id: i32, connection: &MysqlConnection) -> bool {
//         diesel::delete(heroes::table.find(id))
//             .execute(connection)
//             .is_ok()
//     }
// }
// pub fn create_post(hero: Json<Hero>, conn: &MysqlConnection) -> QueryResult<Hero> {
//     diesel::insert_into(heroes::table)
//         .values(&hero)
//         .get_result(conn)
// }

#[post("/read")]
pub fn read() -> Json<JsonValue> {
    Json(json!(["hero 1", "hero 2"]))
}
#[post("/hello", format = "json", data = "<hero>")]
pub fn create(hero: Json<Hero>) -> Json<Hero> {
    hero
}
// #[post("/hello", data = "<hero>")]
// pub fn create(hero: Json<Hero>, connection: Conn) -> Json<Hero> {
//     let insert = Hero {
//         id: None,
//         ..hero.into_inner()
//     };
//     Json(Hero::create(insert, &connection))
// }
#[delete("/hello/<id>")]
pub fn delete(id: i32) -> Json<JsonValue> {
    Json(json!({"status": "ok"}))
}
fn error_status(error: Error) -> Status {
    match error {
        Error::NotFound => Status::Ok, // 챗봇은 무조건 200
        _ => Status::InternalServerError,
    }
}
