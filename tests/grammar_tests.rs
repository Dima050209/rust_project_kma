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

    // let pair = Grammar::parse(Rule::text, "")?
    //     .next()
    //     .ok_or_else(|| anyhow!("no pair"))?;
    // assert_eq!(pair.as_str(), "");
    // assert_eq!(pair.as_span().start(), 0);
    // assert_eq!(pair.as_span().end(), 0);

    let pair = Grammar::parse(Rule::text, "<some text");
    // println!("text {}", pair?.as_str());
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

    let pair = Grammar::parse(Rule::attribute_name, "_");
    assert!(pair.is_err());

    let pair = Grammar::parse(Rule::attribute_name, "");
    assert!(pair.is_err());

    Ok(())
}
#[test]
fn attribute_value_test() -> anyhow::Result<()> {
    let pair = Grammar::parse(Rule::attribute_value, "someval")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!(pair.as_str(), "someval");
    assert_eq!(pair.as_span().start(), 0);
    assert_eq!(pair.as_span().end(), 7);

    let pair = Grammar::parse(Rule::attribute_value, "\"someval");
    assert!(pair.is_err());

    let pair = Grammar::parse(Rule::attribute_value, "som\'eval");
    assert_eq!(pair?.as_str(), "som");

    Ok(())
}
#[test]
fn attribute_test() -> anyhow::Result<()> {
    let pair = Grammar::parse(Rule::attribute, "a=\"b\"")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!(pair.as_str(), "a=\"b\"");
    assert_eq!(pair.as_span().start(), 0);
    assert_eq!(pair.as_span().end(), 5);

    let pair = Grammar::parse(Rule::attribute, "_1 =   \"\"")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!(pair.as_str(), "_1 =   \"\"");

    let pair = Grammar::parse(Rule::attribute, "_=");
    assert!(pair.is_err());

    Ok(())
}
#[test]
fn opening_tag_test() -> anyhow::Result<()> {
    let pair = Grammar::parse(Rule::opening_tag, "<html>")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!(pair.as_str(), "<html>");
    assert_eq!(pair.as_span().start(), 0);
    assert_eq!(pair.as_span().end(), 6);

    let pair = Grammar::parse(Rule::opening_tag, "<style link=\"anylink.com\" >")?
    .next()
    .ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!(pair.as_str(), "<style link=\"anylink.com\" >");

    let pair = Grammar::parse(Rule::opening_tag, "<style link=\"anylink.com\" _a2=\"secondattr\"   >")?
    .next()
    .ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!(pair.as_str(), "<style link=\"anylink.com\" _a2=\"secondattr\"   >");
 

    let pair = Grammar::parse(Rule::opening_tag, "<<a>");
    assert!(pair.is_err());

    Ok(())
}
#[test]
fn closing_tag_test() -> anyhow::Result<()> {
    let pair = Grammar::parse(Rule::closing_tag, "</html>")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!(pair.as_str(), "</html>");
    assert_eq!(pair.as_span().start(), 0);
    assert_eq!(pair.as_span().end(), 7);

    let pair = Grammar::parse(Rule::closing_tag, "</   style  >");
    assert!(pair.is_err());

    
    let pair = Grammar::parse(Rule::closing_tag, "<</a>");
    assert!(pair.is_err());

    Ok(())
}
