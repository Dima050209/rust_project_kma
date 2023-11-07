use my_parser_kma_group_1::*;

use anyhow::anyhow;
use my_parser_kma_group_3_Kharchenko::*;
use pest::Parser;

pub fn main() -> anyhow::Result<()> {
    //println!("{:?}", list_parser::list("[1,1,2,3,5,8]"));

    let pair = Grammar::parse(Rule::text, "-273<.15")?;
        // .next()
        // .ok_or_else(|| anyhow!("no pair"))?;
    dbg!(pair);

    Ok(())
}
