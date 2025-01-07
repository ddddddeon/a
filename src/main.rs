use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut args: Vec<_> = std::env::args().collect();
    let (prompt, lang) = a::gather_args(&mut args)?;

    let api_key = match std::env::var("DEEPSEEK_API_KEY") {
        Ok(k) => k,
        Err(_) => return Err("Please set the DEEPSEEK_API_KEY environment variable".into()),
    };

    let response = a::prompt(&prompt, "deepseek", &api_key, "https://api.deepseek.com")?;

    #[cfg(feature = "clipboard")]
    {
        if let Err(e) = a::copy_to_clipboard(&response) {
            eprintln!("{}", e);
        }
    }

    a::pretty_print(&response, &lang);

    Ok(())
}
