use super::util;
use reqwest::{
    blocking::Client,
    header::{HeaderMap, HeaderValue},
};
use serde::{Deserialize, Serialize};
use serde_json::{from_str, Value};
use std::error::Error;
use std::io::Read;
use std::time::Duration;

const OPENAI_API_URL: &str = "https://api.openai.com/v1/chat/completions";
const OPENAI_MODEL: &str = "gpt-3.5-turbo";
const MAX_TOKENS: u32 = 4000;
const TEMPERATURE: f32 = 0.3;

type BoxResult<T> = Result<T, Box<dyn Error>>;

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    role: String,
    content: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Prompt {
    model: String,
    messages: Vec<Message>,
    temperature: f32,
    max_tokens: u32,
}

pub(crate) struct GPTClient {
    api_key: String,
    url: String,
}

impl GPTClient {
    pub(crate) fn new(api_key: String) -> Self {
        GPTClient {
            api_key,
            url: String::from(OPENAI_API_URL),
        }
    }

    pub(crate) fn prompt(&self, prompt: String) -> BoxResult<String> {
        let prompt_length = prompt.len() as u32;
        if prompt_length >= MAX_TOKENS {
            return Err(format!(
                "Prompt cannot exceed length of {} characters",
                MAX_TOKENS - 1
            )
            .into());
        }

        let system =
            "You are a system that only generates code. Do not describe or contextualize the code. Do not apply any formatting or syntax highlighting. Do not wrap the code in a code block.";

        let messages = vec![
            Message {
                role: "system".to_string(),
                content: system.to_string(),
            },
            Message {
                role: "user".to_string(),
                content: prompt,
            },
        ];

        let p = Prompt {
            max_tokens: MAX_TOKENS - prompt_length,
            model: String::from(OPENAI_MODEL),
            temperature: TEMPERATURE,
            messages,
        };

        let mut auth = String::from("Bearer ");
        auth.push_str(&self.api_key);

        let mut headers = HeaderMap::new();
        headers.insert("Authorization", HeaderValue::from_str(auth.as_str())?);
        headers.insert("Content-Type", HeaderValue::from_str("application/json")?);

        let body = serde_json::to_string(&p)?;

        let client = Client::builder().timeout(Duration::from_secs(60)).build()?;
        let mut res = client.post(&self.url).body(body).headers(headers).send()?;

        let mut response_body = String::new();
        res.read_to_string(&mut response_body)?;
        let json_object: Value = from_str(&response_body)?;
        let answer = json_object["choices"][0]["message"]["content"].as_str();

        match answer {
            Some(a) => Ok(String::from(a)),
            None => {
                util::pretty_print(&response_body, "json");
                Err("JSON parse error".into())
            }
        }
    }
}
