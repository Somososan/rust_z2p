use zero2prod::configuration::get_configuration;
use zero2prod::startup::Application;
use zero2prod::telemetry;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subsciber = telemetry::get_subscriber("zero2prod".into(), "info", std::io::stdout);
    telemetry::init_subscriber(subsciber);

    let config = get_configuration().expect("Failed to retrieve configuration");
    let app = Application::build(config).await?;
    app.run_until_stopped().await?;
    Ok(())
}
