use std::{env, process, fs};
use std::error::Error;
use std::path::PathBuf;

use astrolang::tokeniser::Tokeniser;

fn main() -> Result<(), Box<dyn Error>> {
    let args = env::args();
    let path = get_path(args);

    let file = fs::read_to_string(path)?;

    let tokens = Tokeniser::new(file).run();

    println!("{:?}", tokens);

    Ok(())
}

fn get_path(mut args: impl Iterator<Item = String>) -> PathBuf {
    args.next();
    if let Some(path) = args.next() {
        PathBuf::from(path)
    } else {
        eprintln!("Issue with path");
        process::exit(0)
    }
}
