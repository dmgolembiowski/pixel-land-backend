use pixel_land_backend::cubes::{CubeColor, CubeData};
use pixel_land_backend::startup;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // here, we declare and instantiate any application general stuff, like database urls, config choices
    let listener = TcpListener::bind("0.0.0.0:8080")?;

    // create our cube array, and wrap it in the fastest read write-lock on earth
    let rw_cube_data = parking_lot::RwLock::new(CubeData::default());

    // then we create and run our actix web server
    startup::run(listener, rw_cube_data)?.await?;

    Ok(())
}
