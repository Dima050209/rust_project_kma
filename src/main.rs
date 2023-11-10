use my_parser_kma_group_1::*;

use anyhow::anyhow;
use my_parser_kma_group_3_Kharchenko::*;
use pest::Parser as PestParser;

use clap::Parser as ClapParser;

#[derive(ClapParser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}

pub fn main() -> anyhow::Result<()> {
    let args = Cli::parse();
    let content = std::fs::read_to_string(&args.path).expect("could not read file");

    let pair = Grammar::parse(Rule::document, &content)?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    println!("{}", pair);

    // for line in content.lines() {
    //     if line.contains(&args.pattern) {
    //         println!("{}", line);
    //     }
    // }

    Ok(())
}
