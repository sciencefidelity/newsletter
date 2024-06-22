pub mod configuration;
pub use configuration::{get_configuration, DatabaseSettings};

pub mod domain;
pub use domain::{NewSubscriber, SubscriberEmail, SubscriberName};

pub mod routes;

pub mod startup;
pub use startup::run;

pub mod telemetry;
pub use telemetry::{get_subscriber, init_subscriber};
