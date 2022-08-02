use rocket::{get, routes};

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
