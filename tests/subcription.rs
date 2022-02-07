mod common;

#[tokio::test]
async fn subscribe_returns_200_on_valid_form_data() {
    let (address, db_pool) = common::spawn_app().await;
    let client = reqwest::Client::new();

    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = client
        .post(&format!("{}/subscriptions", &address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to send formdata");

    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&db_pool)
        .await
        .expect("Failed to fetch saved subscription.");

    assert_eq!(saved.email, "ursula_le_guin@gmail.com");
    assert_eq!(saved.name, "le guin");
}

#[tokio::test]
async fn subscribe_returns_400_on_form_data_missing() {
    let (address, _) = common::spawn_app().await;
    let client = reqwest::Client::new();

    let test_cases = vec![
        ("name=le%20guin", "missing email"),
        ("email=ursula_le_guin%40gmail.com", "missing name"),
        ("", "both missing"),
    ];
    for (body, msg) in test_cases.iter() {
        let response = client
            .post(&format!("{}/subscriptions", &address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(*body)
            .send()
            .await
            .expect("Failed to send formdata");

        assert_eq!(
            400,
            response.status().as_u16(),
            "Did not fail on case: {}",
            msg
        );
    }
}

#[tokio::test]
async fn subscribe_returns_400_when_fields_are_present_but_invalid() {
    let (address, _) = common::spawn_app().await;
    let client = reqwest::Client::new();

    let body = "name=le%{}20guin&email=ursula_le_guin%40gmail.com";
    let response = client
        .post(&format!("{}/subscriptions", &address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to send formdata");

    assert_eq!(
        400,
        response.status().as_u16(),
        "The API did not return a 400 BAD REQUEST when the payload was: {}",
        body
    );
}
