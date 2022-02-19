use actix_web::HttpResponse;

/// Return an HttpResponse 200
pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}
