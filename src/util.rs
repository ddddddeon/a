use bat::PrettyPrinter;
use bat::Syntax;
use copypasta_ext::prelude::*;
use copypasta_ext::x11_fork::ClipboardContext;
use std::error::Error;

fn lang_exists(lang: &str, langs: &Vec<Syntax>) -> bool {
    for l in langs {
        if l.name.to_lowercase() == lang.to_lowercase() {
            return true;
        }
        for e in &l.file_extensions {
            if e == &lang.to_lowercase() {
                return true;
            }
        }
    }
    false
}

pub fn pretty_print(str: &str, lang: &str) {
    let mut lang = lang.to_owned();
    let mut pp = PrettyPrinter::new();

    let langs: Vec<_> = pp.syntaxes().collect();
    if !lang_exists(&lang, &langs) {
        lang = "txt".to_owned();
    }

    pp.input_from_bytes(str.as_bytes())
        .language(&lang)
        .print()
        .unwrap();
}

pub fn copy_to_clipboard(str: &str) -> Result<(), Box<dyn Error>> {
    let mut ctx = match ClipboardContext::new() {
        Ok(c) => c,
        Err(e) => {
            return Err(format!(
                "Cannot initialize clipboard context: {e}\nConsider recompiling with the \"clipboard\" feature disabled\n"
            )
            .into())
        }
    };

    match ctx.set_contents(str.to_owned()) {
        Ok(_) => return Ok(()),
        Err(e) => Err(format!(
            "Cannot initialize clipboard context: {e}\nConsider recompiling with the \"clipboard\" feature disabled\n"
        )
        .into()),
    }
}
