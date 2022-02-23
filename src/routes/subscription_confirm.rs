use actix_web::web::{Data, Query};
use actix_web::HttpResponse;
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;
use validator::{Validate, ValidationError};

pub const SUBSCRIPTION_TOKEN_LENGTH: usize = 25;

#[derive(Debug, Validate, Deserialize)]
pub struct Parameters {
    #[validate(
        length(equal = "SUBSCRIPTION_TOKEN_LENGTH"),
        custom = "validate_subscription_token"
    )]
    subscription_token: String,
}

#[allow(clippy::async_yields_async)]
#[tracing::instrument(name = "confirm pending subscriber", skip(param, pool))]
pub async fn confirm(pool: Data<PgPool>, param: Query<Parameters>) -> HttpResponse {
    if param.validate().is_err() {
        return HttpResponse::BadRequest().finish();
    }

    let id = match get_subscriber_id_from_token(&pool, &param.subscription_token).await {
        Ok(id) => id,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };
    match id {
        None => HttpResponse::Unauthorized().finish(),
        Some(subscriber_id) => {
            if confirm_subscriber(&pool, subscriber_id).await.is_err() {
                return HttpResponse::InternalServerError().finish();
            };
            HttpResponse::Ok().finish()
        }
    }
}
#[allow(clippy::async_yields_async)]
#[tracing::instrument(name = "Mark subscriber as confirmed", skip(subscriber_id, pool))]
async fn confirm_subscriber(pool: &PgPool, subscriber_id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"UPDATE subscriptions SET status = 'confirmed' WHERE id = $1"#,
        subscriber_id,
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;
    Ok(())
}

#[tracing::instrument(name = "Get subscriber_id from token", skip(subscription_token, pool))]
async fn get_subscriber_id_from_token(
    pool: &PgPool,
    subscription_token: &str,
) -> Result<Option<Uuid>, sqlx::Error> {
    let result = sqlx::query!(
        r#"SELECT subscriber_id FROM subscription_tokens WHERE subscription_token = $1"#,
        subscription_token,
    )
    .fetch_optional(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;
    Ok(result.map(|r| r.subscriber_id))
}

#[tracing::instrument(name = "Validate subscription token", skip(token))]
fn validate_subscription_token(token: &str) -> Result<(), ValidationError> {
    if token.chars().any(|c| !c.is_ascii_alphanumeric()) {
        tracing::error!("Invalid subscription token received: {:?}", token);
        return Err(ValidationError::new(
            "Subscription token contains non ascii alphanumeric char",
        ));
    };
    Ok(())
}

#[cfg(test)]
mod tests {
    use claim::{assert_err, assert_ok};
    use fake::{Fake, StringFaker};
    use validator::Validate;

    use super::Parameters;

    fn valid_string(len: usize) -> String {
        const ASCII_ALPHANNUMERIC: &str =
            "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
        StringFaker::with(Vec::from(ASCII_ALPHANNUMERIC), len).fake()
    }

    fn invalid_string(len: usize) -> String {
        const NOT_ASCII_ALPHANNUMERIC: &str = "!\"#$%&\'()*+,-./:;<=>?@";
        StringFaker::with(Vec::from(NOT_ASCII_ALPHANNUMERIC), len).fake()
    }

    #[test]
    fn subcription_token_with_valid_length_and_charachters() {
        assert_ok!(Parameters {
            subscription_token: valid_string(super::SUBSCRIPTION_TOKEN_LENGTH),
        }
        .validate());
    }

    #[test]
    fn subcription_token_with_valid_charachters_and_wrong_length() {
        assert_err!(Parameters {
            subscription_token: valid_string(super::SUBSCRIPTION_TOKEN_LENGTH + 1),
        }
        .validate());
    }

    #[test]
    fn subcription_token_with_invalid_charachters_and_correct_length() {
        assert_err!(Parameters {
            subscription_token: invalid_string(super::SUBSCRIPTION_TOKEN_LENGTH),
        }
        .validate());
    }

    #[test]
    fn subcription_token_with_both_invalid_charachters_and_length() {
        assert_err!(Parameters {
            subscription_token: invalid_string(super::SUBSCRIPTION_TOKEN_LENGTH + 1),
        }
        .validate());
    }
}
