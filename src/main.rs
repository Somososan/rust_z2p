use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::email_client::EmailClient;
use zero2prod::run;
use zero2prod::telemetry;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subsciber = telemetry::get_subscriber("zero2prod".into(), "info", std::io::stdout);
    telemetry::init_subscriber(subsciber);

    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(configuration.database.with_db());

    let sender_email = configuration
        .email_client
        .sender()
        .expect("Invalid sender email address");
    let timeout = configuration.email_client.timeout();
    let email_client = EmailClient::new(
        configuration.email_client.base_url,
        sender_email,
        configuration.email_client.authorization_token.clone(),
        timeout,
    );

    let listener = TcpListener::bind(configuration.application.address())
        .expect("Failed to allocate local port");
    run(listener, connection_pool, email_client)?.await
}
