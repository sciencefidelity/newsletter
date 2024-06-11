use newsletter::{get_configuration, get_subscriber, init_subscriber, run};
use sqlx::postgres::PgPoolOptions;
use std::{io, net::TcpListener};

#[tokio::main]
async fn main() -> io::Result<()> {
    let subscriber = get_subscriber("newsletter", "info", std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPoolOptions::new().connect_lazy_with(configuration.database.with_db());
    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );

    let address = TcpListener::bind(address)?;
    run(address, connection_pool)?.await?;
    Ok(())
}
