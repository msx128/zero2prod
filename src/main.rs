use secrecy::ExposeSecret;
use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

// added comment

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let configuration =
        get_configuration().expect("Failed to read configuration.");
    let database_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL not set");
    println!("Connecting to: {}", database_url);
    println!(
        "CONNECTION TO: {}",
        &configuration.database.connection_string().expose_secret()
    );

    let subscriber = get_subscriber(
        "zero2prod".into(),
        "info".into(),
        std::io::stdout,
    );
    init_subscriber(subscriber);

    let connection_pool = PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_lazy(
            &configuration
                .database
                .connection_string()
                .expose_secret(),
        )
        .expect("Failed to create connection pool");
    let address = format!(
        "{}:{}",
        configuration.application.host,
        configuration.application.port
    );
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await?;
    Ok(())
}
