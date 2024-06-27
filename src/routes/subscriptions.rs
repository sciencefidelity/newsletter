use crate::{ApplicationBaseUrl, EmailClient, NewSubscriber, SubscriberEmail, SubscriberName};
use actix_web::{web, HttpResponse};
use chrono::Utc;
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use serde::Deserialize;
use sqlx::{Executor, PgPool, Postgres, Transaction};
use tracing::instrument;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct FormData {
    pub email: String,
    pub name: String,
}

impl TryFrom<FormData> for NewSubscriber {
    type Error = String;

    fn try_from(value: FormData) -> Result<Self, Self::Error> {
        let name = SubscriberName::parse(value.name)?;
        let email = SubscriberEmail::parse(value.email)?;
        Ok(Self { email, name })
    }
}

#[instrument(
    name = "Adding a new subscriber", 
    skip(form, pool),
    fields(subscriber_email = %form.email, subscriber_name = %form.name)
)]
pub async fn subscribe(
    form: web::Form<FormData>,
    pool: web::Data<PgPool>,
    email_client: web::Data<EmailClient>,
    base_url: web::Data<ApplicationBaseUrl>,
) -> HttpResponse {
    let Ok(new_subscriber) = form.0.try_into() else {
        return HttpResponse::BadRequest().finish();
    };
    let Ok(mut transaction) = pool.begin().await else {
        return HttpResponse::InternalServerError().finish();
    };
    let Ok(subscriber_id) = insert_subscriber(&mut transaction, &new_subscriber).await else {
        return HttpResponse::InternalServerError().finish();
    };
    let subscription_token = generate_subscription_token();
    if store_token(&mut transaction, subscriber_id, &subscription_token)
        .await
        .is_err()
    {
        return HttpResponse::InternalServerError().finish();
    }
    if transaction.commit().await.is_err() {
        return HttpResponse::InternalServerError().finish();
    }
    if send_confirmation_email(
        &email_client,
        new_subscriber,
        &base_url.0,
        &subscription_token,
    )
    .await
    .is_err()
    {
        return HttpResponse::InternalServerError().finish();
    }
    HttpResponse::Ok().finish()
}

/// Generate a random 25-character-long case-sensitive subscription token.
fn generate_subscription_token() -> String {
    let mut rng = thread_rng();
    std::iter::repeat_with(|| rng.sample(Alphanumeric))
        .map(char::from)
        .take(25)
        .collect()
}

#[instrument(
    name = "Send a confirmation email to a new subscriber",
    skip(email_client, new_subscriber, base_url, subscription_token)
)]
pub async fn send_confirmation_email(
    email_client: &EmailClient,
    new_subscriber: NewSubscriber,
    base_url: &str,
    subscription_token: &str,
) -> Result<(), reqwest::Error> {
    let confirmation_link =
        format!("{base_url}/subscriptions/confirm?subscription_token={subscription_token}",);
    let plain_body = format!(
        "Welcome to our newsletter!\nVisit {confirmation_link} to confirm your subscription.",
    );
    let html_body = format!(
        "Welcome to our newsletter!<br />\
                Click <href=\"{confirmation_link}\">here</a>a to confirm your subscription."
    );
    email_client
        .send_email(new_subscriber.email, "Welcome!", &html_body, &plain_body)
        .await
}

#[instrument(
    name = "Saving new subscriber detais in the database",
    skip(new_subscriber, transaction)
)]
pub async fn insert_subscriber(
    transaction: &mut Transaction<'_, Postgres>,
    new_subscriber: &NewSubscriber,
) -> Result<Uuid, sqlx::Error> {
    let subscriber_id = Uuid::new_v4();
    let query = sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at, status)
        VALUES ($1, $2, $3, $4, 'pending_confirmation')
        "#,
        subscriber_id,
        new_subscriber.email.as_ref(),
        new_subscriber.name.as_ref(),
        Utc::now()
    );
    transaction.execute(query).await.map_err(|e| {
        tracing::error!("Failed to execute query: {e:?}.");
        e
    })?;

    Ok(subscriber_id)
}

#[instrument(
    name = "Store subscription token in the database",
    skip(subscription_token, transaction)
)]
pub async fn store_token(
    transaction: &mut Transaction<'_, Postgres>,
    subscriber_id: Uuid,
    subscription_token: &str,
) -> Result<(), sqlx::Error> {
    let query = sqlx::query!(
        r#"INSERT INTO subscription_tokens (subscription_token, subscriber_id)
        VALUES ($1, $2)"#,
        subscription_token,
        subscriber_id
    );
    transaction.execute(query).await.map_err(|e| {
        tracing::error!("Failed to execute query: {e:?}");
        e
    })?;

    Ok(())
}
