pub mod gpt;
pub mod util;

fn main() {
    let mut args: Vec<_> = std::env::args().collect();
    args.remove(0);

    if args.len() == 0 {
        println!("no prompt provided");
        std::process::exit(1);
    }

    let mut lang = args[0].clone();
    let prompt = args.join(" ");
    let api_key = std::env::var("OPENAI_API_KEY").unwrap();

    let client = gpt::GPTClient::new(api_key);
    let response = client.prompt(prompt).expect("Could not make request to API");

    let mut response = String::from(response.strip_prefix("\n\n").unwrap());
    response.push_str("\n");

    #[cfg(feature = "clipboard")]
    {
        util::copy_to_clipboard(&response);
    }

    util::pretty_print(&response, &mut lang);

}
