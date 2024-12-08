use crate::routes;
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};

use std::net::TcpListener;

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let address = listener.local_addr().unwrap().to_string();
    let server = HttpServer::new(|| {
        App::new()
            .route(
                "/health_check",
                web::get().to(routes::health_check::health_check),
            )
            .route(
                "/subscriptions",
                web::post().to(routes::subscription::subscribe),
            )
    })
    .listen(listener)? //bind error will bubble
    .run();

    println!("Server address: {}", address);

    Ok(server)
}
