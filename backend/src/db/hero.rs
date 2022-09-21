use crate::db::schema::heroes;
use diesel;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;

#[table_name = "heroes"]
#[derive(Serialize, Deserialize, Queryable, Insertable)]
pub struct Hero {
    pub id: Option<i32>,
    pub name: String,
    pub identity: String,
    pub hometown: String,
    pub age: i32,
}
