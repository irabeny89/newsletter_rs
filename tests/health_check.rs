// launch server in the background
async fn spawn_app() {
    let server = newsletter_rs::run().expect("Failed to bind address");
    tokio::spawn(server);
}

#[tokio::test]
async fn health_check_works() {
    // => Arrange
    // spawn server app in background
    spawn_app();
    // setup http request client
    let client = reqwest::Client::new();
    // => Act
    // send request to test the endpoint
    let response = client
        .get("http://127.0.0.1:8000/health_check")
        .send()
        .await
        .expect("Failed to execute http request");

    // => Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
