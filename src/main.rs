use pixel_land_backend::startup;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // here, we declare and instantiate any application general stuff, like database urls, config choices
    let listener = TcpListener::bind("0.0.0.0:8080")?;

    // then we create and run our actix web server
    startup::run(listener)?.await?;
    Ok(())
}
