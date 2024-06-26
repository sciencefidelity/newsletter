use newsletter::{get_configuration, get_subscriber, init_subscriber, Application};
use std::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    let subscriber = get_subscriber("newsletter", "info", std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration.");
    let application = Application::build(configuration)?;
    application.run_until_stopped().await?;
    Ok(())
}
