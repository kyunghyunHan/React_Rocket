use crate::db::connecion::Conn;

use crate::db::schema::users;
use crate::db::user::InsertableUserView;
use crate::db::user::User;
use diesel::prelude::*;

use diesel;

use rocket_contrib::json::Json;
use rocket_contrib::json::JsonValue;

#[post("/read")]
pub fn read() -> Json<JsonValue> {
    Json(json!(["hero 1", "hero 2"]))
}

#[delete("/delete/<id>")]
pub fn delete(id: i32) -> Json<JsonValue> {
    Json(json!({"status": "ok"}))
}
#[post("/join", data = "<page_view>")]

pub fn create(conn: Conn, page_view: Json<InsertableUserView>) -> Result<String, String> {
    let inserted_rows = diesel::insert_into(users::table)
        .values(&page_view.0)
        .execute(&*conn.0)
        .map_err(|err| -> String {
            println!("Error inserting row: {:?}", err);

            "Error inserting row into database".into()
        })?;

    Ok(format!("Inserted {} row(s).", inserted_rows))
}
#[get("/Login")]
pub fn list_page_views(conn: Conn) -> Result<Json<Vec<User>>, String> {
    use crate::db::schema::users::dsl::*;

    users
        .load::<User>(&*conn)
        .map_err(|err| -> String {
            println!("Error querying page views: {:?}", err);
            "Error querying page views from the database".into()
        })
        .map(Json)
}
