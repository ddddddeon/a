pub mod gpt;
pub mod util;

fn main() {
    let mut args: Vec<_> = std::env::args().collect();
    args.remove(0);

    if args.is_empty() {
        println!("no prompt provided");
        std::process::exit(1);
    }

    let lang = args[0].clone();
    let prompt = args.join(" ");
    let api_key = std::env::var("OPENAI_API_KEY")
        .expect("Please set the OPENAI_API_KEY environment variable");

    let client = gpt::GPTClient::new(api_key);
    let mut response = client
        .prompt(prompt)
        .expect("Could not make request to API");

    response.push('\n');
    if let Some(r) = response.strip_prefix("\n\n") {
        response = String::from(r);
    }

    #[cfg(feature = "clipboard")]
    {
        util::copy_to_clipboard(&response);
    }

    util::pretty_print(&response, &lang);
}
