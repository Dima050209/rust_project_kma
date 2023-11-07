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
#[test]
fn tag_name_test() -> anyhow::Result<()> {
    let pair = Grammar::parse(Rule::tag_name, "sometag")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!(pair.as_str(), "sometag");
    assert_eq!(pair.as_span().start(), 0);
    assert_eq!(pair.as_span().end(), 7);

    let pair = Grammar::parse(Rule::tag_name, "1sometag");

    assert!(pair.is_err());

    let pair = Grammar::parse(Rule::tag_name, "_sometag")?
    .next()
    .ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!(pair.as_str(), "_sometag");
    assert_eq!(pair.as_span().start(), 0);
    assert_eq!(pair.as_span().end(), 8);
   
    Ok(())
}
#[test]
fn attribute_name_test() -> anyhow::Result<()> {
    let pair = Grammar::parse(Rule::attribute_name, "someattr")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!(pair.as_str(), "someattr");
    assert_eq!(pair.as_span().start(), 0);
    assert_eq!(pair.as_span().end(), 8);

    let pair = Grammar::parse(Rule::attribute_name, "_someattr")?
    .next()
    .ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!(pair.as_str(), "_someattr");
    assert_eq!(pair.as_span().start(), 0);
    assert_eq!(pair.as_span().end(), 9);

    let pair = Grammar::parse(Rule::attribute_name, "1someattr");
    assert!(pair.is_err());
   
    Ok(())
}
