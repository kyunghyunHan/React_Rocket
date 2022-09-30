use crate::db::schema::users;

#[table_name = "users"]
#[derive(Serialize, Deserialize, Queryable, Insertable)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub identity: String,
    pub hometown: String,
    pub age: i32,
}
/// This represents a page view being inserted into the database, without the auto-generated fields
#[derive(Deserialize, Insertable)]
#[table_name = "users"]
pub struct InsertableUserView {
    pub name: String,
    pub identity: String,
    pub hometown: String,
    pub age: i32,
}
