use crate::db::connecion::Conn;
use crate::db::schema::heroes;
use diesel::prelude::*;
// pub fn all(connection: Conn) -> QueryResult<Vec<Hero>> {
//     heroes::table.load::<Hero>(*connection)
// }

// pub fn get(id: i32, connection: &Conn) -> QueryResult<Hero> {
//     heroes::table.find(id).get_result::<Hero>(connection)
// }

// pub fn insert(person: Hero, connection: Conn) -> QueryResult<Hero> {
//     diesel::insert_into(heroes::table)
//         .values(&InsertableHero::from_person(person))
//         .get_result(&*connection)
// }

// pub fn update(id: i32, person: Hero, conn: Conn) -> QueryResult<Hero> {
//     diesel::update(heroes::table.find(id))
//         .set(&person)
//         .get_result(connection)
// }

// pub fn delete(id: i32, connection: &Conn) -> QueryResult<usize> {
//     diesel::delete(heroes::table.find(id)).execute(connection)
// }
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

impl Hero {
    fn from_person(hero: Hero) -> Hero {
        Hero {
            id: hero.id,
            name: hero.name,
            identity: hero.identity,
            hometown: hero.hometown,
            age: hero.age,
        }
    }
}
