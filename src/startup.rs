use crate::cubes::{CubeColor, CubeData};
use crate::routes::{fetch_cubes, health_check};
use actix_web::dev::Server;
use actix_web::web::{route, Data};
use actix_web::{web, App, HttpServer};
use parking_lot::RwLock;
use std::net::TcpListener;

pub fn run(
    listener: TcpListener,
    rw_cube_data: RwLock<CubeData>,
) -> Result<Server, std::io::Error> {
    let rwlock_cube_arr = Data::new(rw_cube_data);
    let server = HttpServer::new(move || {
        App::new()
            .app_data(Data::clone(&rwlock_cube_arr))
            .route("/health_check", web::get().to(health_check))
            .route("/fetch_cubes", web::get().to(fetch_cubes))
    })
    .listen(listener)?
    .run();
    Ok(server)
}
