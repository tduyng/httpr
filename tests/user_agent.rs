use reqwest::Client;
mod support;

#[tokio::test]
async fn test_user_agent_route() {
    let address = support::setup::spawn_app().await.address;

    let client = Client::new();
    let response = client
        .get(format!("{}/user_agent", address))
        .header("User-Agent", "TestAgent/1.0")
        .send()
        .await
        .unwrap();

    assert!(response.status().is_success());

    let body = response.text().await.unwrap();
    assert_eq!(body, "TestAgent/1.0");
}
