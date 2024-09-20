

use super::*;

#[test]
fn test_api_instance() {
    let api_instance = api::Instance::new();
    assert!(api_instance.is_ok());
}

#[test]
fn test_read_words_from_file() {
    let to_link = read_words_from_file(PathBuf::from("./examples/link.txt"));
    assert!(to_link.is_ok());
    let to_avoid = read_words_from_file(PathBuf::from("./examples/avoid.txt"));
    assert!(to_avoid.is_ok());
}

#[tokio::test]
async fn test_language_models_call() {
    let api_instance = api::Instance::new();
    assert!(api_instance.is_ok());

    let to_link = vec![
        "bond".to_string(),
        "sound".to_string(),
        "park".to_string(),
        "penny".to_string(),
        "bee".to_string(),
        "tokyo".to_string(),
        "walrus".to_string(),
        "hospital".to_string(),
        "scuba diver".to_string(),
    ];

    let to_avoid = vec![
        "angel".to_string(),
        "ski".to_string(),
        "captain".to_string(),
        "bass".to_string(),
        "boil".to_string(),
        "casino".to_string(),
        "star".to_string(),
        "fish".to_string(),
        "blind".to_string(),
        "day".to_string(),
        "tip".to_string(),
        "goldilocks".to_string(),
        "field".to_string(),
        "file".to_string(),
        "cotton".to_string(),
        "scarecrow".to_string(),
        "extra virgin olive oil".to_string(),
    ];

    let models = api_instance
        .unwrap()
        .fetch_clue_collection(to_link, to_avoid)
        .await;

    assert!(models.is_ok());
}
