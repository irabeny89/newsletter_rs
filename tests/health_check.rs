use std::net::TcpListener;

use newsletter_rs::startup;

const HEALTH_CHECK_PATH: &str = "/health_check";
const SUBSCRIPTIONS_PATH: &str = "/subscriptions";

// launch server in the background
fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let address = listener.local_addr().unwrap();

    let server = startup::run(listener).expect("Failed to bind address");
    tokio::spawn(server);

    // return server address
    format!("http://{}", address)
}

#[tokio::test]
async fn health_check_works() {
    // => Arrange
    // spawn server app in background & extract address
    let address = spawn_app();

    // setup http request client
    let client = reqwest::Client::new();

    // => Act
    // send request to test the endpoint
    let response = client
        .get(format!("{address}{HEALTH_CHECK_PATH}"))
        .send()
        .await
        .expect("Failed to execute http request");

    // => Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_returns_200_for_valid_form_data() {
    // => Arrange
    // spawn server app in background & extract address
    let address = spawn_app();
    // setup http request client
    let client = reqwest::Client::new();

    // => Act
    let body = "name=irabeny&email=irabeny89@gmail.com";
    // send request to test the endpoint
    let response = client
        .post(format!("{address}{SUBSCRIPTIONS_PATH}"))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute http request");

    // => Assert
    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn subscribe_returns_400_for_missing_data() {
    // => Arrange
    // spawn server app in background & extract address
    let address = spawn_app();
    // setup http request client
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=irabeny", "missing email"),
        ("email=irabeny89@gmail.com", "missing name"),
        ("", "missing both name and email"),
    ];

    for (body, err_msg) in test_cases {
        // => Act
        // send request to test the endpoint
        let response = client
            .post(format!("{address}{SUBSCRIPTIONS_PATH}"))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .expect("Failed to execute http request");

        // => Assert
        assert_eq!(
            response.status().as_u16(),
            400,
            "The API did not fail when {}.",
            err_msg
        );
    }
}
