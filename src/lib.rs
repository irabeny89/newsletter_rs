use actix_web::dev::Server;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};
use std::net::TcpListener;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}
async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let address = listener.local_addr().unwrap().to_string();
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
    })
    .listen(listener)? //bind error will bubble
    .run();

    println!("Server address: {}", address);

    Ok(server)
}
