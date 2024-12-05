use std::net::TcpListener;

// launch server in the background
fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let address = listener.local_addr().unwrap();

    let server = newsletter_rs::run(listener).expect("Failed to bind address");
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
        .get(format!("{}/health_check", address))
        .send()
        .await
        .expect("Failed to execute http request");

    // => Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
