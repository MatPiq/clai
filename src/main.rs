use clap::Parser;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use serde::Deserialize;
use serde_json::json;

#[derive(Parser, Default, Debug)]
#[clap(
    author = "Matias Piqueras",
    version,
    about = "Simple CLI tool to prompt and get answers from OpenAI's language models"
)]
struct Cli {
    #[clap(forbid_empty_values = true)]
    /// The prompt to generate completion for
    prompt: String,
    /// The model to use
    #[clap(short, long, default_value = "text-davinci-002")]
    model: String,
    /// Max number of tokens to return
    #[clap(short='n', long, default_value = "200")]
    max_tokens: usize,
    /// Higher values mean model will take
    // more risk.
    #[clap(short, long, default_value = "0.75")]
    temperature: f32,
}

// Struct for parsing the text key from the json response
#[derive(Deserialize)]
struct TextCompletion {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    text: String,
}

#[tokio::main]
async fn main() {
    // get baerer from system environment
    let bearer = std::env::var("OPENAI_BEARER").expect("OPENAI_BEARER not set");

    let args = Cli::parse();

    // Generate payload from args
    let payload = json!({
        "prompt": args.prompt,
        "model": args.model,
        "max_tokens": args.max_tokens,
        "temperature": args.temperature,
    });

    let client = reqwest::Client::new();
    let res = client
        .post("https://api.openai.com/v1/completions")
        .json(&payload)
        .header(AUTHORIZATION, format!("Bearer {}", bearer))
        .header(CONTENT_TYPE, "application/json")
        .send()
        .await;

    let text_completion = res.unwrap().json::<TextCompletion>().await.unwrap();
    let text = &text_completion.choices[0].text;
    // remove lines with just whitespace
    let text = text.split('\n').filter(|s| !s.trim().is_empty()).collect::<Vec<&str>>().join("\n");

    println!("\u{1F914}: {}", args.prompt);
    println!("\u{1F680}: {}",  text);
}
