use anyhow::anyhow;
use my_html_parser_kma::*;
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

    let pair = Grammar::parse(Rule::attribute, "a=\"b c d\"")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!(pair.as_str(), "a=\"b c d\"");

    let pair = Grammar::parse(Rule::attribute, "_=");
    assert!(pair.is_err());

    Ok(())
}
#[test]
fn tag_test() -> anyhow::Result<()> {
    let pair = Grammar::parse(Rule::tag, "<html></html>")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!(pair.as_str(), "<html></html>");
    assert_eq!(pair.as_span().start(), 0);
    assert_eq!(pair.as_span().end(), 13);

    let pair = Grammar::parse(
        Rule::tag,
        "<a link=\"anylink.com\" _a2=\"secondattr\">some text</a>",
    )?
    .next()
    .ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!(
        pair.as_str(),
        "<a link=\"anylink.com\" _a2=\"secondattr\">some text</a>"
    );

    let pair = Grammar::parse(
        Rule::tag,
        "<a link=\"anylink.com\" _a2=\"secondattr\">
    <div class = \"123\">
        text
    </div>
    another paragraph
</a>",
    )?
    .next()
    .ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!(
        pair.as_str(),
        "<a link=\"anylink.com\" _a2=\"secondattr\">
    <div class = \"123\">
        text
    </div>
    another paragraph
</a>"
    );

    let pair = Grammar::parse(Rule::tag, "<a><</a>");
    assert!(pair.is_err());

    let pair = Grammar::parse(Rule::tag, "<div class=\"a\"/>")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!(pair.as_str(), "<div class=\"a\"/>");
    assert_eq!(pair.as_span().start(), 0);
    assert_eq!(pair.as_span().end(), 16);

    Ok(())
}
#[test]
fn document_test() -> anyhow::Result<()> {
    let doc = std::fs::read_to_string("tests/document_test.html").expect("could not read file");

    let pair = Grammar::parse(Rule::document, &doc)?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;

    assert_eq!(pair.as_str(), "<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n    <meta charset=\"UTF-8\">\n    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n    <title>Complex HTML Example</title>\n</head>\n<body>\n\n    <header>\n        <h1>Welcome to My Website</h1>\n        <nav>\n            <ul>\n                <li><a href=\"#home\">Home</a></li>\n                <li><a href=\"#about\">About</a></li>\n                <li><a href=\"#services\">Services</a></li>\n                <li><a href=\"#contact\">Contact</a></li>\n            </ul>\n        </nav>\n    </header>\n\n    <section id=\"home\">\n        <h2>Home</h2>\n        <p>This is the home section of the website.</p>\n    </section>\n\n    <section id=\"home\">\n        <h2>Home</h2>\n        <p>This is the home section of the website.</p>\n    </section>\n\n    <section id=\"home\">\n        <h2>Home</h2>\n        <p>This is the home section of the website.</p>\n    </section>\n\n    <section id=\"home\">\n        <h2>Home</h2>\n        <p>This is the home section of the website.</p>\n    </section>\n\n    <section id=\"home\">\n        <h2>Home</h2>\n        <p>This is the home section of the website.</p>\n    </section>\n\n    <footer>\n        <p>&copy; 2023 My Website. All rights reserved.</p>\n    </footer>\n\n</body>\n</html>\n");

    Ok(())
}
