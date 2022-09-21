#![allow(proc_macro_derive_resolution_fallback)]

use crate::db::hero::Hero;
use crate::db::schema::heroes;
use diesel;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;

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
