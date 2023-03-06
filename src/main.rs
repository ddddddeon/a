use std::error::Error;
use std::io;

pub mod gpt;
pub mod util;

fn main() -> Result<(), Box<dyn Error>> {
    let mut args: Vec<_> = std::env::args().collect();
    args.remove(0);

    let lang;
    let mut prompt = String::new();

    if args.is_empty() {
        if let Err(e) = io::stdin().read_line(&mut prompt) {
            println!("Could not read from stdin: {e}");
            return Err(e.into());
        }

        let words: Vec<String> = prompt.split_whitespace().map(str::to_string).collect();
        if words.len() <= 1 {
            println!("Please supply a prompt!");
            std::process::exit(1);
        }

        lang = if words[0] != "a" {
            words[0].to_string()
        } else {
            words[1].to_string()
        };
    } else {
        lang = args[0].clone();
        prompt = args.join(" ");
    }

    let api_key = std::env::var("OPENAI_API_KEY")
        .expect("Please set the OPENAI_API_KEY environment variable");

    let client = gpt::GPTClient::new(api_key);
    let mut response = client
        .prompt(prompt)
        .expect("Could not make request to API");

    while response.starts_with('\n') {
        response.remove(0);
    }
    response.push('\n');

    #[cfg(feature = "clipboard")]
    {
        util::copy_to_clipboard(&response);
    }

    util::pretty_print(&response, &lang);

    Ok(())
}
