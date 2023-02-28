use reqwest::{
    blocking::Client,
    header::{HeaderMap, HeaderValue},
};
use serde::{Deserialize, Serialize};
use serde_json::{from_str, Value};
use std::env;
use std::io::Read;

const MAX_TOKENS: u32 = 4097;

#[derive(Serialize, Deserialize)]
struct Prompt {
    model: String,
    prompt: String,
    temperature: f32,
    max_tokens: u32,
}

pub fn make_request(url: String, prompt: String) -> Result<String, Box<dyn std::error::Error>> {
    let prompt_length = prompt.len() as u32;
    if prompt_length >= MAX_TOKENS {
        return Err(format!("Prompt cannot exceed length of {} characters", MAX_TOKENS - 1).into());
    }

    let p = Prompt {
        max_tokens: MAX_TOKENS - prompt_length,
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
