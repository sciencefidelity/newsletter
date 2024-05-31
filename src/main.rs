use newsletter::{get_configuration, get_subscriber, init_subscriber, run};

use sqlx::PgPool;
use std::{io, net::TcpListener};

#[tokio::main]
async fn main() -> io::Result<()> {
    let subscriber = get_subscriber("newsletter".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    let address = format!("127.0.0.1:{}", configuration.application_port);

    let address = TcpListener::bind(address)?;
    run(address, connection_pool)?.await
}
