use llm::LLMClient;
use std::error::Error;
use std::io;

pub mod llm;
pub mod util;

pub use util::*;

pub fn gather_args(args: &mut Vec<String>) -> Result<(String, String), Box<dyn Error>> {
    let lang;
    let mut prompt = String::new();

    if args.is_empty() {
        return Err("Received zero arguments".into());
    }

    args.remove(0);
    if args.is_empty() {
        if let Err(e) = io::stdin().read_line(&mut prompt) {
            println!("Could not read from stdin: {e}");
            return Err(e.into());
        }

        let words: Vec<String> = prompt.split_whitespace().map(str::to_string).collect();
        if words.is_empty() {
            return Err("Please supply a prompt".into());
        }

        if words[0] != "a" {
            lang = words[0].to_string();
        } else if words.len() >= 2 {
            lang = words[1].to_string();
        } else {
            return Err("Please supply a prompt".into());
        }
    } else {
        lang = args[0].clone();
        prompt = args.join(" ");
    }

    Ok((prompt, lang))
}

pub fn prompt(
    prompt: &str,
    model: &str,
    api_key: &str,
    base_url: &str,
) -> Result<String, Box<dyn Error>> {
    let client = LLMClient::new(model, base_url, api_key);
    let mut response = client.prompt(prompt.to_string())?;

    while response.starts_with('\n') {
        response.remove(0);
    }
    response.push('\n');

    Ok(response)
}
