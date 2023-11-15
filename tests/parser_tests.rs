use my_html_parser_kma::*;

#[test]
fn parser_test() -> anyhow::Result<()> {
    let doc = std::fs::read_to_string("tests/document_test.html").expect("could not read file");
    let pr = parse_html_file(&doc)?;

    let doc = std::fs::read_to_string("tests/empty_doc.html").expect("could not read file");
    let empty_doc = parse_html_file(&doc)?;
    assert!(pr != empty_doc);

    let doc = std::fs::read_to_string("tests/bad_doc.html").expect("could not read file");
    let bad_doc = parse_html_file(&doc);
    assert!(bad_doc.is_err());

    Ok(())
}
