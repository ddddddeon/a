use gpt::GPTClient;
use std::error::Error;
use std::io;

pub mod gpt;
pub mod util;

fn gather_args(args: &mut Vec<String>) -> Result<(String, String), Box<dyn Error>> {
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
        if words.len() < 1 {
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

pub fn run(args: &mut Vec<String>) -> Result<(), Box<dyn Error>> {
    let (prompt, lang) = gather_args(args)?;

    let api_key = std::env::var("OPENAI_API_KEY")
        .expect("Please set the OPENAI_API_KEY environment variable");

    let client = GPTClient::new(api_key);
    let mut response = client
        .prompt(prompt)
        .expect("Could not make request to API");

    while response.starts_with('\n') {
        response.remove(0);
    }
    response.push('\n');

    #[cfg(feature = "clipboard")]
    {
        if let Err(e) = util::copy_to_clipboard(&response) {
            println!("{}", e);
        }
    }

    util::pretty_print(&response, &lang);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prompt_full_sentence() -> Result<(), Box<dyn Error>> {
        let prompt = String::from("a python script to parse args");
        let mut args: Vec<String> = prompt.split_whitespace().map(str::to_string).collect();
        let (prompt, lang) = gather_args(&mut args)?;
        if lang != "python".to_string() || prompt != "python script to parse args".to_string() {
            return Err(format!(
                "Expected lang to be \"python\", got {}\n
                        Expected prompt to be \"python script to parse args\", got {}",
                lang, prompt
            )
            .into());
        }
        Ok(())
    }

    #[test]
    fn prompt_one_arg() -> Result<(), Box<dyn Error>> {
        let prompt = String::from("a");
        let mut args: Vec<String> = prompt.split_whitespace().map(str::to_string).collect();
        match gather_args(&mut args) {
            Ok(_) => Err("run() should return Err() if only one arg is present".into()),
            Err(_) => Ok(()),
        }
    }

    #[test]
    fn prompt_zero_args() -> Result<(), Box<dyn Error>> {
        let prompt = String::from("\n");
        let mut args: Vec<String> = prompt.split_whitespace().map(str::to_string).collect();
        match gather_args(&mut args) {
            Ok(_) => Err("run() should return Err() if no args are present".into()),
            Err(_) => Ok(()),
        }
    }
}
