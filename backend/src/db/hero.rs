use crate::db::schema::heroes;

#[table_name = "heroes"]
#[derive(Serialize, Deserialize, Queryable, Insertable)]
pub struct Hero {
    pub id: Option<i32>,
    pub name: String,
    pub identity: String,
    pub hometown: String,
    pub age: i32,
}
/// This represents a page view being inserted into the database, without the auto-generated fields

#[derive(Deserialize, Insertable)]
#[table_name = "heroes"]
pub struct InsertableHero {
    pub name: String,
    pub identity: String,
    pub hometown: String,
    pub age: i32,
}
