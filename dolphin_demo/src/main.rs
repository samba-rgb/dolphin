extern crate dolphin;

use dolphin::{get_db_strategy, DbBackend, Entity};


#[derive(Entity)]
#[EntityName("users")]
#[DbType("SQL")]
struct User {
    id: i32,
    name: String,
}

fn main() {
    let query =  get_db_strategy(DbBackend::from_str("SQL"));  
    println!("{}", query.find());
    println!("Entity name: {}", User::entity_name().to_string());
    println!("SQL: {}", User::find_by_id(1).to_string());

    println!("Entity name: {}", User::entity_name().to_string());

    println!("SQL: {}", User::find_by_id(1).to_string());

    println!("Database backend: {}", DbBackend::Mongo.as_str().to_string());
    println!("Database backend: {}", DbBackend::Sql.as_str().to_string());
    

}
