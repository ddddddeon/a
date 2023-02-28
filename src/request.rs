use std::env;
use std::io::{Read};
use reqwest::{blocking::Client, header::{HeaderMap, HeaderValue}};
use serde_json::{Value, from_str};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Prompt {
    model: String,
    prompt: String,
    temperature: f32,
    max_tokens: u32
}

pub fn make_request(url: String, prompt: String) -> Result<String, Box<dyn std::error::Error>> {
    let p = Prompt {
        max_tokens: 4097 - (prompt.len() as u32),
        model: String::from("text-davinci-003"),
        prompt: prompt,
        temperature: 0.2,
    };

    let api_key = &env::var("OPENAI_API_KEY")?;
    let mut auth = String::from("Bearer ");
    auth.push_str(api_key);

    let mut headers = HeaderMap::new();
    headers.insert("Authorization", HeaderValue::from_str(auth.as_str())?);
    headers.insert("Content-Type", HeaderValue::from_str("application/json")?);

    let body = serde_json::to_string(&p)?;

    let client = Client::new();
    let mut res = client.post(url)
        .body(body)
        .headers(headers)
        .send()?;

    let mut response_body = String::new();
    res.read_to_string(&mut response_body)?;
    let json_object: Value = from_str(&response_body)?;
    let answer = json_object["choices"][0]["text"].as_str();

    match answer {
        Some(a) => Ok(String::from(a)),
        None => Err("JSON parse error".into()),
    }
 }
