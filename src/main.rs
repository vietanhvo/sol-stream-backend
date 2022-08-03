#[macro_use]
extern crate diesel;

mod models;
mod routes;
mod schema;
mod solana;

use diesel::prelude::*;
use dotenv::dotenv;
use rocket::{get, routes};
use std::env;

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

#[rocket::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cors = rocket_cors::CorsOptions::default().to_cors()?;

    rocket::build()
        .mount("/", routes![index, route_with_pubkey])
        .attach(cors)
        .launch()
        .await?;

    Ok(())
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/<pubkey>")]
fn route_with_pubkey(pubkey: &str) -> String {
    format!("Hello {}", pubkey)
}
