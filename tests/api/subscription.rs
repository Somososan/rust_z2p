use crate::common::spawn_app;
use claim::assert_err;

#[tokio::test]
async fn subscribe_returns_200_on_valid_form_data() {
    let app = spawn_app().await;
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = app.post_subscriptions(body.to_string()).await;

    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved subscription.");

    assert_eq!(saved.email, "ursula_le_guin@gmail.com");
    assert_eq!(saved.name, "le guin");
}

#[tokio::test]
async fn subscribe_returns_400_on_form_data_missing() {
    let app = spawn_app().await;

    let test_cases = vec![
        ("name=le%20guin", "missing email"),
        ("email=ursula_le_guin%40gmail.com", "missing name"),
        ("", "both missing"),
    ];
    for (body, msg) in test_cases.iter() {
        assert_err!(
            app.post_subscriptions(body.to_string())
                .await
                .error_for_status()
                .map_err(|_| ()),
            "Did not fail on case: {:?}",
            msg
        )
    }
}

#[tokio::test]
async fn subscribe_returns_400_when_fields_are_present_but_invalid() {
    let app = spawn_app().await;
    let body = "name=le%{}20guin&email=ursula_le_guin%40gmail.com";
    assert_err!(
        app.post_subscriptions(body.to_string())
            .await
            .error_for_status()
            .map_err(|_| ()),
        "The API did not return a 400 BAD REQUEST when the payload was: {}",
        body
    );
}
