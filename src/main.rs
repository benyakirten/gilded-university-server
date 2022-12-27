use dotenvy::dotenv;

pub mod auth;
pub mod models;
// pub mod schema;

fn main() {
    dotenv().ok();
    println!("Hello, world!");
}
