use secrecy::ExposeSecret;
use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::run;
use zero2prod::telemetry;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subsciber = telemetry::get_subscriber("zero2prod".into(), "info", std::io::stdout);
    telemetry::init_subscriber(subsciber);

    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool =
        PgPool::connect_lazy(configuration.database.connection_string().expose_secret())
            .expect("Failed to connect to database");

    let addres = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(addres).expect("Failed to allocate local port");
    run(listener, connection_pool)?.await
}
