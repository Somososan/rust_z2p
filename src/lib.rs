use actix_web::dev::Server;
use sqlx::PgPool;
use std::net::TcpListener;

pub mod configuration;
pub mod domain;
pub mod routes;
pub mod startup;
pub mod telemetry;

pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    startup::start(listener, db_pool)
}
