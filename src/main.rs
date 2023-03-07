use a;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut args: Vec<_> = std::env::args().collect();
    a::run(&mut args)
}
