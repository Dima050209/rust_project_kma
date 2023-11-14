use pest_derive::Parser;
use thiserror::Error;
use pest::error::Error as PestError;

#[derive(Parser)]
#[grammar = "./grammar.pest"]
pub struct Grammar;

#[derive(Error, Debug)]
enum MyError {
    #[error("Could not read file: {0}")]
    FileReadError(#[from] std::io::Error),

    #[error("Failed to parse HTML: {0}")]
    HtmlParseError(#[from] anyhow::Error),
}

#[derive(Debug)]
pub struct HTMLDocument {
    content: Tag
}

#[derive(Debug)]
pub struct Tag {
    name: String,
    attributes: Vec<Attribute>,
    content: Vec<Content>,
}

#[derive(Debug)]
pub struct Attribute {
    name: String,
    value: Option<String>,
}

#[derive(Debug)]
pub enum Content {
    ContentTag(Tag),
    ContentText(String),
}

impl Tag {
    pub fn new(name: String) -> Self {
        Tag {
            name,
            attributes: Vec::new(),
            content: Vec::new(),
        }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_attributes(&self) -> &Vec<Attribute> {
        &self.attributes
    }

    pub fn get_content(&self) -> &Vec<Content> {
        &self.content
    }
}

impl Attribute {
    pub fn new(name: String, value: Option<String>) -> Self {
        Attribute { name, value }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_value(&self) -> &Option<String> {
        &self.value
    }
}


use anyhow::anyhow;
use pest::Parser;
pub fn parse_html_file(file: &str) -> anyhow::Result<HTMLDocument> {
    let mut pair = Grammar::parse(Rule::document, &file)?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
      
    // for inner_pair in pair.clone().into_inner(). {
    //   println!("{}", inner_pair);
    //   println!(" ---------");
    // }
    let root_tag = parse_tag(pair)?;
    let html_doc = HTMLDocument {content:root_tag};

    use pest::iterators::Pair;
    fn parse_tag(pair: Pair<Rule>) -> Result<Tag, PestError<Rule>> {
        let mut tag = Tag::new(pair.as_str().to_string());

        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::tag_name => {
                    tag.name = inner_pair.as_str().to_string();
                }
                Rule::attribute => {
                    let mut attr_name = String::new();
                    let mut attr_value = None;

                    for attribute_pair in inner_pair.into_inner() {
                        match attribute_pair.as_rule() {
                            Rule::attribute_name => {
                                attr_name = attribute_pair.as_str().to_string();
                            }
                            Rule::attribute_value => {
                                attr_value = Some(attribute_pair.as_str().to_string());
                            }
                            _ => unreachable!(),
                        }
                    }

                    tag.attributes.push(Attribute::new(attr_name, attr_value));
                }
                Rule::text => {
                    tag.content
                        .push(Content::ContentText(inner_pair.as_str().to_string()));
                }
                Rule::tag => {
                    let inner_tag = parse_tag(inner_pair)?;
                    tag.content.push(Content::ContentTag(inner_tag));
                }
                Rule::EOI => {
                    return Ok(tag);
                }
                _ => {
                    println!("{:?}", inner_pair.as_rule());
                }
            }
        }
       
        Ok(tag)
    }
    // dbg!(&html_doc);
    Ok((html_doc))
}
