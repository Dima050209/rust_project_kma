use anyhow::anyhow;
use my_parser_kma_group_3_Kharchenko::*;
use pest::Parser;

#[test]
fn text_test() -> anyhow::Result<()> {
    let pair = Grammar::parse(Rule::text, "some text")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!(pair.as_str(), "some text");
    assert_eq!(pair.as_span().start(), 0);
    assert_eq!(pair.as_span().end(), 9);

    let pair = Grammar::parse(Rule::text, "<some text");
    assert!(pair.is_err());
   
    Ok(())
}

