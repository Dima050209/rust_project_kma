### My HTML parser

HTML parser for Rust course

![Rust Crab](https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcQ9jFG4XUD60ap87Arcx6wkBP3GJhYB5ChQyz_1u1onpVI3-4fpshRHDpuV4HE_T5n113E&usqp=CAU)

### Example

// will be soon

### Description 

This is a parser for HTML documents.<br />

Structure of an HTML document: <br />
HTML document consists of tags and text.  <br />
Tags consist of: opening tag <tag>, closing tag </tag> and some text between them. <br />
Inside of an opening tag can be some attributes <tag attr1 attr2> <br />
Standart html page contains tags html, head and body. As for now, it's planned to ignore this and make a parser parse a list of tags. Later the program might be modified to match this requirements. In the final version of the parser this moment will be specified. <br />

Grammar structure: <br />
opening_tag = `<`, tag_name, attribute, `>`; <br />
closing_tag = `</`, tag_name, `>`; <br />
tag_name = some text, that can't start with digit and can contain letters, digits and `_`; <br />
attribute = attribute_name, `="`, attribute_value, `"`; <br />
attribute_name = some text, that can't start with digit and can contain letters, digits, `_` and `-`; <br />
attributer_value = some text, that doesn't contain `'` or `"`; <br />
text = some text, that can't contain `<`; <br />
content = tag or text; _<br />_
tag = opening_tag, content, closing_tag; <br />
document = for now it is planned as a list of tags. Later this might be something like
<pre>
<code>
&lt;html&gt;
    &lt;head&gt;
        content
    &lt;/head&gt;
    &lt;body&gt;
        content
    &lt;/body&gt;
&lt;/html&gt;
</code>
</pre> 