use super::util;
use anyhow::{anyhow, Error};
use reqwest::{
    blocking::Client,
    header::{HeaderMap, HeaderValue},
};
use serde::{Deserialize, Serialize};
use serde_json::{from_str, Value};
use std::io::Read;
use std::time::Duration;

const MAX_TOKENS: u32 = 4000;
const TEMPERATURE: f32 = 0.3;

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    role: String,
    content: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Prompt<'a> {
    model: &'a str,
    messages: Vec<Message>,
    temperature: f32,
    max_tokens: u32,
}

pub(crate) struct LLMClient<'a> {
    model: &'a str,
    api_base_url: &'a str,
    api_key: &'a str,
}

impl<'a> LLMClient<'a> {
    pub(crate) fn new(model: &'a str, api_base_url: &'a str, api_key: &'a str) -> Self {
        LLMClient {
            model,
            api_key,
            api_base_url,
        }
    }

    pub(crate) fn prompt(&self, prompt: String) -> Result<String, Error> {
        let prompt_length = prompt.len() as u32;
        if prompt_length >= MAX_TOKENS {
            return Err(anyhow!(
                "Prompt cannot exceed length of {} characters",
                MAX_TOKENS - 1
            ));
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
            model: self.model,
            temperature: TEMPERATURE,
            messages,
        };

        let mut auth = String::from("Bearer ");
        auth.push_str(self.api_key);

        let mut headers = HeaderMap::new();
        headers.insert("Authorization", HeaderValue::from_str(auth.as_str())?);
        headers.insert("Content-Type", HeaderValue::from_str("application/json")?);

        let body = serde_json::to_string(&p)?;

        let client = Client::builder().timeout(Duration::from_secs(60)).build()?;
        let mut res = client
            .post(format!("{}/chat/completions", self.api_base_url))
            .body(body)
            .headers(headers)
            .send()?;

        let mut response_body = String::new();
        res.read_to_string(&mut response_body)?;
        let json_object: Value = from_str(&response_body)?;
        let answer = json_object["choices"][0]["message"]["content"].as_str();

        match answer {
            Some(a) => Ok(String::from(a)),
            None => {
                util::pretty_print(&response_body, "json");
                Err(anyhow!("JSON parse error"))
            }
        }
    }
}
