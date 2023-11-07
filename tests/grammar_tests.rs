use pest::Parser;
use anyhow::anyhow;
use my_parser_kma_group_3_Kharchenko::*;

#[test]
fn field_test() -> anyhow::Result< () > {

    let pair = Grammar::parse(Rule::field, "-273.15")?.next().ok_or_else( || anyhow!( "no pair" ) )?;
    assert_eq!( pair.as_str(), "-273.15" );
    assert_eq!( pair.as_span().start(), 0 );
    assert_eq!( pair.as_span().end(), 7 );


    let pair = Grammar::parse(Rule::field, "x");
    assert!(pair.is_err());

    let pair = Grammar::parse(Rule::field, "");
    assert!(pair.is_err());

    Ok(())
}


#[test]
fn record_test() -> anyhow::Result< () > {
  
    let pair = Grammar::parse(Rule::record, "-273.15,99")?.next().ok_or_else( || anyhow!( "no pair" ) )?;
    assert_eq!( pair.as_str(), "-273.15,99" );
    assert_eq!( pair.as_span().start(), 0 );
    assert_eq!( pair.as_span().end(), 10 );
  
    let pair = Grammar::parse(Rule::record, "");
    assert!( pair.is_err() );
  
    Ok( () )
}

#[test]
fn file_test() -> anyhow::Result< () > {

    let pair = Grammar::parse(Rule::file, "-273.15,99\n")?.next().ok_or_else( || anyhow!( "no pair" ) )?;

    assert_eq!( pair.as_str(), "-273.15,99\n" );
    assert_eq!( pair.as_span().start(), 0 );
    assert_eq!( pair.as_span().end(), 11 );

    let pair = Grammar::parse(Rule::file, "-273.15,99");
    assert!( pair.is_err() );

    let pair = Grammar::parse(Rule::file, "abc\n");
    assert!( pair.is_err() );

    Ok( () )
}