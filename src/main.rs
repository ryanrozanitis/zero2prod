//! src/main.rs

use std::net::TcpListener;
use zero2prod::startup::run;
use zero2prod::configuration::get_configuration;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let _configuration = get_configuration().expect("Failed to read configuration.");

    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    //let port = listener.local_addr().unwrap().port();
    run(listener)?.await
}
