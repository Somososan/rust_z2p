use crate::domain::{NewSubscriber, SubscriberEmail, SubscriberName};
use crate::email_client::EmailClient;
use actix_web::web;
use actix_web::HttpResponse;
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    pub name: String,
    pub email: String,
}

impl TryFrom<FormData> for NewSubscriber {
    type Error = String;

    fn try_from(value: FormData) -> Result<Self, Self::Error> {
        let name = SubscriberName::parse(value.name)?;
        let email = SubscriberEmail::parse(value.email)?;

        Ok(Self { name, email })
    }
}

#[allow(clippy::async_yields_async)]
#[tracing::instrument(
    name="Adding a new subsciber",
    skip(form,pool,email_client),
    fields(
        subscriber_email=%form.email,
        subscriber_name=%form.name
    )
)]
pub async fn subscribe(
    form: web::Form<FormData>,
    pool: web::Data<PgPool>,
    email_client: web::Data<EmailClient>,
) -> HttpResponse {
    let subscriber = match NewSubscriber::try_from(form.0) {
        Ok(x) => x,
        Err(e) => {
            tracing::error!("Failed to parse formdata: {:?}", e);
            return HttpResponse::BadRequest().finish();
        }
    };
    if let Err(e) = insert_subscriber(&subscriber, &pool).await {
        tracing::error!("Failed to execute query: {:?}", e);
        return HttpResponse::InternalServerError().finish();
    } else {
    };

    if email_client
        .send_email(subscriber.email, "Welcome", "todo", "todo")
        .await
        .is_err()
    {
        tracing::error!("Failed to send mail");
        return HttpResponse::InternalServerError().finish();
    };
    HttpResponse::Ok().finish()
}

#[tracing::instrument(
    name = "Saving new subscriber details in the database",
    skip(subsciber, pool)
)]
pub async fn insert_subscriber(
    subsciber: &NewSubscriber,
    pool: &PgPool,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at,status)
        VALUES ($1, $2, $3, $4,'confirmed')   
        "#,
        Uuid::new_v4(),
        subsciber.email.as_ref(),
        subsciber.name.as_ref(),
        Utc::now()
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;
    Ok(())
}
