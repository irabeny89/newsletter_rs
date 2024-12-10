use dotenvy::dotenv;
use std::env;

pub fn get_db_url() -> String {
    dotenv().expect("Failed to load .env file");

    let default_url = String::from("postgres://postgres@localhost/newsletter");

    env::var("DATABASE_URL").unwrap_or(default_url)
}

pub fn get_app_port() -> String {
    dotenv().expect("Failed to load .env file");
    let default_port = "5000".to_owned();

    env::var("PORT").unwrap_or(default_port)
}
