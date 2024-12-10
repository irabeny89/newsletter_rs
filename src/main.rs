use newsletter_rs::{startup, utils};
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let address = format!("127.0.0.1:{}", utils::get_app_port());
    let listener = TcpListener::bind(address).expect("Failed to bind random port");

    startup::run(listener)?.await // expect bind error to bubble on error here
}
