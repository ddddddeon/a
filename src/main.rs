use a;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut args: Vec<_> = std::env::args().collect();
    let (prompt, lang) = a::gather_args(&mut args)?;
    let response = a::prompt(&prompt)?;

    #[cfg(feature = "clipboard")]
    {
        if let Err(e) = a::copy_to_clipboard(&response) {
            eprintln!("{}", e);
        }
    }

    a::pretty_print(&response, &lang);

    Ok(())
}
