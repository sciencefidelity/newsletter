use crate::configuration::Settings;
use crate::routes::{confirm, health_check, publish_newsletter, subscribe};
use crate::{DatabaseSettings, EmailClient};
use actix_web::{dev::Server, web, App, HttpServer};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::{io, net::TcpListener};
use tracing_actix_web::TracingLogger;

pub struct Application {
    port: u16,
    server: Server,
}

impl Application {
    /// # Errors
    ///
    /// Will return `Err` if the `run` function returns an error.
    ///
    /// # Panics
    ///
    /// Will panic if the email client is instantiated with an invalid email address.
    pub fn build(configuration: Settings) -> io::Result<Self> {
        let connection_pool = get_connection_pool(&configuration.database);

        let sender_email = configuration
            .email_client
            .sender()
            .expect("invalid sender email address");

        let timeout = configuration.email_client.timeout();
        let email_client = EmailClient::new(
            configuration.email_client.base_url,
            sender_email,
            configuration.email_client.authorization_token,
            timeout,
        );

        let address = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        );

        let listener = TcpListener::bind(address)?;
        let port = listener
            .local_addr()
            .expect("listener failed to return port")
            .port();
        let server = run(
            listener,
            connection_pool,
            email_client,
            configuration.application.base_url,
        )?;

        Ok(Self { port, server })
    }

    #[must_use]
    pub const fn port(&self) -> u16 {
        self.port
    }

    /// # Errors
    ///
    /// Will return `Err` if the server fails to start.
    pub async fn run_until_stopped(self) -> io::Result<()> {
        self.server.await
    }
}

#[must_use]
pub fn get_connection_pool(configuration: &DatabaseSettings) -> PgPool {
    PgPoolOptions::new().connect_lazy_with(configuration.with_db())
}

#[derive(Debug)]
pub struct ApplicationBaseUrl(pub String);

/// # Errors
///
/// Will return `Err` if the `actix_web` server fails to start
pub fn run(
    listener: TcpListener,
    db_pool: PgPool,
    email_client: EmailClient,
    base_url: String,
) -> io::Result<Server> {
    let db_pool = web::Data::new(db_pool);
    let email_client = web::Data::new(email_client);

    let base_url = web::Data::new(ApplicationBaseUrl(base_url));
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .route("/health-check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .route("/subscriptions/confirm", web::get().to(confirm))
            .route("/newsletters", web::post().to(publish_newsletter))
            .app_data(db_pool.clone())
            .app_data(email_client.clone())
            .app_data(base_url.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
