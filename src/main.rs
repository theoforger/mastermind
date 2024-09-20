use clap::Parser;

use mastermind::api::Instance;
use mastermind::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read arguments and environment variables
    let args = Args::parse();

    // Create an API instance
    let mut api_instance = Instance::new()?;

    // If -g is set, call the models API endpoint instead
    if args.get {
        println!("{}", api_instance.fetch_all_model_ids().await?.join("\n"));
        return Ok(());
    }

    // If -m is set, use a preferred language model
    if let Some(model_id) = args.model {
        api_instance.set_model_id(model_id).await?;
    }

    // Attempt to read words from the two files
    let link_words = read_words_from_file(args.to_link.unwrap())?;
    let avoid_words = read_words_from_file(args.to_avoid.unwrap())?;

    // Get clues from API
    let clue_collection = api_instance
        .fetch_clue_collection(link_words, avoid_words)
        .await?;


    // Output
    if clue_collection.is_empty() {
        println!("The language model didn't return any useful clues. Maybe try again?");
    } else if let Some(output_path) = args.output {
        println!("Writing to file '{}'...", output_path.display());
        write_content_to_file(output_path, clue_collection.output())?;
    } else {
        clue_collection.display();
    }

    // If -t is set, output the token usage information
    if args.token {
        // Write to stderr in the format: prompt_tokens, completion_tokens, total_tokens
        eprintln!("\nTokens Usage\n----------------------\nPrompt Tokens: {}\nCompletion Tokens: {}\n----------------------\nTotal Tokens: {}",
                  clue_collection.usage.prompt_tokens, clue_collection.usage.completion_tokens, clue_collection.usage.total_tokens);
    }

    Ok(())
}
