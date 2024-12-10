use actix_web::{get, HttpResponse};

#[get("/health_check")]
pub async fn check() -> HttpResponse {
    HttpResponse::Ok().finish()
}
