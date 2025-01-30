use reqwest::Client;
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize)]
struct OpenAIObject {
    role: String,
    content: String,
}

#[derive(Serialize)]
struct OpenAIRequest {
    model: String,
    messages: Vec<OpenAIObject>,
    max_tokens: u32,
}

#[derive(Deserialize, Debug)]
struct OpenAIResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize, Debug)]
struct Choice {
    message: Message,
}

#[derive(Deserialize, Debug)]
struct Message {
    role: String,
    content: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    // Use environment variable for API key
    let api_key = std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");
    let client = Client::new();

    // Create the messages
    let messages = vec![
        OpenAIObject {
            role: "system".to_string(),
            content: "You are a helpful assistant.".to_string(),
        },
        OpenAIObject {
            role: "user".to_string(),
            content: "Give me a simple example of how to handle HTTP requests in Rust.".to_string(),
        },
    ];

    println!("API Key {}", api_key);

    // Define the request payload
    let request_body = OpenAIRequest {
        model: "gpt-4".to_string(), // Use the desired model
        messages: messages,
        max_tokens: 100,
    };

    // Send the request to OpenAI's API
    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&request_body)
        .send()
        .await?;

    // Check if the request was successful
    if response.status().is_success() {
        let response_body: OpenAIResponse = response.json().await?;
        if let Some(choice) = response_body.choices.first() {
            println!("Response: {}", choice.message.content);
        } else {
            println!("No response from OpenAI.");
        }
    } else {
        println!("Error: {}", response.status());
        let error_text = response.text().await?;
        println!("Details: {}", error_text);
    }

    Ok(())
}