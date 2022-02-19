use crate::cubes::{CubeColor, CubeData};
use actix_web::web::Data;
use actix_web::HttpResponse;
use parking_lot::RwLock;
use serde::Serialize;

/// Return an HttpResponse 200
pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

//
pub async fn fetch_cubes(rw_cube_arr: Data<RwLock<CubeData>>) -> HttpResponse {
    let cube_arr = &*rw_cube_arr.read();
    match serde_json::to_string(&cube_arr) {
        Ok(serialized) => HttpResponse::Ok().body(serialized),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
