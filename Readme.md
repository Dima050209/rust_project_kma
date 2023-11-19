### HTML parser for KMA Rust course
![Rust Crab](https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcQ9jFG4XUD60ap87Arcx6wkBP3GJhYB5ChQyz_1u1onpVI3-4fpshRHDpuV4HE_T5n113E&usqp=CAU)

### Example
The following example will parse a HTML file acording to HTML5 standarts and return an instance of a HTMLDocument structure:
<pre>
<code>
use my_html_parser_kma::*;
...
let file_content = std::fs::read_to_string(path_to_file)?;
let parsed = parse_html_file(&file_content);
</code>
</pre>

### Description 

This HTML parser will parse a HTML file according to HTML5 standarts. The parser checks content correctness, such as all tags and text inside of a html tag. Apart from this, it also checks presense of `<!DOCTYPE html>` and `<html>...</html>`.

Grammar part was constructed using pest. <br/>

Grammar contains root element - document. document contains doctype and html tag with some content inside, or it can be empty document( SOI ~ EOI). 

There are several types of tags supported: `<tag></tag>`, `<tag />` or special tag `<meta>`, which comes without `/`. Besides, tag can contain some attributes `<tag attr="attr_value">` and some content, which is basically other tags and some text `<tag><inside_tag /> some text </tag>`. Attribute is described as a simple pair of name and value. Text can't contain `<`. 

The result of parsing is an instanse of a HTMLDocument struct. HTMLDocument contains field 'content', which is an Option<Tag>. Tag is another struct, which represents tag element and consists of  name: String, attributes: Vec<Attribute> and content: Vec<Content>. Attribute struct consists of name: String and value: Option<String>. Finally, Content is an enum and can be ContentTag(Tag) or ContentText(String). All structures have getters. 

parse_html_file() function calls inner function parse_tag(), which parses content of a file and returnes an instanse of Tag. parse_tag() recieves a pair, creates a new Tag, then matches a rule of the pair and 'fills' the Tag according to the rule. It's quite simple for some text or name rules and attributes. If the rule is tag, then we need to use recursion in order to recieve inner tags. Finally, parse_html_file() uses the generated instatnse of a Tag as a content for HTMLDocument and returns it.

### Usage 

In the main.rs implemented simple CLI. To run the program with test file passed to the parser execute:
<pre>
<code>
cargo run -- tests/test.html
</code>
</pre>
You can pass any file, simply change `tests/test.html` to your direction.
To view help and credits info execute:
<pre>
<code>
cargo run -- -h
</code>
</pre>
To run tests execute: 
<pre>
<code>
cargo test
</code>
</pre>