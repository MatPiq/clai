use clap::Parser;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use serde::Deserialize;
use serde_json::json;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about=None)]
struct Cli {
    // The prompt to generate completion for
    #[clap(short, long)]
    prompt: String,
    // The model to use
    #[clap(short = 'm', long = "model", default_value = "text-davinci-002")]
    model: String,
    // Max number of tokens to return
    #[clap(short = 'n', long = "num-tokens", default_value = "200")]
    max_tokens: usize,
    // Higher values mean model will take
    // more risk.
    #[clap(short = 't', long = "temperature", default_value = "0.75")]
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

    // print the text
    println!("Prompt: {}", args.prompt);
    println!("Answer: {}", text);
}
