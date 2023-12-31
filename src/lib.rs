use pest::error::Error;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "./grammar.pest"]
pub struct Grammar;

#[derive(Debug, PartialEq)]
pub struct HTMLDocument {
    content: Option<Tag>,
}

#[derive(Debug, PartialEq)]
pub struct Tag {
    name: String,
    attributes: Vec<Attribute>,
    content: Vec<Content>,
}

#[derive(Debug, PartialEq)]
pub struct Attribute {
    name: String,
    value: Option<String>,
}

#[derive(Debug, PartialEq)]
pub enum Content {
    ContentTag(Tag),
    ContentText(String),
}
impl Default for HTMLDocument {
    fn default() -> Self {
        Self::new()
    }
}
impl HTMLDocument {
    pub fn new() -> Self {
        HTMLDocument { content: None }
    }
    pub fn get_content(&self) -> &Option<Tag> {
        &self.content
    }
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
    let pair = Grammar::parse(Rule::document, file)?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;

    let mut html_doc = HTMLDocument::new();
    if let Some(inner_pair) = pair.clone().into_inner().next() {
        if inner_pair.as_rule() != Rule::EOI {
            let root_tag = parse_tag(pair)?;
            html_doc.content = Some(root_tag);
        }
    }

    return Ok(html_doc);

    use pest::iterators::Pair;
    fn parse_tag(pair: Pair<Rule>) -> Result<Tag, Box<Error<Rule>>> {
        let mut tag = Tag::new("html".to_string());

        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::doctype => {
                    tag.name = "doctype".to_string();
                }
                Rule::tag_name => {
                    tag.name = inner_pair.as_str().to_string();
                }
                Rule::meta_name => {
                    tag.name = "meta".to_string();
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
                Rule::html_tag => {
                    let inner_tag = parse_tag(inner_pair)?;
                    tag.content.push(Content::ContentTag(inner_tag));
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
}
