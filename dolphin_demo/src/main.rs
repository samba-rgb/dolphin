use dolphin::Entity;

#[derive(Entity)]
#[EntityName = "users"]
struct User {
    id: i32,
    name: String,
}

fn main() {
    println!("Entity name: {}", User::table_name());
}
