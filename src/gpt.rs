use super::util::pretty_print;
use reqwest::{
    blocking::Client,
    header::{HeaderMap, HeaderValue},
};
use serde::{Deserialize, Serialize};
use serde_json::{from_str, Value};
use std::error::Error;
use std::io::Read;

const OPENAI_API_URL: &str = "https://api.openai.com/v1/completions";
const OPENAI_MODEL: &str = "text-davinci-003";
const MAX_TOKENS: u32 = 4097;
const TEMPERATURE: f32 = 0.2;

type BoxResult<T> = Result<T, Box<dyn Error>>;

#[derive(Serialize, Deserialize, Debug)]
struct Prompt {
    model: String,
    prompt: String,
    temperature: f32,
    max_tokens: u32,
}

pub struct GPTClient {
    api_key: String,
    url: String,
}

impl GPTClient {
    pub fn new(api_key: String) -> Self {
        GPTClient {
            api_key,
            url: String::from(OPENAI_API_URL),
        }
    }

    pub fn prompt(&self, prompt: String) -> BoxResult<String> {
        let prompt_length = prompt.len() as u32;
        if prompt_length >= MAX_TOKENS {
            return Err(format!(
                "Prompt cannot exceed length of {} characters",
                MAX_TOKENS - 1
            )
            .into());
        }

        let p = Prompt {
            max_tokens: MAX_TOKENS - prompt_length,
            model: String::from(OPENAI_MODEL),
            prompt,
            temperature: TEMPERATURE,
        };

        let mut auth = String::from("Bearer ");
        auth.push_str(&self.api_key);

        let mut headers = HeaderMap::new();
        headers.insert("Authorization", HeaderValue::from_str(auth.as_str())?);
        headers.insert("Content-Type", HeaderValue::from_str("application/json")?);

        let body = serde_json::to_string(&p)?;

        let client = Client::new();
        let mut res = client.post(&self.url).body(body).headers(headers).send()?;

        let mut response_body = String::new();
        res.read_to_string(&mut response_body)?;
        let json_object: Value = from_str(&response_body)?;
        let answer = json_object["choices"][0]["text"].as_str();

        match answer {
            Some(a) => Ok(String::from(a)),
            None => {
                pretty_print(&response_body, "json");
                Err(format!("JSON parse error").into())
            }
        }
    }
}
