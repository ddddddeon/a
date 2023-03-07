use a;
use std::error::Error;

#[test]
fn gather_args_full_sentence() -> Result<(), Box<dyn Error>> {
    let prompt = String::from("a python script to parse args");
    let mut args: Vec<String> = prompt.split_whitespace().map(str::to_string).collect();
    let (prompt, lang) = a::gather_args(&mut args)?;
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
fn gather_args_one_arg() -> Result<(), Box<dyn Error>> {
    let prompt = String::from("a");
    let mut args: Vec<String> = prompt.split_whitespace().map(str::to_string).collect();
    match a::gather_args(&mut args) {
        Ok(_) => Err("run() should return Err() if only one arg is present".into()),
        Err(_) => Ok(()),
    }
}

#[test]
fn gather_args_zero_args() -> Result<(), Box<dyn Error>> {
    let prompt = String::from("\n");
    let mut args: Vec<String> = prompt.split_whitespace().map(str::to_string).collect();
    match a::gather_args(&mut args) {
        Ok(_) => Err("run() should return Err() if no args are present".into()),
        Err(_) => Ok(()),
    }
}

#[test]
fn prompt_without_api_key() -> Result<(), Box<dyn Error>> {
    if let Ok(_) = std::env::var("OPENAI_API_KEY") {
        std::env::remove_var("OPENAI_API_KEY")
    }
    match a::prompt("a python script that fetches a url") {
        Ok(_) => Err("prompt() should return Err() if OPENAI_API_KEY is not set".into()),
        Err(_) => Ok(()),
    }
}
