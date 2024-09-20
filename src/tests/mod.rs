use super::*;
use crate::api::Instance;
use httpmock::prelude::*;

#[test]
fn test_api_instance() {
    let api_instance = api::Instance::new();
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
async fn test_fetch_language_models() {
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
    let response = api_instance.fetch_all_model_ids().await.unwrap();
    mock.assert();

    // Compare results
    let expected_response = [
        "distil-whisper-large-v3-en",
        "gemma-7b-it",
        "gemma2-9b-it",
        "llama-3.1-70b-versatile",
        "llama-3.1-8b-instant",
        "llama-guard-3-8b",
        "llama3-70b-8192",
        "llama3-8b-8192",
        "llama3-groq-70b-8192-tool-use-preview",
        "llama3-groq-8b-8192-tool-use-preview",
        "llava-v1.5-7b-4096-preview",
        "mixtral-8x7b-32768",
        "whisper-large-v3",
    ];
    assert_eq!(response, expected_response);
}
