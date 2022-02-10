use actix_web::dev::Server;
use email_client::EmailClient;
use sqlx::PgPool;
use std::net::TcpListener;

pub mod configuration;
pub mod domain;
pub mod email_client;
pub mod routes;
pub mod startup;
pub mod telemetry;

pub fn run(
    listener: TcpListener,
    db_pool: PgPool,
    email_client: EmailClient,
) -> Result<Server, std::io::Error> {
    startup::start(listener, db_pool, email_client)
}
