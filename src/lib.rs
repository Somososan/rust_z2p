use actix_web::dev::Server;
use std::net::TcpListener;

pub mod configuration;
pub mod routes;
pub mod startup;

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    startup::start(listener)
}
