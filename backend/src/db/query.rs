#![allow(proc_macro_derive_resolution_fallback)]
use diesel;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;

use crate::db::hero::Hero;

use crate::db::schema::heroes::dsl::*;

// SELECT * from ajou_sched limit 5
// pub fn show_scheds(connection: &MysqlConnection) -> QueryResult<Vec<Schedule>> {
//     //posts.filter(published.eq(true))
//     heroes.limit(5).load::<Hero>(&*connection)
// }
