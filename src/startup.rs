use crate::cubes::{CubeColor, Cubes};
use crate::routes::{fetch_cubes, health_check, set_cube};
use actix_cors::Cors;
use actix_web::dev::Server;
use actix_web::web::{route, Data};
use actix_web::{web, App, HttpServer};
use std::net::TcpListener;
use std::sync::Mutex;

pub fn run(listener: TcpListener, rw_cubes: Cubes) -> Result<Server, std::io::Error> {
    let rw_cubes = Data::new(rw_cubes);

    let server = HttpServer::new(move || {
        App::new()
            .app_data(Data::clone(&rw_cubes))
            .wrap(Cors::permissive())
            .route("/health_check", web::get().to(health_check))
            .route("/fetch_cubes", web::get().to(fetch_cubes))
            .route("/set_cube", web::post().to(set_cube))
        //.route("/set_cube_color", web::post().to(set_cube_color))
    })
    .listen(listener)?
    .run();
    Ok(server)
}
