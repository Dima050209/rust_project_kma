use clap::Parser;
use my_html_parser_kma::*;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum MyError {
    #[error("could not read file")]
    FileReadError(#[from] std::io::Error),
}

#[derive(Parser, Default, Debug)]
#[clap(
    author = "Dmytro Kharchenko",
    version,
    about = "This is a simple HTML parser"
)]
struct Arguments {
    /// Path to file
    path: Option<String>,
}

fn main() -> Result<(), MyError> {
    let args = Arguments::parse();
    if let Some(path) = args.path {
        let content = std::fs::read_to_string(path)?;

        let pr = parse_html_file(&content);
        println!("{:?}", &pr);
    }

    Ok(())
}
