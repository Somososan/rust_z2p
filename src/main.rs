use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to database");

    let addres = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(addres).expect("Failed to allocate local port");
    run(listener, connection_pool)?.await
}
