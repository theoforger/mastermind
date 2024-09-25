use super::*;
use crate::api::Instance;
use httpmock::prelude::*;

#[test]
fn test_api_instance() {
    let api_instance = Instance::new();
    assert!(api_instance.is_ok());
}

#[test]
fn test_read_words_from_file() {
    let to_link = read_words_from_file(PathBuf::from("examples/link.txt"));
    assert!(to_link.is_ok());
    let to_avoid = read_words_from_file(PathBuf::from("examples/avoid.txt"));
    assert!(to_avoid.is_ok());
}

#[tokio::test]
async fn test_get_models() {
    // Start a lightweight mock server.
    let server = MockServer::start_async().await;

    // Create a mock on the server.
    let mock = server.mock(|when, then| {
        when.method(GET).path("/models");
        then.status(200)
            .header("content-type", "application/json")
            .body_from_file("src/tests/mock_responses/language_models.json");
    });

    // Create an API instance and set the base url to mock server url
    let mut api_instance = Instance::new().unwrap();
    api_instance.set_base_url(server.url("/"));

    // Get response from mock server
    let response = api_instance.get_models().await.unwrap();
    mock.assert();

    // Compare outputs
    let output = response.join("\n");
    let expected_output = fs::read_to_string("src/tests/expected_outputs/language_models.txt").unwrap();
    assert_eq!(output, expected_output);
}

#[tokio::test]
async fn test_post_chat_completions() {
    // Start a lightweight mock server.
    let server = MockServer::start_async().await;

    // Create a mock on the server.
    let mock = server.mock(|when, then| {
        when.method(POST).path("/chat/completions");
        then.status(200)
            .header("content-type", "application/json")
            .body_from_file("src/tests/mock_responses/chat_completions.json");
    });

    // Create an API instance and set the base url to mock server url
    let mut api_instance = Instance::new().unwrap();
    api_instance.set_base_url(server.url("/"));

    // Get response from mock server
    let response = api_instance
        .post_chat_completions(vec![], vec![])
        .await
        .unwrap();
    mock.assert();

    // Compare outputs
    let output = response.generate_raw_list();
    let expected_output =
        fs::read_to_string("src/tests/expected_outputs/chat_completions.txt").unwrap();
    assert_eq!(output, expected_output);
}
