use my_html_parser_kma::*;

use clap::Parser as ClapParser;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MyError {
    #[error("could not read file")]
    FileReadError(#[from] std::io::Error),
}

#[derive(ClapParser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}

pub fn main() -> Result<(), MyError> {
    let args = Cli::parse();
    let content = std::fs::read_to_string(args.path)?;

    let pr = parse_html_file(&content);
    println!("{:?}", &pr);
    Ok(())
}
