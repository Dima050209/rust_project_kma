### My Parser

Parser for Rust course

![Rust Crab](https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcQ9jFG4XUD60ap87Arcx6wkBP3GJhYB5ChQyz_1u1onpVI3-4fpshRHDpuV4HE_T5n113E&usqp=CAU)

### Example

// will be soon

### Description 

This is a parser for HTML documents. 

Structure of an HTML document:
HTML document consists of tags and text. 
Tags consist of: opening tag <tag>, closing tag </tag> and some text between them.
Inside of an opening tag can be some attributes <tag attr1 attr2>
Standart html page contains tags html, head and body. As for now, it's planned to ignore this and make a parser parse a list of tags. Later the program might be modified to match this requirements. In the final version of the parser this moment will be specified. 

Grammar structure:
opening_tag = '<', tag_name, attribute, '>';
closing_tag = '</', tag_name, '>';
attribute = attribute_name, '="', attribute_value, `"`;
attribute_name = some text. Can't contain special characters.
In this parser I'll simplify this rule, so that attribute_name starts with letter or '_' and another part can consist of ASCII_ALPHA, '_' and '-';
attributer_value = some text, that doesn't contain `'` or `"`;
text = some text, that can't contain '<';
content = tag or text,
tag = opening_tag, content, closing_tag;
document = for now it is planned as a list of tags. Later this might be something like '<html><head>...</head><body>...</body></html>'. 