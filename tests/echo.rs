use reqwest::Client;
mod support;

#[tokio::test]
async fn test_echo_route() {
    let address = support::setup::spawn_app().await.address;

    let client = Client::new();
    let response = client
        .get(format!("{}/echo/HelloWorld", address))
        .send()
        .await
        .unwrap();

    assert!(response.status().is_success());

    let body = response.text().await.unwrap();
    assert_eq!(body, "Hello World!");
}
