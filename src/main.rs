use api::{
    configurations::{get_config, Settings},
    startup::app,
};
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let config = get_config().expect("Failed to read configuration.");
    let Settings {
        application_port, ..
    } = config;
    let address = format!("127.0.0.1:{application_port}");
    println!("Listening on {}", &address);
    let listener = TcpListener::bind(address).expect("Failed to bind random port");
    app(listener)?.await
}
