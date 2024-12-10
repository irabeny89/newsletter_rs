use crate::routes::{health_check, subscription};
use actix_web::dev::Server;
use actix_web::{middleware::Logger, web, App, HttpServer};

use std::net::TcpListener;

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let address = listener.local_addr().unwrap().to_string();
    let server = HttpServer::new(|| {
        App::new().wrap(Logger::default()).service(
            web::scope("/api")
                .service(health_check::check)
                .service(subscription::subscribe),
        )
    })
    .listen(listener)? //bind error will bubble
    .run();

    println!("Server address: {}", address);

    Ok(server)
}
