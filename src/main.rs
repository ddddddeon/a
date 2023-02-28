use bat::PrettyPrinter;

pub mod request;

fn main() {
    let mut args: Vec<_> = std::env::args().collect();
    args.remove(0);

    if args.len() == 0 {
        println!("no prompt provided");
        std::process::exit(1);
    }

    let lang = args[0].clone();
    let prompt = args.connect(" ");
    let mut response = request::make_request(String::from("https://api.openai.com/v1/completions"), prompt).unwrap();
    response = String::from(response.strip_prefix("\n\n").unwrap());
    response.push_str("\n");

    PrettyPrinter::new()
        .input_from_bytes(response.as_bytes())
        .language(&lang)
        .print()
        .unwrap();
}
