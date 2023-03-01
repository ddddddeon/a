use bat::Syntax;
use bat::PrettyPrinter;
use copypasta_ext::prelude::*;
use copypasta_ext::x11_fork::ClipboardContext;

fn lang_exists(lang: &String, langs: &Vec<Syntax>) -> bool {
    for l in langs {
        if &l.name.to_lowercase() == &lang.to_lowercase() {
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

pub fn pretty_print(str: &String, lang: &String) {
    let mut lang = lang.clone();
    let mut pp = PrettyPrinter::new();

    let langs: Vec<_> = pp.syntaxes().collect();
    if !lang_exists(&lang, &langs) {
        lang = String::from("txt");
    }

    pp.input_from_bytes(str.as_bytes())
        .language(&lang)
        .print()
        .unwrap();
}

pub fn copy_to_clipboard(str: &String) {
    let mut ctx = ClipboardContext::new().unwrap();
    ctx.set_contents(str.clone()).unwrap();
}
