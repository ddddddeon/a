use std::env;
use std::io::{self, Read};
use http::{HeaderMap, HeaderValue};
use reqwest;
use serde_json::{Value, from_str};

// let p = Prompt {
//         model: String::from("text-davinci-003"),
//         prompt: prompt,
//         temperature: 0.2,
//         max_tokens: 4097 - (prompt.len() as i32),
//     };

pub fn make_request(url: String, prompt: String) -> Result<String, Box<dyn std::error::Error>> {
    let api_key = &env::var("OPENAI_API_KEY")?;
    let max_tokens = 4097 - prompt.len() as u32; //TODO ??

    let mut auth = String::from("Bearer ");
    auth.push_str(api_key);

    let mut headers = HeaderMap::new();
    headers.insert("Authorization", HeaderValue::from_str(auth.as_str())?);
    headers.insert("Content-Type", HeaderValue::from_str("application/json")?);

    let body = format!("{{ \"model\": \"text-davinci-003\", \"prompt\": \"{prompt}\", \"temperature\": 0.2, \"max_tokens\": {max_tokens} }}");

    let client = reqwest::blocking::Client::new();
    let mut res = client.post(url)
        .body(body)
        .headers(headers)
        .send()?;

    let mut response_body = String::new();
    res.read_to_string(&mut response_body)?;

    let json_object: Value = from_str(&response_body)?;
    let answer = json_object.get("choices")
        .and_then(|value| value.get(0))
        .and_then(|value| value.get("text"))
        .and_then(|value| value.as_str())
        .expect("JSON Parsing error!");

    Ok(String::from(answer))
}
