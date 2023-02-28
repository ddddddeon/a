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
    let mut headers = HeaderMap::new();
    let mut auth = String::from("Bearer ");
    let api_key = &env::var("OPENAI_API_KEY").unwrap();
    auth.push_str(api_key);

    let header = HeaderValue::from_str(auth.as_str()).unwrap();
    headers.insert("Authorization", header);
    headers.insert("Content-Type", HeaderValue::from_str("application/json").unwrap());

    let max_tokens = 4097 - prompt.len() as i32;

    let p = format!("{{ \"model\": \"text-davinci-003\", \"prompt\": \"{prompt}\", \"temperature\": 0.2, \"max_tokens\": {max_tokens} }}");

    let client = reqwest::blocking::Client::new();
    let mut res = client.post(url)
        .body(p)
        .headers(headers)
        .send()?;


    let mut body = String::new();
    res.read_to_string(&mut body)?;

    let json_object: Value = from_str(&body)?;
    let answer = json_object.get("choices")
        .and_then(|value| value.get(0))
        .and_then(|value| value.get("text"))
        .and_then(|value| value.as_str())
        .unwrap(); // TODO handle

    Ok(String::from(answer))
}
